fn main() {
	let str: String = String::from("some string");
	let str = move_ownership_and_give_back(str);
	println!("this is valid: {}", str);
}

fn move_ownership_and_give_back(str: String) -> String {
	println!("this function now has ownership of the following string: {}" , str);
	println!("now moving ownership to wherever the return value is used");
	return str;
}