// -----------------------------------------------------------------------------
// automatic way

// use serde::{Serialize, Deserialize};
//
// #[derive(Serialize, Deserialize, Debug)]
// struct Color {
//     r: i32,
//     g: f64,
//     b: String,
// }
//
// fn main() {
//     let c = Color {r: 55, g:55.55, b:String::from("555")};
//     let c_str = serde_json::to_string(&c).unwrap();
//     println!("serialized string is {:?}", c_str);
// }


// -----------------------------------------------------------------------------
// manual way (custom implementation)
// as per https://serde.rs/impl-serialize.html
// and https://www.joshmcguigan.com/blog/understanding-serde/

use serde::{Serialize, Serializer, ser::SerializeStruct};

struct Color {
    r: i32,
    g: f64,
    b: String,
}


impl serde::Serialize for Color {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut serde_state = match Serializer::serialize_struct(
			serializer,
			"Color",
			false as usize + 1 + 1,
		) {
			Ok(val) => val,
			Err(err) => {
				return Err(err);
			}
		};
        //call 3 times coz we have 3 fields on the struct
		match SerializeStruct::serialize_field(&mut serde_state, "r", &self.r) {
			Ok(val) => val,
			Err(err) => {
				return Err(err);
			}
		};
		match SerializeStruct::serialize_field(&mut serde_state, "g", &self.g) {
			Ok(val) => val,
			Err(err) => {
				return Err(err);
			}
		};
        match SerializeStruct::serialize_field(&mut serde_state, "b", &self.b) {
			Ok(val) => val,
			Err(err) => {
				return Err(err);
			}
		};
		SerializeStruct::end(serde_state)
	}
}

fn main() {
    let c = Color {r: 55, g:55.55, b:String::from("555")};
    let c_str = serde_json::to_string(&c).unwrap();
    println!("serialized string is {:?}", c_str);
}