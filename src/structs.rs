#![allow(dead_code)] // this line prevents compiler warnings

pub fn get_length(text:&str) {
  // Acceder a un método estático de String para conversión
  let s = String::from(text);
  // Usando el método de la instancia
  println!("{} is {} characters long.", s, s.len());
}

struct SeaCreature {
  // String is a struct
  animal_type: String,
  name: String,
  arms: i32,
  legs: i32,
  weapon: String,
}

pub fn get_seacreatures() {
  // SeaCreature's data is on stack
  let ferris = SeaCreature {
    // String struct is also on stack,
    // but holds a reference to data on heap
    animal_type: String::from("crab"),
    name: String::from("Ferris"),
    arms: 2,
    legs: 4,
    weapon: String::from("claw"),
  };

  let sarah = SeaCreature {
    animal_type: String::from("octopus"),
    name: String::from("Sarah"),
    arms: 8,
    legs: 0,
    weapon: String::from("none"),
  };

  println!(
    "{} is a {}. They have {} arms, {} legs, and a {} weapon",
    ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
  );
  println!(
    "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
    sarah.name, sarah.animal_type, sarah.arms, sarah.legs
  );
}

// Estructura de tupla
struct Location(i32, i32);

pub fn print_location(x:i32, y:i32) {
  let loc = Location(x, y);
  println!("Location: x{}, y{}", loc.0, loc.1);
}

// Enumeraciones
enum PlayerState {
  Playing,
  Paused,
  Idle
}

struct MediaPlayer {
  media: String,
  duration: i32,
  state: PlayerState
}

pub fn get_player_state() {
  let player = MediaPlayer {
    media: String::from("El Principito"),
    duration: 8000,
    state: PlayerState::Playing
  };

  let string_state = match player.state {
    PlayerState::Playing => "Está reproduciendo",
    PlayerState::Paused => "Está pausado",
    PlayerState::Idle => "Está inactivo"
  };

  let minutes = player.duration / 60;

  println!(
    "{} tiene una duración de {} minutos. {}",
    player.media, minutes, string_state
  )
}
