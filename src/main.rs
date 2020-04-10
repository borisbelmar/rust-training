mod helloworld;
mod operators;

fn main() {
  let name: &str = "Boris";
  helloworld::hello_world(name);
  let sum: u16 = operators::sum(10, 10);
  let sub: u16 = operators::sub(10, 5);
  let mult: u16 = operators::mult(10, 5);
  let div: f32 = operators::div(3, 6);
  println!("The results are sum: {}, sub: {}, mult: {}, div: {}", sum, sub, mult, div);
}
