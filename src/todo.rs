#[derive(Debug, Clone)]
pub struct TodoItem {
	name: String,
	desc: String,
	complete: bool,
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
