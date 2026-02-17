use std::env;
mod fetch;

fn main() {
    // > lcfetch <number> <location> <language>
    let args: Vec<String> = env::args().collect();

    let num: &String = &args[1];
    let file_path: &String = &args[2];
    let lang: &String = &args[3];

    println!(
        "LeetCode {0}, saved in {1}, file type {2}.",
        num, file_path, lang
    );

    fetch::api(num);
}
