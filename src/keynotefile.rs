use std::{fs, fs::{OpenOptions, File}, io, io::{Write, prelude::*}, collections::HashMap, path::PathBuf, error::Error};
use section::*;
use kn_utils::*;

mod kn_utils;
mod section;

pub struct KeynoteFile {
    pub filepath : PathBuf,
    pub sections : HashMap<String, Section> 
}

impl KeynoteFile {
    fn open_keynote_file(filepath : &PathBuf) -> Option<File>{
        // obtain the path to the path_buf parent folder
        let mut folder = filepath.clone();
        folder.pop();        
  
        // if folder doesn't exist, create it
        if !folder.exists() {
            if let Err(_) = fs::create_dir(folder) {
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

    fn build_entry_string(key: &str, value: &str) -> String {
        let mut entry: String = String::from("\t<");
        entry.push_str(key);
        entry.push('>');
        entry.push_str(value);
        entry.push_str("<~>");
        entry.push('\n');

        entry
    } 

    fn get_entry_from_string(line: &str) -> Option<(&str, &str)>{
        if line.starts_with("\t") {
            if let Some(i) = line.find(">") {
                let k = &line[2..i];
                let v = &line[i+1..line.len()-3];
                return Some((k, v));
            }
        }
        None
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
        let mut curr_section_name = String::new();
        for line in reader.lines() {
            if let Ok(lstr) = line {
                if let Some(section_name) = Section::get_section_name_from_string(&lstr) {
                    let st_name = String::from(section_name);
                    self.sections.insert(st_name.clone(), Section::new(st_name.clone()));
                    curr_section_name = st_name;
                }
                else if let Some((k, v)) = KeynoteFile::get_entry_from_string(&lstr) {
                    let section = self.get_section(&curr_section_name);
                    match section {
                        Some(section) => section.add_entry(k, v), 
                        None => { 
                            eprintln!("error: file format corrupted");
                            return 
                        }
                    };
                }
            }            
        }
    }         

    pub fn new<'a>() -> Result<KeynoteFile, &'a str> {
        // build path to keynotes.dat file        
        let mut data_filepath = match home::home_dir() {
            Some(path_buffer) => path_buffer,
            None => {            
                return Err("error: unable to find home directory") 
            }
        };        
        
        data_filepath.push(".keynotes/keynotes.dat");
        
        Ok(KeynoteFile {
            sections: HashMap::new(),
            filepath: data_filepath 
        })
    }

    pub fn add_key<'a>(mut self, section_to_add_to: &str, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        if self.contains_key(key) {
            println!("key: {} already exists. no key added", key);
            return Ok(())
        }

        self.load_data();
        
        // insert into data structure
        if let Some(section) = self.get_section(section_to_add_to){
            section.add_entry(key, value);
        }
        else {
            println!("cannot add to '{}'. that section does not exist", section_to_add_to);
            return Ok(());
        }

        // * write the new key to the file
        // ** open file and read all lines
        if let Some(file) = KeynoteFile::open_keynote_file(&self.filepath) {
            let reader = io::BufReader::new(file);
            
            let tmp_filepath = self.filepath.with_file_name("_kntemp.dat");
            let tmp_file = KeynoteFile::open_keynote_file(&tmp_filepath);
            if let None = tmp_file {
                return Err("error: failed to create temporary file. no key added".into());               
            }
            
            let mut tmp_file = tmp_file.unwrap();

            for line in reader.lines() {
                let line = line.unwrap();                
                let line = ensure_newline(&line);             

                tmp_file.write_all(line.as_bytes())?;               
               
                if let Some(section_name) = Section::get_section_name_from_string(&line.trim_end()) {
                    if section_name == section_to_add_to {
                        // add new entry to file
                        let entry = KeynoteFile::build_entry_string(key, value);

                        tmp_file.write_all(entry.as_bytes())?;
                            
                    }
                }
            }

            // now we need to delete the old file and rename the temp one
            fs::remove_file(self.filepath.clone())?;
            fs::rename(tmp_filepath, self.filepath.clone())?;            

        } else {
            return Err("error: unable to open file, no key written".into());            
        }
        Ok(())
    }

    pub fn list_keys(mut self) {
        self.load_data();
        for (_, section) in self.sections {   
            if section.data.len() != 0 {
                println!("{}", section.name)
            }    

            for (k, _) in section.data {
                println!("\t{}", k);
            }
        }
    }

    pub fn remove_key(mut self, key: &str) -> Result<(), &str>{
        if !self.contains_key(key) {
            return Err("key: '{}' does not exist. nothing removed");            
        }

        self.load_data();

        // * write the new key to the file
        // ** open file and read all lines
        if let Some(file) = KeynoteFile::open_keynote_file(&self.filepath) {
            let reader = io::BufReader::new(file);
            
            let tmp_filepath = self.filepath.with_file_name("_kntemp.dat");
            let tmp_file = KeynoteFile::open_keynote_file(&tmp_filepath);
            if let None = tmp_file {
                return Err("failed to create temporary file. no key added");                
            }
            
            let mut tmp_file = tmp_file.unwrap();
            let mut curr_section_name = String::new();
            
            for line in reader.lines() {
                let line = line.unwrap();                
                let line = ensure_newline(&line);

                if let Some((k, _)) = KeynoteFile::get_entry_from_string(&line) {
                    // line is an entry, only write if it's not the key we're removing
                    if k != key {
                        if let Err(_) = tmp_file.write_all(line.as_bytes()) {
                            // TODO: delete the temporary file here?
                            return Err("error: failed to write to temporary file. no key added");   
                        } 
                    } 
                    else {
                        // remove from data structure
                        if let Some(section) = self.get_section(&curr_section_name) {
                            section.data.remove(key);                            
                        }
                    }
                } else {    // line is a section, write for sure
                    let curr_section_opt = Section::get_section_name_from_string(&line);
                    match curr_section_opt {
                        Some(v) => curr_section_name = v.to_string(),
                        None => {                            
                            return Err("error: file corrupted");                            
                        }
                    };

                    if let Err(_) = tmp_file.write_all(line.as_bytes()) {
                        // TODO: delete the temporary file here?
                        return Err("error: failed to write to temporary file. no key added");
                    }
                };                                
            }
            
            // now we need to delete the old file and rename the temp one
            if let Err(_) = fs::remove_file(self.filepath.clone()) {
                return Err("error: could not delete original file");                
            }

            if let Err(_) = fs::rename(tmp_filepath, self.filepath.clone()) {
                // TODO: delete the temporary file here?
                return Err("error: could not rename temp file file");                 
            }
        }
        Ok(())
    }

    pub fn remove_section(&mut self, section_to_remove: &str) {
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

            let mut writing = true;

            for line in reader.lines() {
                let line = line.unwrap();                
                let line = ensure_newline(&line);

                let section_name = Section::get_section_name_from_string(&line.trim_end());
                if let Some(section_name) = section_name {
                    if section_name == section_to_remove {
                        writing = false;    // found the section to remove, stop copying
                        continue;
                    }
                }

                if writing || (!writing &&  Section::get_section_name_from_string(&line).is_some()){
                    // !writing in here means we just found a new section after skipping the last, start writing again
                    if !writing { writing = true; } 
                    if let Err(_) = tmp_file.write_all(line.as_bytes()) {
                        eprintln!("error: failed to write to temporary file. no key added");
                        // TODO: delete the temporary file here?
                        return;
                    }
                }
            }

            // now we need to delete the old file and rename the temp one

            if let Err(_) = fs::remove_file(self.filepath.clone()) {
                eprintln!("error: could not delete original file");
                return;
            }

            if let Err(_) = fs::rename(tmp_filepath, self.filepath.clone()) {
                // TODO: delete the temporary file here?
                eprintln!("error: could not rename temp file file");
                return;  
            }
        }
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
            println!("'{}' is not a valid section name", section_name);
            return
        }        

        // refresh the data structure
        self.load_data();
        if let Some(_) = self.get_section(section_name) {
            println!("section named '{}' already exists", section_name);
            return
        }
        // Add Section object to data structure
        self.sections.insert(String::from(section_name), Section::new(String::from(section_name)));

        // Add string representation of Section to file
        // build section header string 
        let section_header_str = Section::build_section_string(section_name);        
        
        // open the file 
        let file_opt = KeynoteFile::open_keynote_file(&self.filepath);            
        if let None = file_opt { return }
        let mut file = file_opt.unwrap();            

        // write the section header
        if let Err(_) = file.write(section_header_str.as_bytes()) {
            println!("error: unable to write to keynotes data file");
            return
        }
        
        println!("'{}' added", section_name);
    }  

    // TODO: good candidate for Result rather than option
    pub fn get_value_from_key(&mut self, key: &str) -> Option<&str>{
        self.load_data();    
        for (_, section) in &self.sections {
            if let Some(value) = section.data.get(key) {
                return Some(value)
            }
        } 
        None
    }
    
    pub fn contains_key(&mut self, key: &str) -> bool {    
        self.load_data();    
        for (_, section) in &self.sections {
            if section.data.contains_key(key) {
                return true;
            }
        }
        return false
    }
}