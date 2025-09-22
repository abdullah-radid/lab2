
use std::env;

fn main() {
    println!("Hello, world!");

    // Challenge: Guess the number game 
    let mut rng = rand::thread_rng();
    let number: i32 = rng.gen_range(1..=100); // inclusive 1..=100

    println!("The system generated a secret number. What is it?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let guess: i32 = input.trim().parse().expect("not a valid integer");

    if guess > number {
        println!("Too high!");
    } else if guess < number {
        println!("Too low!");
    } else {
        println!("Correct!");
    }


    //Challenge: check the roster
    for (index, argument) in env::args().enumerate() {
        println!("Argument {}: {}", index, argument);
    }

    if env::args().len() < 3 {
        eprintln!("Program requires two arguments: <file path> <search name>");
        std::process::exit(1);
    }

    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(&file_path).unwrap().lines() {
        if line == search_name {
            println!("{} did walk on the Moon!", search_name);
            return;
        }
    }

    println!("{} did NOT walk on the Moon... YET!", search_name);

    //Challenge: Represent SHapes
    struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scalar: f64) {
        self.width *= scalar;
        self.height *= scalar;
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let mut rect = Rectangle::new(3.0, 4.0);
    println!("Area: {}", rect.get_area());

    rect.scale(2.0);
    println!("Scaled area: {}", rect.get_area());
}

}
