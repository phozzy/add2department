use std::io;
use std::collections::HashMap;
fn get_string() -> String {
    loop {
        let mut word = String::new();
        println!("Please enter your command:");
        io::stdin().read_line(&mut word)
            .expect("Failed to read line");
        break match word.trim().parse() {
            Ok(str) => str,
            Err(_) => {
                println!("Wrong value! {}", word);
                continue;
            },
        };
    }
}
fn create_directory() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    loop {
        let mut counter = 0;
        let mut name = String::new();
        let mut department = String::new();
        let command = get_string();
        if command == "exit".to_string() {
            break map;
        } else {
	    for word in command.trim().split_whitespace() {
                if counter == 1 {
                    name = word.to_string();
                };
                if counter == 3 {
                    department = word.to_string();
                };
                counter += 1;
            };
            let department_value = map.entry(department).or_insert(Vec::new());
            department_value.push(name);
        };
    }
}
fn main() {
    print!("{:?}", create_directory());
}
