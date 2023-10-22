// https://www.geeksforgeeks.org/standard-i-o-in-rust/#
use std::io;
use self::chrono;

fn main() {
  // https://stackoverflow.com/a/57708129
  let date: String = chrono::Local::now();
  println!("CLI Buddy v0.0.0 - {:?}", date);
  let mut msg = String::new();

  io::stdin().read_line(&mut msg).expect("error: unable to read user input");
  println!("{}", msg);
}

// 