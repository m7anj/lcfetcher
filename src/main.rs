use std::env;

fn main() {
    // > lcfetch <number> <location> <language>
    let args: Vec<String> = env::args().collect();

    let _num: &String = &args[1];
    let file_path: &String = &args[2];
    let lang: &String = &args[3];

    
}
