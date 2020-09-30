use crate::todo::TodoItem;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum TodoFileError {
	InvalidFormat(String),
}

pub fn read_todo_file<P: Into<PathBuf>>(path: P) -> Result<Vec<TodoItem>, TodoFileError> {
	let path = path.into();
	let file_contents = fs::read_to_string(path.as_path()).unwrap();

	let mut result: Vec<TodoItem> = Vec::new();

	for line in file_contents.lines() {
		let split_line: Vec<&str> = line.split(';').collect();

		if split_line.len() < 3 {
			return Err(TodoFileError::InvalidFormat(
				"TODO format error: Not enough information in file".to_string(),
			));
		}

		let is_complete: bool = split_line.get(0).unwrap().parse().unwrap();
		let title = String::from(*split_line.get(1).unwrap_or(&"Untitled"));
		let desc = String::from(*split_line.get(2).unwrap_or(&""));

		result.push(TodoItem::new(title, desc, is_complete));
	}
	Ok(result)
}
