use clap::{load_yaml, App};

mod cli;
mod file;
mod todo;

fn main() {
	let args_yaml = load_yaml!("args.yml");
	let matches = App::from_yaml(args_yaml).get_matches();

	let file_path = matches.value_of("file").unwrap().to_string();
	let list = file::read_todo_file(&file_path).expect("Failed to read .todo file");

	// The program should only print a list of all items if neither --view or --edit were in the arguments
	let mut should_print_all = true;

	if let Some(index_str) = matches.value_of("view") {
		let index = index_str.parse::<usize>().expect("Invalid item index");
		cli::print_item(list.get(index - 1));
		should_print_all = false;
	}

	matches.value_of("edit").and_then(|index| -> Option<&str> {
		println!("Editing index #{}", index);
		should_print_all = false;
		Some(index)
	});

	if should_print_all {
		cli::print_list(&list);
	}
}
