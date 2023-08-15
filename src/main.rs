use std::io::stdin;

fn main() {
    println!("Hello, what's your name? 🤷‍");
    let name = whats_your_name();
    println!("Hello, {}", name);
}

fn whats_your_name() -> String{
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line 😔");
    your_name
        .trim()
        .to_lowercase()
}
