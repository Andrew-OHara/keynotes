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
    let mut file = keynotes::KeynoteFile::new("keynotes.dat")?;   
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
                println!("add section usage:    kn -as [sectionName]     sectionName is mandatory.  see kn -help for details");
            }

        },

        "-rs" => {

            if let Some(section_to_remove) = args.get(2) {                
                println!("removing {}", section_to_remove);
                file.remove_section(section_to_remove)?;                
            }
            else {
                println!("remove section usage:    kn -rs [sectionName]     sectionName is mandatory.  see kn -help for details");
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
        "-ak" => {

            if args.len() != 5 {
                println!("add key usage:    kn -ak [sectionToAddTo] [key] [value]       all options mandatory.  see kn -help for details"); 
                return Ok(())                  
            }        

            if let (Some(section_to_add_to), Some(key), Some(value)) = (args.get(2), args.get(3), args.get(4)) {                
                println!("adding <{}>  {}  to  {}", key, value, section_to_add_to);
                if let Err(e) = file.add_key(section_to_add_to, key, value) {
                    if  e.to_string() == "key: {} already exists. no key added." || 
                        e.to_string() == "cannot add to '{}'. that section does not exist" {
                        
                        println!("{}", e.to_string());
                    }
                    else {
                        return Err(e.to_string().into());
                    }
                }                               
            }
            else {
                return Err("parameters not valid. no key added.".into());
            };
            
        },
        "-rk" => {

            if args.len() != 3 {                
                return Err("list data usage:    kn -rk [key]      key is mandatory.  see kn -help for details".into());                
            }
            if let Some(key) = args.get(2) {
                println!("removing key: {}", key);
                if let Err(e) = file.remove_key(key) {
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
        "-ld" => {

            if args.len() != 3 {                
                return Err("list data usage:    kn -ld [key]      key is mandatory.  see kn -help for details".into());                
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
            print!(" keynotes v0.1.0:\n\n\tlegend:\t\t[] - mandatory    () - optional\n\n\tusage:\t kn [-action] [action params]\
            (additional params)\n\n\tactions:\n\n\t\t -as [sectionName]   Add Section: adds a section to the file with sectionName \
            action param as the name. Disallows duplicate section names. \n\t\t\t\t\t\t  Section names must be alphabetical\n");
        }        
    };

    Ok(())  

 }