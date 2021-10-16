use std::{env, process, collections::HashMap};
use keynotes::*;

const NO_OPTIONS : i32 = -0x1;
const NO_HOME : i32 = -0x2;

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
        filepath : data_filepath
    };   

    // handle various run modes as delineated by option
    match option.as_str() {
        "-as"   => {                                    // add section
            
            let section_name_opt = args.get(2);
            if let Some(section_name) = section_name_opt {
                file.add_section(section_name);
            }      
            else {
                println!("add section usage:    kn -as [sectionName]     sectionName is mandatory.  see kn -help for details");
            }
            
        },

        "-rs"   => {
            if let Some(section_to_remove) = args.get(2) {
                file.remove_section(section_to_remove);
            }
            else {
                println!("remove section usage:    kn -rs [sectionName]     sectionName is mandatory.  see kn -help for details");
            };
            
        },
        "-ls"   => {

            file.list_sections();
        
        },
        "-ak"   => {

            if args.len() < 5 {
                println!("add key usage:    kn -ak [sectionToAddTo] [key] [value]");
                return;
            }

            let section_to_add_to = args.get(2);
            let key = args.get(3);
            let value = args.get(4);

            if let (Some(s), Some(k), Some(v)) = (section_to_add_to, key, value) {
                // TODO: CURRENT - prevent duplicate keys
                file.add_key(s, k, v);
                println!("added <{}>  {}  to  {}", k, v, s);
            }
            else {
                println!("parameters not valid. no key added.");
            };
            
        },
        "-rk"   => {},
        "-lk"   => {
            file.list_keys();
        },
        "-fd"   => {},

        // TODO: put the help string into a file that gets loaded
        _ => {
            print!(" keynotes v0.1.0:\n\n\tlegend:\t\t[] - mandatory    () - optional\n\n\tusage:\t kn [-action] [action params]\
            (additional params)\n\n\tactions:\n\n\t\t -as [sectionName]   Add Section: adds a section to the file with sectionName \
            action param as the name. Disallows duplicate section names. \n\t\t\t\t\t\t  Section names must be alphabetical\n");
        }        
    };    
 }
 
 #[cfg(test)]
 mod tests {
     mod main_tests {
        use super::super::*;

        #[test]
        fn main_test() {
            assert_eq!(NO_OPTIONS, -0x1);
        }
    }
}