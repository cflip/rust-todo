use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	args.iter().for_each(|argument| {
		println!("{}", argument);
	});
}
