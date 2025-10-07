use std::io::{self};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Lecture entier");
    let value = input.trim().parse::<i32>().unwrap();
    abracadabra(value);
}

fn abracadabra(nbr: i32) {
    for counter in 0..nbr {
        println!("{} Abracadabra", counter + 1);
    }
}
