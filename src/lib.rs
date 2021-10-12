use std::{fs::OpenOptions, io, io::{Write, prelude::*}, collections::HashMap};

pub struct Section {
    name : String,
    data : HashMap<String, String>
}

impl Section {
    fn new(name : String) -> Section {
        Section {
            name,
            data : HashMap::new()
        }
    }
}

pub struct KeynoteFile {
    pub path_buf : std::path::PathBuf,
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

    fn build_section_header_string(section_name: &str) -> String {
        let mut header_string = String::new();
        header_string.push('<');
        header_string.push_str(section_name);
        header_string.push_str(">\n");

        header_string
    } 

    fn get_section_name_from_section_header(line : &str) -> Option<&str> {
        if !line.contains("<") || !line.contains(">") || line.contains("\t") {  // not a valid section name
            return None
        }
        Some(&line[1..line.len()-1])
    }
    fn load_data(&mut self) {
        // open the file 
        let file_opt = KeynoteFile::open_keynote_file(&self.path_buf);            
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
        let section_header_str = KeynoteFile::build_section_header_string(section_name);        
        
        // open the file 
        let file_opt = KeynoteFile::open_keynote_file(&self.path_buf);            
        if let None = file_opt { return }
        let mut file = file_opt.unwrap();            

        // write the section header
        if let Err(_) = file.write(section_header_str.as_bytes()) {
            println!("error: unable to write to keynotes data file");
            return
        }       
    }    

    fn get_section(&self, section_name : &str) -> Option<&Section> {
        match self.sections.get(section_name) {
            Some(section) => Some(section),
            None => None
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