//! Keydata is a lib for storing data. 
//!     Data is stored as key-value pairs and organized into named sections
//!     Data is stored in files created by the lib, in a hidden folder in users home directory
//! 
//! # Example
//! ```
//!use std::{error::Error, fs};
//!
//!fn main() -> Result<(), Box<dyn Error>> {
//!    let mut file = keydata::KeynoteFile::new("kntest.dat")?;   
//!    file.load_data()?;
//!    file.add_section("sectionname")?;
//!    file.add_entry("sectionname", "somekey", "somevalue")?;
//!     
//!    for (_, section) in file.get_sections() {   
//!        if section.data.len() != 0 {
//!           println!("{}", section.name)
//!        }    
//!      
//!        for (k, _) in section.data.iter() {
//!            println!("\t{}", k);
//!        }
//!    }    
//
//!    fs::remove_file(file.filepath);  // remove the test file
//!     
//!    Ok(()) 
//!}
//! ```

use std::{fs, fs::{OpenOptions, File}, io, io::{Write, prelude::*}, collections::HashMap, path::PathBuf, error::Error};

mod section;

use aoutils::*;
pub use section::*;

/// A data structure to represent the keynotes data file
pub struct KeynoteFile {
    /// path to the file as a PathBuf
    pub filepath : PathBuf,
    /// hashmap to store Section instances
    sections : HashMap<String, Section> 
}

impl KeynoteFile {
    /// Creates a new KeynoteFile
    ///
    /// # Arguments
    ///
    /// * `filename` - name of file to create in keynotes folder  
    ///
    /// # Examples    ///
    /// ```
    /// use keydata::*;
    /// let kn_file = KeynoteFile::new("kntest.dat").unwrap();
    /// 
    /// assert!(kn_file.filepath.ends_with("kntest.dat"));
    ///  
    /// ```
    pub fn new<'a>(filename: &str) -> Result<KeynoteFile, &'a str> {
        // build path to keynotes.dat file        
        let mut data_filepath = match home::home_dir() {
            Some(path_buffer) => path_buffer,
            None => {            
                return Err("error: unable to find home directory") 
            }
        };        
        
        data_filepath.push(format!(".keynotes/{}", filename));
        
        Ok(KeynoteFile {
            sections: HashMap::new(),
            filepath: data_filepath 
        })
    }

    /// Loads data from file into KeynoteFile structure     
    ///
    /// # Examples
    /// ```
    /// use std::fs;
    /// use keydata::*;
    /// 
    /// let mut file = KeynoteFile::new("kntest.dat").unwrap();
    /// file.load_data(); 
    /// fs::remove_file(file.filepath);  // remove the test file
    /// ```
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

    /// Add a key-value entry into the file
    ///
    /// # Arguments
    ///
    /// * `section_to_add_to` - section to add entry to as string slice 
    /// * `key` - key for the entry as string slice 
    /// * `value` - value of the entry as string slice 
    ///
    /// # Examples    ///
    /// ```
    /// use std::fs;
    /// use keydata::*;
    /// 
    /// let mut kn_file = KeynoteFile::new("kntest.dat").unwrap();     
    /// kn_file.add_section("leaders").unwrap();
    /// 
    /// kn_file.add_entry("leaders", "atreides", "leto");
    ///  
    /// fs::remove_file(kn_file.filepath); // remove the test file
    /// ```
    pub fn add_entry<'a>(&mut self, section_to_add_to: &str, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
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

    /// Remove a key-value entry from the file
    ///
    /// # Arguments
    /// 
    /// * `key` - key for the entry to remove as string slice  
    ///
    /// # Examples    ///
    /// ```
    /// use std::fs;
    /// use keydata::*;
    /// 
    /// let mut kn_file = KeynoteFile::new("kntest.dat").unwrap();    
    /// kn_file.add_section("leaders").unwrap();   
    /// kn_file.add_entry("leaders", "atreides", "leto");
    /// 
    /// kn_file.remove_entry("atreides");
    /// 
    /// fs::remove_file(kn_file.filepath);  // remove the test file  
    /// ```
    pub fn remove_entry(&mut self, key: &str) -> Result<(), Box<dyn Error>>{
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

    /// Remove a section from the file
    ///
    /// # Arguments
    /// 
    /// * `section_to_remove` - section to remove as string slice  
    ///
    /// # Examples    ///
    /// ```
    /// use std::fs;
    /// use keydata::*;
    /// 
    /// let mut kn_file = KeynoteFile::new("kntest.dat").unwrap();    
    /// kn_file.add_section("leaders").unwrap();   
    /// 
    /// kn_file.remove_section("leaders");
    /// 
    /// fs::remove_file(kn_file.filepath);  // remove the test file  
    /// ```
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
    
    /// Returns a reference to this files sections hashmap    
    ///
    /// # Examples    ///
    /// ```
    /// use std::fs;
    /// use keydata::*;
    /// 
    /// let mut kn_file = KeynoteFile::new("kntest.dat").unwrap();    
    /// kn_file.add_section("leaders").unwrap();   
    /// 
    /// let sections = kn_file.get_sections();
    /// 
    /// fs::remove_file(kn_file.filepath);  // remove the test file  
    /// ```
    pub fn get_sections(&self) -> &HashMap<String, Section> {
        return &self.sections;
    }

    /// Adds a new section to the file   
    /// # Arguments
    /// 
    /// * `section_name` - name of the section to add
    /// 
    /// # Examples    
    /// ```
    /// use std::fs;
    /// use keydata::*;
    /// 
    /// let mut kn_file = KeynoteFile::new("kntest.dat").unwrap(); 
    ///    
    /// kn_file.add_section("leaders").unwrap();   
    /// kn_file.add_section("villains").unwrap();  
    ///     
    /// fs::remove_file(kn_file.filepath);  // remove the test file 
    /// ```
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

    /// Gets the value of an entry in the file from a key   
    /// # Arguments
    /// 
    /// * `key` - key to search the file for
    /// 
    /// # Examples    
    /// ```
    /// use std::fs;
    /// use keydata::*;
    /// 
    /// let mut kn_file = KeynoteFile::new("kntest.dat").unwrap(); 
    ///    
    /// kn_file.add_section("leaders").unwrap();   
    /// kn_file.add_entry("leaders", "atreides", "leto");
    /// 
    /// let value = kn_file.get_value_from_key("atreides");  
    /// 
    /// println!("{}", value.unwrap());     // "leto"
    /// 
    /// fs::remove_file(kn_file.filepath);  // remove the test file     
    /// ```
    pub fn get_value_from_key(&mut self, key: &str) -> Option<&str>{           
        for (_, section) in &self.sections {
            if let Some(value) = section.data.get(key) {
                return Some(value)
            }
        } 
        None
    }
    
    /// Checks if a key is present in the file   
    /// # Arguments
    /// 
    /// * `key` - key to search the file for
    /// 
    /// # Examples    
    /// ```
    /// use std::fs;
    /// use keydata::*;
    /// 
    /// let mut kn_file = KeynoteFile::new("kntest.dat").unwrap(); 
    ///    
    /// kn_file.add_section("leaders").unwrap();   
    /// kn_file.add_entry("leaders", "atreides", "leto");
    /// 
    /// println!("{}", kn_file.contains_key("atreides"));
    /// 
    /// 
    /// fs::remove_file(kn_file.filepath);  // remove the test file     
    /// ```
    pub fn contains_key(&mut self, key: &str) -> bool {           
        for (_, section) in &self.sections {
            if section.data.contains_key(key) {
                return true;
            }
        }
        return false
    }

    /// Returns a Section from the file based on section name   
    /// # Arguments
    /// 
    /// * `section_name` - name of section to return if it exists
    /// 
    /// # Examples    
    /// ```
    /// use std::fs;
    /// use keydata::*;
    /// 
    /// let mut kn_file = KeynoteFile::new("kntest.dat").unwrap(); 
    ///    
    /// kn_file.add_section("leaders").unwrap();
    /// 
    /// println!("{}", kn_file.get_section("leaders").unwrap().name);
    /// 
    /// 
    /// fs::remove_file(kn_file.filepath);  // remove the test file     
    /// ```
    pub fn get_section(&mut self, section_name : &str) -> Option<&mut Section> {
        match self.sections.get_mut(section_name) {
            Some(section) => Some(section),
            None => None
        }
    }

    // ---------------------------------------------------- private functions
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
    
    fn add_section_to_data_structure(&mut self, section_name: &str) {
        self.sections.insert(section_name.to_string(), Section::new(section_name));
    }
    
}

// ---------------------------------------------------- tests
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
        test_file.sections.insert("test_section".to_string(), Section::new("test_section"));

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