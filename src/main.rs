use std::io::stdin;

fn main() {
    let visitors = ["bert", "steve", "fred"];

    println!("Hello, what's your name? 🤷‍");
    let name = whats_your_name();
    println!("Hello, {}! 👋", name);

    let mut allow_them_in = false;
    for visitor in &visitors {
        if visitor == &name {
            allow_them_in = true
        }
    }

    if allow_them_in {
        println!("🌴 Welcome to the Treehouse 🌴, {}! 🙏", name);
    }else {
        println!("Sorry, you aren't on the list. 😞");
    }
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
