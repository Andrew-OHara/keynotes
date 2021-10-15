use keynotes::is_alphabetic;

#[test]
fn integration_is_alphabetic_true() {
    let alphabetic_str = "alphabetic";
    let alphabetic: bool = is_alphabetic(alphabetic_str);
    assert_eq!(alphabetic, true);
}