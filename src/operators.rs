pub fn sum(a: u16, b: u16) -> u16 {
  return a + b
}

pub fn sub(a: u16, b: u16) -> u16 {
  return a - b
}

pub fn mult(a: u16, b: u16) -> u16 {
  return a * b
}

// From es mejor forma de parsear tipos que as
pub fn div(a: u16, b: u16) -> f32 {
  return f32::from(a) / f32::from(b);
}