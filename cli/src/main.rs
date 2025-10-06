mod cli;
mod commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        return;
    }

    println!("Filename: {}", args[1]);
}
