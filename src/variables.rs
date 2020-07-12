pub fn variables() {
  // Rust deduce el tipo de x
  let x = 13;
  println!("Variable de tipo i32 implícito: {}", x);

  // Rust también puede ser explícito con el tipo
  let x: f64 = 3.14159;
  println!("Variable de tipo f64 explícito: {}", x);

  // Se redeclara la variable del mismo nombre (Shadowing)
  let x: &str = "Boris";
  println!("Variable redeclarada de tipo &str explícito: {}", x);

  // Rust también puede declarar primero e inicializar después, pero es poco común
  // No se puede mutar esta variable
  let x;
  x = 0;
  println!("Variable inmutable: {}", x);  

  // Ahora si la variable se puede mutar
  let mut x = 0;
  println!("Variable mutable inicializada: {}", x);
  x = x + 1;
  println!("Variable mutable incrementada: {}", x);

  // Los valores constantes se declaran como SCREAMING_SNAKE_CASE
  const PI: f32 = 3.14159;
  println!("Valor constante de PI: {}", PI);
}
