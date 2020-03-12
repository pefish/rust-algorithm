use rust_algorithm::Algorithm;

fn main () {
  let result = Algorithm::gcd_of_strings(String::from("ABCABC"), String::from("ABC"));
  println!("{}", result.unwrap());
}
