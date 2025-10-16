use std::env;

use ravensone::version;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--version" {
        println!("RavensOne CLI v{}", version());
    } else {
        println!("RavensOne CLI v{}", version());
    }
}
