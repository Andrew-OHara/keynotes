use std::collections::HashMap;
/// A Section to hold keynote file entries (key-value pairs)
pub struct Section {
    /// name of the Section
    pub name : String,
    /// hashmap to hold key value pairs that make up entries
    pub data : HashMap<String, String>
}

impl Section {
    /// Returns a Section with the name given
    ///
    /// # Arguments
    ///
    /// * `name` - section name as string slice
    ///
    /// # Examples    ///
    /// ```
    /// use keydata::Section; 
    /// let s = Section::new("test_section");
    /// assert_eq!(s.name, "test_section");
    /// assert_eq!(s.data.len(), 0);
    /// ```
    pub fn new(name : &str) -> Section {
        Section {
            name: name.to_string(),
            data : HashMap::new()
        }
    }

    /// Formats a string into the form it appears as in the data file
    ///
    /// # Arguments
    ///
    /// * `section_name` - name of section to format as string slice
    ///
    /// # Examples    ///
    /// ```
    /// use keydata::Section; 
    /// let s = Section::build_section_string("test_section");
    /// assert_eq!(s, "<test_section>\n"); 
    /// ```
    pub(in super) fn build_section_string(section_name: &str) -> String {
        let mut header_string = String::new();
        header_string.push('<');
        header_string.push_str(section_name);
        header_string.push_str(">\n");

        header_string
    } 
    
    /// Returns name of a section from formatted section string. None if line is not valid section string;
    ///
    /// # Arguments
    ///
    /// * `line` - string slice containing section string
    ///
    /// # Examples    ///
    /// ```
    /// use keydata::Section; 
    /// let line = "<test_section>\n";
    /// let sn = Section::get_section_name_from_string(line);
    /// assert!(sn.is_some());
    /// assert_eq!(sn.unwrap(), "test_section"); 
    /// ```
    pub(in super) fn get_section_name_from_string(line : &str) -> Option<&str> {
        if !line.starts_with("<") || !line.contains(">") || line.contains("\t") {  // not a valid section name
            return None
        }
        
        let chars_to_subtract = if line.ends_with("\n") {   2   } else {    1   };

        // len - 2 excludes the newline and the '>'
        Some(&line[1..line.len()-chars_to_subtract])
    } 

    /// Adds a key-value pair entry to the Sections data
    ///
    /// # Arguments
    ///
    /// * `key` - entry key as string slice
    /// * `value` - entry value as string slice
    ///
    /// # Examples    ///
    /// ```
    /// use keydata::Section; 
    /// let mut s = Section::new("test_section");
    /// assert_eq!(s.data.len(), 0);
    /// 
    /// s.add_entry("theKey", "theValue");
    /// assert_eq!(s.data.len(), 1);
    /// assert_eq!(s.data.get("theKey").unwrap(), "theValue"); 
    /// ```
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
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "test_section");
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