use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        panic!("Less than 3 paramters.")
    }

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    print!("In file {}", filename);
}