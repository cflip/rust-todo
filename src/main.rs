use clap::{load_yaml, App};

fn main() {
	let args_yaml = load_yaml!("../args.yml");
	let matches = App::from_yaml(args_yaml).get_matches();

	let file_path = matches.value_of("file").unwrap();
	println!("Will be reading from: \n{}", file_path);

	matches.value_of("view").and_then(|index| -> Option<&str> {
		println!("Viewing index #{}", index);
		Some(index)
	});

	matches.value_of("edit").and_then(|index| -> Option<&str> {
		println!("Editing index #{}", index);
		Some(index)
	});
}
