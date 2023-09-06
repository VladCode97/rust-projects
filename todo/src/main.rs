struct TodoModel {
    name: String,
    description: String,
}

fn add_todo(todos: &mut Vec<TodoModel>, todo: TodoModel) {
    todos.push(todo);
    println!("Todo added");
}

fn view_todo(todos: Vec<TodoModel>) {
    for t in todos.iter() {
        println!("Name: {} -- Description: {}", t.name, t.description);
    }
}

fn main() {
    let mut todos: Vec<TodoModel> = Vec::new();
    add_todo(&mut todos,TodoModel{
        description: "I will play soccer with my dog".to_string(),
        name: "Play".to_string()
    });
    add_todo(&mut todos,TodoModel{
        description: "I am going to cook pasta today for their".to_string(),
        name: "Cooking".to_string()
    });
    add_todo(&mut todos,TodoModel{
        description: "She is going to read The king by stephen king".to_string(),
        name: "Read".to_string()
    });
    view_todo(todos);
}
