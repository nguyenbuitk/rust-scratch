#![allow(unused_mut, unused_variables)]

fn main() {
  let mut arg: String = std::env::args()
    .collect::<Vec<String>>()
    .iter()
    .nth(1)
    .unwrap_or_else(|| {
      println!("Please supply an argument to this program.");
      std::process::exit(-1);
    }).to_owned();

  inspect(&arg);
  change(&mut arg);
  
  println!("I have many {}", arg);

  if eat (arg) {
    println!("Might be bananas");
  } else {
    println!("Not bananas");
  }
}

fn inspect(s: &String) {
  if s.ends_with("s") {
    println!("{} is plural", s);
  } else {
    println!("{} is singular", s);
  }
}

fn change(s: &mut String) {
  if !s.ends_with("s") {
    s.push_str("s");
  }
}

fn eat(s: String) -> bool {
  if s.starts_with("b") && s.contains("a") {
    true
  } else {
    false
  }
}
