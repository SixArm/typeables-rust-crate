#[test]
fn test_namespacing() {
    use ::typeables::*;

    let x: &Noun_str = "x";
    assert_eq!(x, "x");

    let x: &Verb_str = "x";
    assert_eq!(x, "x");
    
}
