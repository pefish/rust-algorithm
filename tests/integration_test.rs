use rust_algorithm::Algorithm;

#[test]
fn test() {
    let result = Algorithm::gcd_of_strings(String::from("ABCABC"), String::from("ABC"));
    println!("{}", result.unwrap());
    // assert_eq!("hello", hello());
}