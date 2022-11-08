fn main() {

	use num_bigint::{BigInt, Sign};

assert_eq!(BigInt::from_bytes_be(Sign::Plus, b"A"),
           BigInt::parse_bytes(b"65", 10).unwrap());
assert_eq!(BigInt::from_bytes_be(Sign::Plus, b"AA"),
           BigInt::parse_bytes(b"16705", 10).unwrap());
assert_eq!(BigInt::from_bytes_be(Sign::Plus, b"AB"),
           BigInt::parse_bytes(b"16706", 10).unwrap());
assert_eq!(BigInt::from_bytes_be(Sign::Plus, b"Hello world!"),
           BigInt::parse_bytes(b"22405534230753963835153736737", 10).unwrap());    
	println!("Hello, world!");
}
