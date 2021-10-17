pub fn ensure_newline(line: &str) -> String {
    if let Some(c) = line.chars().last() {
        if c != '\n' {
            return String::from(format!("{}{}", line, '\n'));
        }
    }

    return line.to_string();
}

pub fn is_alphabetic(s : &str) -> bool {
    for c in s.chars() {
        if !c.is_alphabetic() {            
            return false
        }
    }

    true
}