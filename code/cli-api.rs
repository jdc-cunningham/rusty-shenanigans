// https://www.geeksforgeeks.org/standard-i-o-in-rust/#
use std::io;

fn main() {
  println!("Write something");
  let mut msg = String::new();

  io::stdin().read_line(&mut msg).expect("error: unable to read user input");
  println!("{}", msg);
}

// 