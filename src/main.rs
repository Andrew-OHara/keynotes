use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // fail if no arguments passed, otherwise get option param 
    let option = args.get(1);    
    let option = if let None = option {
        println!("kn usage :    kn -[option]      option is mandatory.  kn -help for valid options");
        return Ok(())         
    } else {       
        option.unwrap()    
    };
    
    // create file struct
    let mut file = keydata::KeynoteFile::new("keynotes.dat")?;   
    file.load_data()?; 

    // handle various run modes as delineated by option
    match option.as_str() {
        "-as" => {                                    // add section
            
            let section_name_opt = args.get(2);
            if let Some(section_name) = section_name_opt {                
                if let Err(e) = file.add_section(section_name) {
                    if e.to_string() == format!("'{}' is not a valid section name", section_name) ||
                        e.to_string() == "section already exists" {
                            println!("{}", e.to_string());
                            return Ok(())
                    }
                    else {
                        return Err(e);
                    }                    
                }
                println!("added section '{}'", section_name);
            }      
            else {
                println!("add section usage:    kn -as [section_name]     'section_name' is mandatory.  see kn -help for details");
            }

        },

        "-rs" => {

            if let Some(section_to_remove) = args.get(2) {                
                println!("removing {}", section_to_remove);
                file.remove_section(section_to_remove)?;                
            }
            else {
                println!("remove section usage:    kn -rs [section_name]     'section_name' is mandatory.  see kn -help for details");
            };
            
        },
        "-ls" => {

            if file.get_sections().len() == 0 {
                println!("keynotes data file is empty");
                return Ok(())
            }
            for section in file.get_sections() {
                println!("{}", section.0);
            }  

        },
        "-ae" => {

            if args.len() != 5 {
                println!("add entry usage:    kn -ae [section_to_add_to] [key] [value]       all options mandatory.  see kn -help for details"); 
                return Ok(())                  
            }        

            if let (Some(section_to_add_to), Some(key), Some(value)) = (args.get(2), args.get(3), args.get(4)) {                
                println!("adding <{}>  {}  to  {}", key, value, section_to_add_to);
                if let Err(e) = file.add_entry(section_to_add_to, key, value) {
                    if  e.to_string() == "key: {} already exists. no entry added." || 
                        e.to_string() == "cannot add to '{}'. that section does not exist" {
                        
                        println!("{}", e.to_string());
                    }
                    else {
                        return Err(e.to_string().into());
                    }
                }                               
            }
            else {
                return Err("parameters not valid. no entry added.".into());
            };
            
        },
        "-re" => {

            if args.len() != 3 {                
                return Err("remove entry usage:    kn -re [key]      key is mandatory.  see kn -help for details".into());                
            }
            if let Some(key) = args.get(2) {
                println!("removing entry with key: {}", key);
                if let Err(e) = file.remove_entry(key) {
                    if e.to_string() == "key: '{}' does not exist. nothing removed" {
                        println!("{}", e.to_string());
                    }
                    else {
                        return Err(e);
                    }
                }                                        
            }; 

        },
        "-lk" => {

            for (_, section) in file.get_sections() {   
                if section.data.len() != 0 {
                    println!("{}", section.name)
                }    
    
                for (k, _) in section.data.iter() {
                    println!("\t{}", k);
                }
            }

        },
        "-lv" => {

            if args.len() != 3 {                
                return Err("list value usage:    kn -lv [key]      key is mandatory.  see kn -help for details".into());                
            }
            if let Some(key) = args.get(2) {
                if let Some(value) = file.get_value_from_key(key) {
                    println!("{}:   {}", key, value);
                }
                else {
                    println!("key {} does not exist", key);
                };
            };  
                      
        },

        // TODO: put the help string into a file that gets loaded
        _ => {
                        
            println!("\n {}", "keynotes v0.1.0:");
            println!("\n {:>10}\t{}", "legend:",  "[] - mandatory  () - optional");
            println!("\n {:>10}\t{}", "usage:", "kn [-action] [action params] (optional params)");
            println!("\n\n {:>12}  {:<20}{:>30}\t{}", "actions:", "-as [section_name]", "add section:", 
                                                        "adds a section to the file labelled 'section_name'.");
            println!("{:>140}", "section names must be alphabetical and cannot be duplicated.");
            println!("\n\n {:>12}  {:<20}{:>30}\t{}", " ", "-rs [section_name]", "remove section:", 
                                                        "deletes a section from the file if 'section_name' exists.");
            println!("\n\n {:>12}  {:<30}{:>20}\t{}", " ", "-ls", "list sections:", 
                                                        "lists all the sections in the file.");                                            
            println!("\n\n {:>12}  {:<30}{:>18}\t{}", " ", "-ae [section_name] [key] [value]", "add entry:", 
                                                        "adds an entry to the file in 'section_name'. duplicate keys not allowed.");
            println!("\n\n {:>12}  {:<30}{:>20}\t{}", " ", "-re [key]", "remove entry:", 
                                                        "removes an entry from the file if 'key' exists.");
            println!("\n\n {:>12}  {:<30}{:>20}\t{}", " ", "-lk", "list keys:", 
                                                        "lists all the keys in the file.");
            println!("\n\n {:>12}  {:<30}{:>20}\t{}", " ", "-lv", "list value:", 
                                                        "lists a value from the file if 'key' exists.\n");                                            
        
            
        }        
    };

    Ok(())  

 }