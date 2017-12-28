#[macro_use]
extern crate yew;

use yew::html::*;

struct Model {
    input: String,
    todos: Vec<String>,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing,
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Add => {
            let s = model.input.clone();
            model.todos.push(s);
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
        Msg::Nothing => {}
    }
}

fn view(model: &Model) -> Html<Msg> {
    let view_todo = |(i, todo): (usize, &String)| {
        html!{
            <li>{format!("{} |", &todo)}
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
    };
    program(model, update, view);
}
