use std::collections::HashMap;
pub struct Section {
    pub name : String,
    pub data : HashMap<String, String>
}

impl Section {
    pub fn new(name : &str) -> Section {
        Section {
            name: name.to_string(),
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
    
    // this both gets the name from the string and validates that the string is correct section format
    pub fn get_section_name_from_string(line : &str) -> Option<&str> {
        if !line.starts_with("<") || !line.contains(">") || line.contains("\t") || !line.ends_with("\n"){  // not a valid section name
            return None
        }
        
        // len - 2 excludes the newline and the '>'
        Some(&line[1..line.len()-2])
    } 

    pub fn add_entry(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}

#[cfg(test)]
mod tests {            
    use super::*;

    #[test]
    fn new_success() {
        let name = "section_name";
        
        let section = Section::new(name);
        
        assert_eq!(section.name, name);
        assert!(section.data.len() == 0); 
    }

    #[test]
    fn build_section_string_success() {
        let result = Section::build_section_string("test");
        assert_eq!(result, "<test>\n");
    }

    #[test]
    fn get_section_name_from_string_success() {       
        let result = Section::get_section_name_from_string("<test_section>\n");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "test_section");        
    }

    #[test]
    fn get_section_name_from_string_no_newline_expect_none() {
        let result = Section::get_section_name_from_string("<test_section>");
        assert!(result.is_none());
    }

    #[test]
    fn get_section_name_from_string_invalid_start_expect_none() {
        let result = Section::get_section_name_from_string("test_section>");
        assert!(result.is_none());
    }

    #[test]
    fn get_section_name_from_string_missing_piece_expect_none() {
        let result = Section::get_section_name_from_string("<test_section\n");
        assert!(result.is_none());
    }

    #[test]
    fn add_entry_success() {
        let mut section = Section::new("test_section");
        assert!(section.data.len() == 0);

        section.add_entry("test_key", "test_value");

        assert!(section.data.len() == 1);
        assert_eq!(section.data.get("test_key").unwrap(), "test_value");
    }
}