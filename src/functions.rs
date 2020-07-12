fn sum(a: u16, b: u16) -> u16 {
  return a + b
}

fn sub(a: u16, b: u16) -> u16 {
  return a - b
}

fn mult(a: u16, b: u16) -> u16 {
  return a * b
}

// From es mejor forma de parsear tipos que as
fn div(a: u16, b: u16) -> f32 {
  return f32::from(a) / f32::from(b);
}

pub fn operations(a: u16, b: u16) -> () {
  let sum: u16 = sum(a, b);
  let sub: u16 = sub(a, b);
  let mult: u16 = mult(a, b);
  let div: f32 = div(a, b);
  println!("Los resultados de las operaciones:
    sum: {}, sub: {}, mult: {}, div: {}",
    sum, sub, mult, div);
}

// También se puede retornar una variable múltiple gracias a las tuplas
fn switch(x: i32, y: i32) -> (i32, i32) {
  // Retorna una tupla
  return (y, x);
}

pub fn execute_switch(x: i32, y: i32) {
  // Devuelve una tupla de valores
  let result = switch(x, y);
  println!("Primer intercambio con tupla: {} {}", result.0, result.1);

  // Desestructura la tupla en dos variables
  let (a, b) = switch(result.0, result.1);
  println!("Segundo intercambio con destructuring: {} {}", a, b);
}
