use std::{env, process, collections::HashMap};
use keynotes::*;

const NO_OPTIONS : i32 = -1;
const NO_HOME : i32 = -1;

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