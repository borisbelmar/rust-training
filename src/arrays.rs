pub fn arrays () {
  // Arreglo de un tipo de dato y longitud fija
  // [T;N] Tipo;Longitud
  let nums: [i32; 3] = [1, 2, 3];
  // En el println, el uso de {:?} corresponde a una sentencia de depuración
  println!("Todo el arreglo de i32: {:?}", nums);
  println!("El elemento [1] del arreglo: {}", nums[1]);
}