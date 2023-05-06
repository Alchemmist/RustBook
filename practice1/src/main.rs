use std::env;
use std::io;


fn convert_temp() {
    let mut degrees = String::new();
    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line");
    let guess: i32 = degrees.trim().parse().expect("Is NOT integer");

    result: i32 = ...;
    println!("{result}");
}


fn generate_fib() {
    return
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "--temp" {
        convert_temp();
    }
    if args[1] == "--fib" {
        generate_fib();
    }    
}
