extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

  let val = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number!");
    loop {
      println!("Input your guess!");

      let mut guess = String::new();

      io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      println!("You guessed: {}", guess);

      match guess.cmp(&val) {
          Ordering::Less => println!("Too small"),
          Ordering::Greater => println!("Too big"),
          Ordering::Equal => {
            println!("Exact!");
            break;
          }
      }
        
    }

}
