fn main() {
  let mut carrot = Carrot { percent_left: 100.0 };
  carrot.bite();
  println!("I take a bite: {:?}", carrot);

  let mut grapes = Grapes { amount_left: 100 };
  grapes.bite();
  println!("Eat a grape: {:?}", grapes);
}

trait Bite {
  fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Carrot {
  percent_left: f32,
}

#[derive(Debug)]
struct Grapes {
  amount_left: i32,
}

impl Bite for Carrot {
  // similar to 
  // fn do_stuff(s: &mut String) {}
  fn bite (self: &mut Self) {
    self.percent_left *= 0.8;
  }
}

impl Bite for Grapes {
  fn bite(self: &mut Self) {
    self.amount_left -= 1;
  }
}
