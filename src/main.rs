use std::env;
use json;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let result = &args[1];
    let result2 = result.replace('\n', "");
    let parsed = json::parse(&result2);
    println!("{:#?}", parsed);
}
