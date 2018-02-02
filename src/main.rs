#[macro_use]
extern crate yew;

use yew::prelude::*;

type Context = ();


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


impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: &mut Env<Context, Self>) -> Self {
        Model {
            todos: vec![],
            input: "".to_string(),
            edit_input: "".to_string(),
        }
    }

    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
            match msg {
            Msg::Add => {
                let t = Todo {
                    text: self.input.clone(),
                    edit: false,
                };
                self.todos.push(t);
                self.input = "".to_string();
            }
            Msg::Update(s) => {
                self.input = s;
            }
            Msg::Remove(i) => {
                self.todos.remove(i);
            }
            Msg::RemoveAll => {
                self.todos = vec![];
            }
            Msg::UpdateEdit(s) => {
                //assigns the string s to edit_input.
                self.edit_input = s;
            }
            Msg::Edit(i) => {
                //creates a new todo from the edited text.
                let val = Todo {
                    text: self.edit_input.clone(),
                    edit: false,
                };
                self.todos.remove(i);
                self.todos.push(val);
            }
            Msg::Toggle(i) => {
                //gets todo from vector then looks at edit field.
                let todo = self.todos.get_mut(i).unwrap();
                todo.edit = !todo.edit;
            }
            Msg::Nothing => {}            
        } // end match
        
        true
    }// end update
} //end impl Component<Context> for Model

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
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
                    value=&self.input,
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
                {for self.todos.iter().enumerate().map(view_todo)}
                </ul>
            </div>
        }
       
    } // end fn view(&self) -> Html<Context, Self>
} // end impl Renderable<Context, Model> for Model

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
