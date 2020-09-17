use clap::{load_yaml, App};

mod todo;

fn main() {
	let args_yaml = load_yaml!("../args.yml");
	let matches = App::from_yaml(args_yaml).get_matches();

	let file_path = matches.value_of("file").unwrap().to_string();
	let todo_list = todo::TodoList::from_file(&file_path).unwrap();
	for item in todo_list.as_vec() {
		println!("{:?}", item);
	}

	matches.value_of("view").and_then(|index| -> Option<&str> {
		println!("Viewing index #{}", index);
		Some(index)
	});

	matches.value_of("edit").and_then(|index| -> Option<&str> {
		println!("Editing index #{}", index);
		Some(index)
	});
}
