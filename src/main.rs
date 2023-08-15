use std::io::stdin;

#[derive(Debug)]
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

    let mut visitors = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge. 🥛"),
        Visitor::new("fred", "Wow, who invited Fred? 🙂")
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
                        Visitor::new(&name, "New Friend")
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
