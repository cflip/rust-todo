use crate::todo::TodoItem;

pub fn print_list(list: &Vec<TodoItem>) {
	for i in 0..list.len() {
		let item = list.get(i).unwrap();
		let completed_char = if item.complete { "✓" } else { " " };
		println!("#{} [{}] {}", i + 1, completed_char, item.name);
	}
}

pub fn print_item(item: Option<&TodoItem>) {
	match item {
		Some(item) => println!("\n {}\n{}", item.name, item.desc),
		None => println!("Todo list does not contain an item with that index."),
	}
}
