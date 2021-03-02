mod state;
use std::env;
use state::read_file;
use serde_json::value::Value;
use serde_json::Map;

mod todo;
use todo::todo_factory;

mod processes;
use processes::process_input;

fn main() {

    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let state: Map<String, Value> = read_file("./state.json");
    let status: String;
    //println!("CMD:{} TTL:{}", command, title);
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None=> {
            status = "pending".to_string();
        }
    }
    let item = todo_factory(&status, title).expect(&status);
    process_input(item, command.to_string(), &state);
}