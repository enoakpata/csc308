mod greetings;

use greetings:: {french, spanish, default_greeting}

fn main() {
    println!("Hello, world!");
    println!("{}", default_greeting());
    println("{}", spanish());
    println("{}", french());
}


