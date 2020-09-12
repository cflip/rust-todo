use std::path::Path;
use std::fs;

pub struct TodoItem {
	name: String,
	desc: String,
	complete: bool
}

pub fn read_file(path: &Path) -> Result<Vec<TodoItem>, &'static str> {
	let result: Vec<TodoItem> = Vec::new();
	let file_contents = fs::read_to_string(path).unwrap();

	for line in file_contents.lines() {
		let split_line: Vec<&str> = line.split(';').collect();
		
		if split_line.len() < 3 {
			return Err("File format error: Not enough information in file");
		}

		let is_complete: bool = split_line.get(0).unwrap().parse().unwrap();
		let title = split_line.get(1).unwrap_or(&"Untitled");
		let desc = split_line.get(2).unwrap_or(&""); 

		result.push(TodoItem { name: String::from(title), desc: String::from(title), complete: is_complete })
	}

	Ok(result)
}