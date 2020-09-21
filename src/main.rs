use clap::{load_yaml, App};

mod cli;
mod file;
mod todo;

fn main() {
	let args_yaml = load_yaml!("args.yml");
	let matches = App::from_yaml(args_yaml).get_matches();

	let file_path = matches.value_of("file").unwrap().to_string();

	match file::read_todo_file(&file_path) {
		Ok(list) => match matches.value_of("view") {
			Some(index_str) => match index_str.parse::<usize>() {
				Ok(index) => cli::print_item(list.get(index - 1)),
				Err(e) => eprintln!("Invalid item index: {}", e),
			},
			None => cli::print_list(&list),
		},
		Err(_) => eprintln!("Failed to read .todo file."),
	}

	matches.value_of("edit").and_then(|index| -> Option<&str> {
		println!("Editing index #{}", index);
		Some(index)
	});
}
