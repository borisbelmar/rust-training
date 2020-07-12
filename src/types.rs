pub fn types() {
  let x = 12; // Por defecto es i32
  let a = 12u8; // Se puede declarar explicitamente como unsigned 8
  let b = 4.3; // Por defecto es f64
  let c = 4.3f32; // Se puede declarar explicitamente como float 32
  let bv = true; // Declarado boolean
  let t = (13, false); // Una tupla fija de valores
  let sentence = "¡Hola, mundo!"; // Implicitamente un slice de una cadena de texto &[T]
  // Println permite imprimir variables de cualquier tipo
  println!(
      "Algunos tipos:
      i32: {}
      u8: {}
      f64: {}
      f32: {}
      boolean: {}
      tupla índice 0: {}
      tupla índice 1: {}
      slice: {}",
      x, a, b, c, bv, t.0, t.1, sentence
  );
}

pub fn conversion() {
  // Se debe hacer la conversión de tipos para hacer operaciones entre tipos distintos
  // Para esto se usa la expresión 'as'
  let a = 13u8;
  let b = 7u32;
  let c = a as u32 + b;
  println!("Suma de un u8 y un u32: {}", c);

  let t = true;
  println!("boolean conversión a u8: {}", t as u8);
}