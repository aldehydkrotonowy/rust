pub fn run() {
  create_box();
}

fn create_box() {
  let b = Box::new(55);
  println!("b = {}", b);
}
