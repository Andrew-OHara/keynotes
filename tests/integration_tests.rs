use keydata::*;

#[test]
fn section_test() {
    let mut section = Section::new("testsection");
    assert_eq!(section.name, "testsection");
    assert!(section.data.len() == 0);

    section.add_entry("one", "value_one");
    section.add_entry("two", "value_two");

    assert!(section.data.len() == 2);
    assert_eq!(section.data.get("one").unwrap(), "value_one");
}

#[test]
fn keynotefile_test() {
    let mut test_file = KeynoteFile::new("kntest.dat").unwrap();
    
    test_file.add_section("testsection").unwrap();
    test_file.add_section("sectiontwo").unwrap();

    assert_eq!(test_file.get_sections().len(), 2);

    test_file.add_entry("sectiontwo", "keyone", "valueone").unwrap();
    test_file.add_entry("sectiontwo", "keytwo", "valuetwo").unwrap();

    assert_eq!(test_file.get_section("sectiontwo").unwrap().data.len(), 2);

    test_file.add_entry("testsection", "testkey", "testvalue").unwrap();
    test_file.remove_entry("keyone").unwrap();

    assert_eq!(test_file.get_section("sectiontwo").unwrap().data.len(), test_file.get_section("testsection").unwrap().data.len());

    test_file.remove_section("sectiontwo").unwrap();

    assert_eq!(test_file.get_sections().len(), 1);
    assert_eq!(test_file.get_value_from_key("testkey").unwrap(), "testvalue");

    std::fs::remove_file(test_file.filepath).unwrap();
}