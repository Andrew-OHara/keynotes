# keydata

A small library for storing string data in simple data files. Useful for config files. 
Data is stored as key-value pairs organized into sections and saved with a simple custom format.
Files are saved in .keynotes folder in the users home folder

## version

0.1.0

## usage

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let mut file = keydata::KeynoteFile::new("kntest.dat")?;    // saved in hidden folder in users home dir   
    file.load_data()?;
    file.add_section("sectionname")?;
    file.add_entry("sectionname", "somekey", "somevalue")?;
    
    // list all the keys in the file
    for (_, section) in file.get_sections() {   
        if section.data.len() != 0 {
           println!("{}", section.name)
        }    
      
        for (k, _) in section.data.iter() {
            println!("\t{}", k);
        }
    }    

    fs::remove_file(file.filepath);  // remove the test file
     
    Ok(()) 
}
```

## License
[MIT](https://choosealicense.com/licenses/mit/)