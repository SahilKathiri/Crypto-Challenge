extern crate rustc_serialize as serialize;

use serialize::hex::FromHex;
use serialize::hex::ToHex;

fn main() {
	let encoded = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	let hex = encoded.from_hex().unwrap();


	// TODO
	for i in &hex {
		println!("{}", i);	
	}
}
