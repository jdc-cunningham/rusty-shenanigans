// https://www.geeksforgeeks.org/standard-i-o-in-rust/#
use std::io;
use chrono;
use std::collections::HashMap;

fn main() {
  let responses = HashMap::from([
    ("devices", "pi zero w, pi 2b"),
  ]);

  // https://stackoverflow.com/a/57708129
  let date_parts: Vec<&str> = chrono::Local::now().to_string().split("T").collect::<Vec<&str>>();
  println!("CLI Buddy v0.0.0 - {:?}", date_parts[0]);
  let mut msg = String::new();

  io::stdin().read_line(&mut msg).expect("error: unable to read user input");
  println!("{}", msg);
}

// 