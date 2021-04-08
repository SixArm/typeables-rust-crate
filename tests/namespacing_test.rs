#[test]
fn test_namespacing() {
    use ::typeables::*;

    let x: Noun = "x".into();
    assert_eq!(x, "x");

    let x: Verb = "x".into();
    assert_eq!(x, "x");
    
}
