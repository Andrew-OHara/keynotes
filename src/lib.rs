use std::{fs::OpenOptions, io, io::{Write, prelude::*}, collections::HashMap};

fn ensure_newline(line: &str) -> String {
    if let Some(c) = line.chars().last() {
        if c != '\n' {
            return String::from(format!("{}{}", line, '\n'));
        }
    }

    return line.to_string();
}

pub struct Section {
    _name : String,
    data : HashMap<String, String>
}

impl Section {
    fn new(_name : String) -> Section {
        Section {
            _name,
            data : HashMap::new()
        }
    }
}

pub struct KeynoteFile {
    pub filepath : std::path::PathBuf,
    pub sections : HashMap<String, Section>
}

impl KeynoteFile {
    fn open_keynote_file(filepath : &std::path::PathBuf) -> Option<std::fs::File>{
        // obtain the path to the path_buf parent folder
        let mut folder = filepath.clone();
        folder.pop();        
  
        // if folder doesn't exist, create it
        if !folder.exists() {
            if let Err(_) = std::fs::create_dir(folder) {
                println!("error: unable to create path to keynote data file");
                return None
            }
        }

        // open file and extract from Result or return
        let file = OpenOptions::new().append(true).read(true).create(true).open(filepath.as_path());
        if let Err(_) = file {
            println!("error: unable to open keynotes data file");
            return None
        }        
        let file = file.unwrap();

        Some(file)
    }

    fn build_section_string(section_name: &str) -> String {
        let mut header_string = String::new();
        header_string.push('<');
        header_string.push_str(section_name);
        header_string.push_str(">\n");

        header_string
    } 

    fn build_entry_string(key: &str, value: &str) -> String {
        let mut entry: String = String::from("\t<");
        entry.push_str(key);
        entry.push('>');
        entry.push_str(value);
        entry.push_str("<~>");
        entry.push('\n');

        entry
    } 

    fn get_section_name_from_section_header(line : &str) -> Option<&str> {
        if !line.contains("<") || !line.contains(">") || line.contains("\t") {  // not a valid section name
            return None
        }

        Some(&line[1..line.len()-1])
    }    

    fn get_section(&mut self, section_name : &str) -> Option<&mut Section> {
        match self.sections.get_mut(section_name) {
            Some(section) => Some(section),
            None => None
        }
    }
    
    fn load_data(&mut self) {
        // open the file 
        let file_opt = KeynoteFile::open_keynote_file(&self.filepath);            
        if let None = file_opt { return }
        let file = file_opt.unwrap();

        // read lines one at a time, checking for sections and reading them into the data structure
        let reader = io::BufReader::new(file);         
        for line in reader.lines() {
            if let Ok(lstr) = line {
                if let Some(name) = KeynoteFile::get_section_name_from_section_header(&lstr) {
                    let st_name = String::from(name);
                    self.sections.insert(st_name.clone(), Section::new(st_name));
                }
            }            
        }
    }         

    pub fn add_key(mut self, section_to_add_to: &str, key: &str, value: &str) {
        self.load_data();
        
        // insert into data structure
        if let Some(section) = self.get_section(section_to_add_to){
            section.data.insert(String::from(key), String::from(value));
        }
        else {
            println!("cannot add to {}. that section does not exist", section_to_add_to);
            return;
        }

        // * write the new key to the file
        // ** open file and read all lines
        if let Some(file) = KeynoteFile::open_keynote_file(&self.filepath) {
            let reader = io::BufReader::new(file);
            
            let tmp_filepath = self.filepath.with_file_name("_kntemp.dat");
            let tmp_file = KeynoteFile::open_keynote_file(&tmp_filepath);
            if let None = tmp_file {
                eprintln!("failed to create temporary file. no key added");
                return;
            }
            
            let mut tmp_file = tmp_file.unwrap();

            for line in reader.lines() {
                let line = line.unwrap();                
                let line = ensure_newline(&line);             

                if let Err(_) = tmp_file.write_all(line.as_bytes()) {
                    println!("error: failed to write to temporary file. no key added");
                    // TODO: delete the temporary file here?
                    return;
                }
               
                if let Some(section_name) = KeynoteFile::get_section_name_from_section_header(&line.trim_end()) {
                    if section_name == section_to_add_to {
                        // add new entry
                        let entry = KeynoteFile::build_entry_string(key, value);

                        if let Err(_) = tmp_file.write_all(entry.as_bytes()) {
                            println!("error: failed to write to temporary file. no key added");
                            // TODO: delete the temporary file here?
                            return;
                        }
                    }
                }
            }

            // now we need to delete the old file and rename the temp one

            if let Err(_) = std::fs::remove_file(self.filepath.clone()) {
                println!("error: could not delete original file");
                return;
            }

            if let Err(_) =std::fs::rename(tmp_filepath, self.filepath.clone()) {
                // TODO: delete the temporary file here?
                println!("error: could not rename temp file file");
                return;  
            }

        } else {
            println!("unable to open file, no key written");
            return;
        }
    }

    pub fn remove_section(&mut self, section_to_remove: &str) {
        println!("remove_section not implemented yet! {}", section_to_remove);
    }
    
    pub fn list_sections(mut self) {
        self.load_data();
        if self.sections.len() == 0 {
            println!("keynotes data file is empty");
            return
        }
        for section in self.sections {
            println!("{}", section.0);
        }
    }

    pub fn add_section(&mut self, section_name : &str) {       
        if !is_alphabetic(section_name) {
            println!("{} is not a valid section name", section_name);
            return
        }        

        // refresh the data structure
        self.load_data();
        if let Some(_) = self.get_section(section_name) {
            println!("section named {} already exists", section_name);
            return
        }
        // Add Section object to data structure
        self.sections.insert(String::from(section_name), Section::new(String::from(section_name)));

        // Add string representation of Section to file
        // build section header string 
        let section_header_str = KeynoteFile::build_section_string(section_name);        
        
        // open the file 
        let file_opt = KeynoteFile::open_keynote_file(&self.filepath);            
        if let None = file_opt { return }
        let mut file = file_opt.unwrap();            

        // write the section header
        if let Err(_) = file.write(section_header_str.as_bytes()) {
            println!("error: unable to write to keynotes data file");
            return
        }       
    }        
}


pub fn is_alphabetic(s : &str) -> bool {
    for c in s.chars() {
        if !c.is_alphabetic() {            
            return false
        }
    }

    true
}

#[cfg(test)]
 mod tests {
     mod is_alphabetic_tests {
        use super::super::*;

        #[test]
        fn is_alphabetic_true() {
            let alphabetic_str = "alphabetic";
            let alphabetic: bool = is_alphabetic(alphabetic_str);
            assert_eq!(alphabetic, true);
        }

        #[test]
        fn is_alphabetic_with_numbers() {
            let alphabetic_str = "1alphabetic";
            let alphabetic: bool = is_alphabetic(alphabetic_str);
            assert_eq!(alphabetic, false);
        }

        #[test]
        fn is_alphabetic_with_space() {
            let alphabetic_str = "alpha betic";
            let alphabetic: bool = is_alphabetic(alphabetic_str);
            assert_eq!(alphabetic, false);
        }

        #[test]
        fn is_alphabetic_with_special_chars() {
            let alphabetic_str = "alpha!betic?";
            let alphabetic: bool = is_alphabetic(alphabetic_str);
            assert_eq!(alphabetic, false);
        }

        #[test]
        fn is_alphabetic_with_tab_and_newline() {
            let alphabetic_str = "\talphabetic\n";
            let alphabetic: bool = is_alphabetic(alphabetic_str);
            assert_eq!(alphabetic, false);
        }
    }
 }