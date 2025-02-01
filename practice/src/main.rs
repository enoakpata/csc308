
fn main() {
    /*let age = 15;
    match age {
        18 => println!("you are an adult"),
        10..=15 => println!("you are 10!"),
        _ => println!("invalid option"),
    }*/


    // let account_balance: Option<i32> = Some(8824628);
    // match account_balance {
    //     Some(value) => println!("Value was retrived: {}", value),
    //     None => println!("Nothing was retrieved!")
    // 
    
    
        macro_rules! testing {
            ($arg:expr) => {
                println!("Testing a macro! : {}", $arg);
            };
        }

        testing!("leemao");
    }
    


