use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub enum TodoListError {
    InvalidFormat(String),
}

#[derive(Debug, Clone)]
pub struct TodoItem {
    name: String,
    desc: String,
    complete: bool,
}

pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoItem {
    pub fn new<S: Into<String>>(name: S, desc: S, complete: bool) -> TodoItem {
        let (name, desc, complete) = (name.into(), desc.into(), complete);
        TodoItem {
            name,
            desc,
            complete,
        }
    }
}

impl Default for TodoList {
    fn default() -> TodoList {
        TodoList { items: Vec::new() }
    }
}

impl TodoList {
    pub fn from_string<S: Into<String>>(string: S) -> Result<TodoList, TodoListError> {
        let mut result = Self::default();

        for line in string.into().lines() {
            let split_line: Vec<&str> = line.split(';').collect();

            if split_line.len() < 3 {
                return Err(TodoListError::InvalidFormat(
                    "File format error: Not enough information in file".to_string(),
                ));
            }

            let is_complete: bool = split_line.get(0).unwrap().parse().unwrap();
            let title: String = String::from(*split_line.get(1).unwrap_or(&"Untitled"));
            let desc: String = String::from(*split_line.get(2).unwrap_or(&""));

            result.items.push(TodoItem::new(title, desc, is_complete));
        }
        Ok(result)
    }

    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<TodoList, TodoListError> {
        let path: PathBuf = path.into();
        let file_contents = fs::read_to_string(path.as_path()).unwrap();
        Self::from_string(file_contents)
    }

    pub fn as_vec(&self) -> &Vec<TodoItem> {
        &self.items
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<TodoItem> {
        &mut self.items
    }
}
