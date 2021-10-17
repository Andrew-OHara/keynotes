use std::collections::HashMap;
pub struct Section {
    pub name : String,
    pub data : HashMap<String, String>
}

impl Section {
    pub fn new(name : String) -> Section {
        Section {
            name,
            data : HashMap::new()
        }
    }

    pub fn build_section_string(section_name: &str) -> String {
        let mut header_string = String::new();
        header_string.push('<');
        header_string.push_str(section_name);
        header_string.push_str(">\n");

        header_string
    } 
    
    pub fn get_section_name_from_string(line : &str) -> Option<&str> {
        if !line.contains("<") || !line.contains(">") || line.contains("\t") {  // not a valid section name
            return None
        }

        Some(&line[1..line.len()-1])
    } 

    pub fn add_entry(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}