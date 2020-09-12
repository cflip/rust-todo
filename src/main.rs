use clap::{load_yaml, App};

mod todo;

fn main() {
    let args_yaml = load_yaml!("../args.yml");
    let matches = App::from_yaml(args_yaml).get_matches();

    let file_path = matches.value_of("file").unwrap().to_string();
    let todo_list = todo::TodoList::from_file(&file_path).unwrap();

    // Example code that you could implement:
    //      todo_list.get_item("laundry").uncheck();
    //      todo_list.get_item("code rust").check();
    //      todo_list.get_item("foo").rename("oof");
    //      todo_list.remove_item("foo");
    //      todo_list.add_item(TodoItem::new("watch movie", None, false));

    matches.value_of("view").and_then(|index| -> Option<&str> {
        println!("Viewing index #{}", index);
        Some(index)
    });

    matches.value_of("edit").and_then(|index| -> Option<&str> {
        println!("Editing index #{}", index);
        Some(index)
    });
}
