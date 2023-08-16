use std::io::stdin;

#[derive(Debug)]
struct Visitor{
    name: String,
    age: u8,
    action: VisitorAction
}

#[derive(Debug)]
enum VisitorAction{
    Accept,
    AcceptWithNote{note: String},
    Refuse,
    Probation
}

impl Visitor{
    fn new(name: &str, age: u8, action: VisitorAction) -> Self{
        Self{
            name: name.to_owned().to_lowercase(),
            action,
            age
        }
    }
    fn greet(&self){
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house 🌴, {}", self.name),
            VisitorAction::AcceptWithNote {note} => {
                println!("Welcome to the tree house 🌴, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name)
        }
    }
}

fn main() {
    println!(":: 🌴 Welcome to the Treehouse 🌴 ::  🙏 ::");

    let mut visitors = vec![
        Visitor::new("Bert", 45, VisitorAction::Accept),
        Visitor::new("Steve", 15, VisitorAction::AcceptWithNote {note: String::from("Hi Steve. Lactose-free milk is in the fridge. 🥛")}),
        Visitor::new("Fred", 30, VisitorAction::Refuse)
    ];

    loop {
        println!("Hello, what's your name? 🤷‍ (Leave empty and press ENTER to quit)");
        let name = whats_your_name();
        let known_visitor = visitors
            .iter()
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(name) => name.greet(),
            None => {
                if name.is_empty(){
                    break;
                } else{
                    println!("Sorry {}, you aren't on the visitors list. 😞", name);
                    visitors.push(
                        Visitor::new(&name, 0, VisitorAction::Probation)
                    )
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitors);
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
