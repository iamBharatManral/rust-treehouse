use std::io::stdin;

struct Visitor{
    name: String,
    greeting: String
}

impl Visitor{
    fn new(name: &str, greeting: &str) -> Self{
        Self{
            name: name.to_owned(),
            greeting: greeting.to_owned()
        }
    }
    fn greet(&self){
        println!("{}", self.greeting);
    }
}

fn main() {
    println!(":: 🌴 Welcome to the Treehouse 🌴 ::  🙏 ::");

    let visitors = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge. 🥛"),
        Visitor::new("fred", "Wow, who invited Fred? 🙂")
    ];

    println!("Hello, what's your name? 🤷‍");
    let name = whats_your_name();

    let known_visitor = visitors
        .iter()
        .find(|visitor| visitor.name == name);

    match known_visitor {
        Some(name) => name.greet(),
        None => println!("Sorry {}, you aren't on the list. 😞", name)
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
