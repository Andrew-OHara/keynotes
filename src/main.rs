use std::{env, process, fs::OpenOptions, io, io::{Write, prelude::*}, collections::HashMap};

const NO_OPTIONS : i32 = -1;
const NO_HOME : i32 = -1;

struct Section {
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


fn is_alphabetic(s : &str) -> bool {
    for c in s.chars() {
        if !c.is_alphabetic() {            
            return false
        }
    }

    true
}

struct KeynoteFile {
    path_buf : std::path::PathBuf,
    sections : HashMap<String, Section>
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

    fn add_section(&mut self, section_name : &str) {       
        if !is_alphabetic(section_name) {
            println!("{} is not a valid section name", section_name);
            return
        }        

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

fn main() {
    let args: Vec<String> = env::args().collect();

    // fail if no arguments passed, otherwise get option param 
    let option = args.get(1);
    let option = if let None = option {

        println!("kn usage :    kn -[option]      option is mandatory.  kn -help for valid options");
        process::exit(NO_OPTIONS);
    
    } else {
       
        option.unwrap()
    
    };

    println!("{}", option);

    // build path to keynotes.dat file        
    let mut data_filepath = match home::home_dir() {
        Some(path_buffer) => path_buffer,
        None => {
            println!("error: could not find the home folder");
            process::exit(NO_HOME); 
        }
    };        
    data_filepath.push(".keynotes/keynotes.dat");

    // create file struct
    let mut file = KeynoteFile {
        sections : HashMap::new(),
        path_buf : data_filepath
    };   

    // handle various run modes as delineated by option
    match option.as_str() {
        "-as"   => {                                    // add section

            let section_name_opt = args.get(2);
            if let Some(section_name) = section_name_opt {
                file.add_section(section_name);
            }      
            else {
                println!("add section usage:    kn -as [sectionName]     sectionName is mandatory.  see kn -help for details")
            }     

        },

        "-rs"   => {},
        "-ls"   => {},
        "-ak"   => {},
        "-rk"   => {},
        "-lk"   => {},
        "-fd"   => {},
        // TODO: put the help string into a file that gets loaded
        "h" => {
            print!(" keynotes v0.1.0:\n\n\tlegend:\t\t[] - mandatory    () - optional\n\n\tusage:\t kn [-action] [action params]\
            (additional params)\n\n\tactions:\n\n\t\t -as [sectionName]   Add Section: adds a section to the file with sectionName \
            action param as the name. Disallows duplicate section names. \n\t\t\t\t\t\t  Section names must be alphabetical\n");
        }

        _ => {}
    }
 }