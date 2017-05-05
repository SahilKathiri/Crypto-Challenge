extern crate rustc_serialize as serialize;

use serialize::hex::FromHex;
use serialize::hex::ToHex;

fn main() {
	let op_1 = "1c0111001f010100061a024b53535009181c";
	let hex_1 = op_1.from_hex().unwrap();
	let op_2 = "686974207468652062756c6c277320657965";
	let hex_2 = op_2.from_hex().unwrap();

	let mut res = Vec::new();

	for (i, _) in hex_1.iter().enumerate() {
		res.push(hex_1[i] ^ hex_2[i]);
	}

	println!("{:?}", res.to_hex());
}
