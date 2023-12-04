use std::env;
use std::fs;

fn main() {
	// Collect the arguments
	let args = env::args().collect::<Vec<String>>();
	println!("{}", fs::read_to_string(handle_args(args.as_slice())).unwrap())
}

fn handle_args(args: &[String]) -> &String{
	if args.len() < 2 {
		panic!("Please provide the source code");
	} else if args.len() > 2 {
		panic!("Please only provide the source code, nothing more");
	} else if !args.last().is_some_and(|item| item.ends_with(".arb")) {
		panic!("Please provide the source code in a .arb file, the argument provided is not correct");
	} else {
		args.last().unwrap()
	}
}