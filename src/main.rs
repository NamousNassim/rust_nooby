use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1..=100);
    println!("Give your guess :  ");

    let mut urnumber = String::new();

    io::stdin().read_line(&mut urnumber).expect("hmm idk bro");

    println!("your guessed number is {urnumber}");

    let urnumber: u32 = urnumber.trim().parse().expect("a number baliz");

    match urnumber.cmp(&rand_num) {
        Ordering::Less => println!("too smaaall"),
        Ordering::Equal => println!("Good job !! "),
        Ordering::Greater => println!("Too big "),
    }
}
