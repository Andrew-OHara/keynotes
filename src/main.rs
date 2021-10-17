use std::env;

use keynotes::KeynoteFile;

fn main() -> Result<(), &'static str> {
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
    let mut file = KeynoteFile::new()?;    

    // handle various run modes as delineated by option
    match option.as_str() {
        "-as" => {                                    // add section
            
            let section_name_opt = args.get(2);
            if let Some(section_name) = section_name_opt {
                file.add_section(section_name);
            }      
            else {
                println!("add section usage:    kn -as [sectionName]     sectionName is mandatory.  see kn -help for details");
            }            
        },

        "-rs" => {
            if let Some(section_to_remove) = args.get(2) {
                println!("removing {}", section_to_remove);
                file.remove_section(section_to_remove);                
            }
            else {
                println!("remove section usage:    kn -rs [sectionName]     sectionName is mandatory.  see kn -help for details");
            };
            
        },
        "-ls" => {

            file.list_sections();
        
        },
        "-ak" => {

            if args.len() != 5 {
                println!("add key usage:    kn -ak [sectionToAddTo] [key] [value]       all options mandatory.  see kn -help for details"); 
                return Ok(())                  
            }        

            if let (Some(section_to_add_to), Some(key), Some(value)) = (args.get(2), args.get(3), args.get(4)) {
                
                println!("adding <{}>  {}  to  {}", key, value, section_to_add_to);
                file.add_key(section_to_add_to, key, value)?
                               
            }
            else {
                return Err("parameters not valid. no key added.");
            };
            
        },
        "-rk" => {
            if args.len() != 3 {                
                return Err("list data usage:    kn -rk [key]      key is mandatory.  see kn -help for details");
                
            }
            if let Some(key) = args.get(2) {
                println!("removing key: {}", key);
                file.remove_key(key);                
            }; 
        },
        "-lk" => {
            file.list_keys();
        },
        "-ld" => {
            if args.len() != 3 {                
                return Err("list data usage:    kn -ld [key]      key is mandatory.  see kn -help for details");                
            }
            if let Some(key) = args.get(2) {
                if let Some(entry) = file.get_value_from_key(key) {
                    println!("{}:   {}", key, entry);
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