#[macro_use]
extern crate yew;

use yew::html::*;

struct Model {
    input: String,
    edit_input: String,
    todos: Vec<Todo>,
}
//added Todo Type for editing.
struct Todo {
    text: String,
    //edit field allows todo to be editted independently.
    edit: bool,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    //these three msg types are for editing todos.
    Edit(usize),
    UpdateEdit(String),
    Toggle(usize),
    RemoveAll,
    Nothing,
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Add => {
            let t = Todo {
                text: model.input.clone(),
                edit: false,
            };
            model.todos.push(t);
            model.input = "".to_string();
        }
        Msg::Update(s) => {
            model.input = s;
        }
        Msg::Remove(i) => {
            model.todos.remove(i);
        }
        Msg::RemoveAll => {
            model.todos = vec![];
        }
        Msg::UpdateEdit(s) => {
            //assigns the string s to edit_input.
            model.edit_input = s;
        }
        Msg::Edit(i) => {
            //creates a new todo from the edited text.
            let val = Todo {
                text: model.edit_input.clone(),
                edit: false,
            };
            model.todos.remove(i);
            model.todos.push(val);
        }
        Msg::Toggle(i) => {
            //gets todo from vector then looks at edit field.
            let todo = model.todos.get_mut(i).unwrap();
            todo.edit = !todo.edit;
        }
        Msg::Nothing => {}
    }
}

fn view(model: &Model) -> Html<Msg> {
    //allows for editing of todos independently.
    let view_todo_edit = |(i, todo): (usize, &Todo)| if todo.edit == true {
        html!{
            <label><input type="text",
                    value=&todo.text,
                    oninput=|e: InputData| Msg::UpdateEdit(e.value),
                    onkeypress=move |e: KeyData| {
                        if e.key == "Enter" {Msg::Edit(i)} else {Msg::Nothing}
                    },
                    />
                    </label>
        }
    } else {
        html! {
            <label ondoubleclick=move|_| Msg::Toggle(i), > {format!("{} ", &todo.text)}
            </label>
        }
    };
    let view_todo = |(i, todo): (usize, &Todo)| {
        html!{
            <li>
                { view_todo_edit((i, &todo))}
            </li>
            <button onclick = move |_| Msg::Remove(i),>{"X"}</button></li>
        }
    };


    html! {
        <div>
            <h1>{"Todo App"}</h1>
            <input
                placeholder="what do you want to do?",
                value=&model.input,
                oninput=|e: InputData| Msg::Update(e.value),
                onkeypress=|e: KeyData| {
                    if e.key == "Enter" {Msg::Add} else {Msg::Nothing}
                },/>

        </div>
        <div>
            <button onclick = |_| Msg::RemoveAll, >{"Delete all Todos!"}</button>
        </div>
        <div>
            <ul>
            {for model.todos.iter().enumerate().map(view_todo)}
            </ul>
        </div>
    }
}

fn main() {
    let model = Model {
        todos: vec![],
        input: "".to_string(),
        edit_input: "".to_string(),
    };
    program(model, update, view);
}
