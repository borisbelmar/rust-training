// El if de toda la vida
pub fn if_is_7(x: i32) {
  const SEVEN: i32 = 7;
  if x < SEVEN {
    println!("{} es menor que {}", x, SEVEN);
  } else if x == SEVEN {
    println!("{} es {}", x, SEVEN);
  } else {
    println!("{} es mayor que {}", x, SEVEN);
  }
}

// Loop ejecuta un loop infinito hasta que break sea ejecutado
pub fn exec_loop(max: i32) {
  let mut x = 0;
  loop {
    x += 1;
    if x == max {
      break;
    }
  }
  println!("X value augmented in loop: {}", x);
}

// Un while de toda la vida
pub fn exec_while(max: i32) {
  let mut x = 0;
  while x != max {
      x += 1;
  }
  println!("X value augmented in while: {}", x);
}

// El bucle for puede iterar sobre cualquier iterable
pub fn exec_for() {
  // Itera desde el 0 hasta el 4 (No incluye el último número)
  for x in 0..5 {
    println!("For in 0..5: {}", x);
  }

  // Itera desde el 0 hasta el 5 (Incluye el último número)
  for x in 0..=5 {
    println!("For in 0..=5: {}", x);
  }
}

// Match es similar a switch, pero mucho más poderoso
pub fn match_number(num: i32) {
  match num {
    0 => {
      println!("encontré 0");
    }
    // podemos hacer comparaciones con múltiples valores
    1 | 2 => {
      println!("encontré 1 o 2!");
    }
    // podemos hacer comparaciones con iteradores
    3..=9 => {
      println!("encontré un numero entre 3 y 9, ambos incluidos");
    }
    // podemos asignar el valor encontrado a una variable
    matched_num @ 10..=100 => {
      println!("encontré {}, un número entre 10 y 100!", matched_num);
    }
    matched_num @ 101..=200 => {
      println!("encontré {}, un número entre 101 y 200!", matched_num);
    }
    // esta es la condición por defecto que debe existir 
    // si no se han evaluado todos los casos
    _ => {
      println!("encontré otra cosa!");
    }
  }
}

// Loop retorna el valor declarado en el break
pub fn exec_loop_return() {
  let mut x = 0;
  let v = loop {
    x += 1;
    if x == 13 {
      break "encontré el 13";
    }
  };
  println!("Del loop: {}", v);
}

pub fn exec_block_expressions() {
  // Si dentro de una expresión no termino con ';'
  // Entonces ese será el valor retornado, en caso contrario, debo usar return
  // o será una función void
  fn inner_func () -> i32 {
    let x = 42;
    // Expresión ternaria en Rust
    let v = if x < 42 { -1 } else { 1 };
    println!("Del bloque if: {}", v);

    let food = "hamburguesa";
    let result = match food {
      // los paréntesis son opcionales cuando hay una sola expresión de retorno
      "perrito caliente" => "es un perrito caliente",
      _ => "no es un perrito caliente",
    };
    println!("Identificando comida en el match: {}", result);

    let v = {
      // Este bloque de código nos permite obtener un resultado
      // sin tener que definir una función
      let a = 1;
      let b = 2;
      a + b
    };
    println!("Del bloque: {}", v);

    // Esta es la forma de devolver un valor en Rust al final de una función
    v + 4
  }

  println!("De la función interna: {}", inner_func());
}