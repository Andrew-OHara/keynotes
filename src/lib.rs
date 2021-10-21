use std::{fs, fs::{OpenOptions, File}, io, io::{Write, prelude::*}, collections::HashMap, path::PathBuf, error::Error};

mod section;

use aoutils::*;
use section::*;

pub struct KeynoteFile {
    pub filepath : PathBuf,
    sections : HashMap<String, Section> 
}

impl KeynoteFile {
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

    fn open_keynote_file(filepath : &PathBuf) -> Result<File, Box<dyn Error>>{
        // obtain the path to the path_buf parent folder
        let mut folder = filepath.clone();
        folder.pop();        
  
        // if folder doesn't exist, create it
        if !folder.exists() {
            fs::create_dir(folder)?;
        }   

        // open file as append and read, and return
        let file = OpenOptions::new().append(true).read(true).create(true).open(filepath.as_path())?;     
     
        Ok(file)       
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
    
    fn add_section_to_data_structure(&mut self, section_name: &str) {
        self.sections.insert(section_name.to_string(), Section::new(section_name));
    }

    pub fn load_data(&mut self) -> Result<(), Box<dyn Error>> {
        let file = KeynoteFile::open_keynote_file(&self.filepath)?;

        // read lines one at a time, checking for sections and reading them into the data structure
        let reader = io::BufReader::new(file);         
        let mut curr_section_name = String::new();
        for line in reader.lines() {
            if let Err(_) = line { return Err("error: unable to load data".into()) }

            let line = line.unwrap();            
            if let Some(section_name) = Section::get_section_name_from_string(&line) {        // handle sections           
                self.add_section_to_data_structure(section_name);
                curr_section_name = section_name.to_string();
            }
            else if let Some((k, v)) = KeynoteFile::get_entry_from_string(&line) {          // handle entries
                let section = self.get_section(&curr_section_name);
                match section {
                    Some(section) => section.add_entry(k, v), 
                    None => { 
                        return Err("error: file format corrupted".into());                            
                    }
                };
            }                        
        }
        Ok(())
    }   

    pub fn add_key<'a>(mut self, section_to_add_to: &str, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        if self.contains_key(key) {
            return Err(format!("key: {} already exists. no key added.", key).into());            
        }      
        
        // insert into data structure
        if let Some(section) = self.get_section(section_to_add_to){
            section.add_entry(key, value);
        }
        else {
            return Err(format!("cannot add to '{}'. that section does not exist", section_to_add_to).into());
            
        }

        // write the new key to the file        
        let file = KeynoteFile::open_keynote_file(&self.filepath)?;
        let reader = io::BufReader::new(file);
            
        let tmp_filepath = self.filepath.with_file_name("_kntemp.dat");
        let mut tmp_file = KeynoteFile::open_keynote_file(&tmp_filepath)?;      

        for line in reader.lines() {
            let line = line.unwrap();                
            let line = ensure_newline(&line);             

            tmp_file.write_all(line.as_bytes())?;               
               
            if let Some(section_name) = Section::get_section_name_from_string(&line) {
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
       
        Ok(())
    }

    pub fn remove_key(mut self, key: &str) -> Result<(), Box<dyn Error>>{
        if !self.contains_key(key) {
            return Err(format!("key: '{}' does not exist. nothing removed.", key).into());            
        }     
              
        let file = KeynoteFile::open_keynote_file(&self.filepath)?;
        let reader = io::BufReader::new(file);
            
        let tmp_filepath = self.filepath.with_file_name("_kntemp.dat");
        let mut tmp_file = KeynoteFile::open_keynote_file(&tmp_filepath)?;    

        let mut curr_section_name = String::new();
            
        for line in reader.lines() {
            let line = line.unwrap();                
            let line = ensure_newline(&line);

            if let Some((k, _)) = KeynoteFile::get_entry_from_string(&line) {
                // line is an entry, only write if it's not the key we're removing
                if k != key {
                    tmp_file.write_all(line.as_bytes())?;                             
                } 
                else {
                    // it is the key we're removing... remove from data structure
                    if let Some(section) = self.get_section(&curr_section_name) {
                        section.data.remove(key);                            
                    }
                }
            } else {    // line is a section, write for sure
                let curr_section_opt = Section::get_section_name_from_string(&line);
                match curr_section_opt {
                    Some(v) => curr_section_name = v.to_string(),
                    None => {                           
                        return Err("error: file corrupted".into());                            
                    }
                };

                tmp_file.write_all(line.as_bytes())?;
            };                                
        }
            
        // now we need to delete the old file and rename the temp one
        fs::remove_file(self.filepath.clone())?;
        fs::rename(tmp_filepath, self.filepath.clone())?;
        
        Ok(())
    }

    pub fn remove_section(&mut self, section_to_remove: &str) -> Result<(), Box<dyn Error>> {    
        let file = KeynoteFile::open_keynote_file(&self.filepath)?;
        let reader = io::BufReader::new(file);
            
        let tmp_filepath = self.filepath.with_file_name("_kntemp.dat");
        let mut tmp_file = KeynoteFile::open_keynote_file(&tmp_filepath)?;

        let mut writing = true;
        for line in reader.lines() {
            let line = line.unwrap();               
            let line = ensure_newline(&line);

            let section_name = Section::get_section_name_from_string(&line);
            if let Some(section_name) = section_name {
                if section_name == section_to_remove {
                    writing = false;    // found the section to remove, stop copying
                    continue;
                }
            }

            if writing || (!writing &&  Section::get_section_name_from_string(&line).is_some()){
                // !writing in here means we just found a new section after skipping the last, start writing again
                if !writing { writing = true; } 
                tmp_file.write_all(line.as_bytes())?;                    
            }
        }

        // now we need to delete the old file and rename the temp one
        fs::remove_file(self.filepath.clone())?;
        fs::rename(tmp_filepath, self.filepath.clone())?;        
        
        // remove from data structure
        self.sections.remove(section_to_remove);

        Ok(())
    }
    
    pub fn get_sections(&self) -> &HashMap<String, Section> {
        return &self.sections;
    }

    pub fn add_section(&mut self, section_name : &str) -> Result<(), Box<dyn Error>> {       
        if !is_alphabetic(section_name) {
            return Err(format!("'{}' is not a valid section name", section_name).into());            
        }   

        if let Some(_) = self.get_section(section_name) {
            return Err("section already exists".into());            
        }        
        
        self.add_section_to_data_structure(section_name);
    
        let section_header_str = Section::build_section_string(section_name);
        let mut file = KeynoteFile::open_keynote_file(&self.filepath)?;

        // write the section header
        file.write(section_header_str.as_bytes())?;        

        Ok(())
    }  

    pub fn get_value_from_key(&mut self, key: &str) -> Option<&str>{           
        for (_, section) in &self.sections {
            if let Some(value) = section.data.get(key) {
                return Some(value)
            }
        } 
        None
    }
    
    pub fn contains_key(&mut self, key: &str) -> bool {           
        for (_, section) in &self.sections {
            if section.data.contains_key(key) {
                return true;
            }
        }
        return false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_path_to_test_file() -> PathBuf {
        let mut data_filepath = match home::home_dir() {
            Some(path_buffer) => path_buffer,
            None => panic!("error: unable to find home directory") 
        };            
        
        data_filepath.push(".keynotes/kntest.dat");
        data_filepath
    }
    
    fn get_path_to_file_in_nonexistant_folder() -> PathBuf {
        let mut data_filepath = match home::home_dir() {
            Some(path_buffer) => path_buffer,
            None => panic!("error: unable to find home directory") 
        };            
        
        data_filepath.push(".keynotes/fakefolder/onemore/kntest.dat");

        data_filepath
    }

    #[test]
    fn open_keynote_file_success() {
        // create test file
        let path_to_test_file = get_path_to_test_file();        

        let result = KeynoteFile::open_keynote_file(&path_to_test_file);

        assert!(result.is_ok());

        // delete test file
        fs::remove_file(path_to_test_file).expect("error: unable to remove test file");
    }

    #[test]
    #[should_panic]
    fn open_keynote_file_nonexistant_location() {
        // create test file
        let path_to_test_file = get_path_to_file_in_nonexistant_folder();        

        match KeynoteFile::open_keynote_file(&path_to_test_file) {
            Ok(_) => {  // delete test file
                fs::remove_file(path_to_test_file).expect("error: unable to remove test file");
            },
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };       
    }

    #[test]
    fn get_section_success() {
        // setup
        let mut test_file = KeynoteFile {
            filepath : PathBuf::new(), // not used for this test, can leave uninitialized
            sections : HashMap::new() 
        };
        test_file.add_section_to_data_structure("test_section");

        // execute
        let section = test_file.get_section("test_section");
        
        // assert
        assert!(section.is_some());
        assert_eq!(section.unwrap().name, "test_section");        
    }

    #[test]
    fn get_section_not_found() {
        // setup
        let mut test_file = KeynoteFile {
            filepath : PathBuf::new(), // not used for this test, can leave uninitialized
            sections : HashMap::new() 
        };

        // execute
        let result = test_file.get_section("nonexistant_section");
        
        //assert
        assert!(result.is_none());
    }
}