use hello_utils;
use regex::Regex;
use world_utils;

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

extern "C" {
	fn hello();
}

pub fn do_main() {
	println!("Hello, world!");
	let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
	println!("Did our date match? {}", re.is_match("2014-01-01"));
	println!("Test util 1");
	let x = hello_utils::add_two(12);
	println!("{}", x);
	println!("Test util 2");
	let x = world_utils::test_util_2();
	println!("{}", x);

	let version = env!("CARGO_PKG_VERSION");
	let package_name = env!("CARGO_PKG_NAME");

	println!("Version: {}", version);
	println!("Package name: {}", package_name);

	println!("{}", message());

	unsafe {
		hello();
	}
}
