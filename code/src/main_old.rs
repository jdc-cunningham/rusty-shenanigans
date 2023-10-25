// https://www.geeksforgeeks.org/standard-i-o-in-rust/#
use std::io;
use chrono;
use std::collections::HashMap;
use std::str;

fn main() {
  let responses = HashMap::from([
    ("devices", "pi zero w, pi 2b"),
  ]);

  // https://stackoverflow.com/a/57708129
  let datetime: String = chrono::Local::now().to_string();
  let collection: Vec<&str> = datetime.split(" ").collect();

  println!("Home v0.1.0 - {}", collection[0]);

  let mut msg = String::new();

  io::stdin().read_line(&mut msg).expect("error: unable to read user input");

  let dev_str: String = String::from("devices");

  if msg.trim() == dev_str {
    println!("{}", responses["devices"]);
  } else {
    println!("Unknown command");
  }
}

// 