#![allow(dead_code)]

#[derive(Clone)]
enum Class { Rogue, Mage, Warrior, Priest }

#[derive(Clone)]
enum DmgType { Blunt, Slash, Stab, Fire, Dark}

#[derive(Clone)]
enum State { Poison, Paralized, Burn, Stun, Bleeding, None }

#[derive(Clone)]
struct Ammunition {
  dmg: u8,
  dmg_type: DmgType,
  state: State
}

#[derive(Clone)]
enum Weapon {
  Melee(u8, DmgType, State),
  Ranged(u8, Ammunition, u8),
  Magic(u8, DmgType, State, u8),
  None
}

#[derive(Clone)]
struct Character {
  name: String,
  class: Class,
  lvl: u8,
  hp: u16,
  sp: u16,
  weapon: Weapon
}

fn main() {
  const IRON_ARROW: Ammunition = Ammunition {
    dmg: 2,
    dmg_type: DmgType::Stab,
    state: State::None
  };
  const STEEL_ARROW: Ammunition = Ammunition {
    dmg: 4,
    dmg_type: DmgType::Stab,
    state: State::None
  };
  const FIRE_ARROW: Ammunition = Ammunition {
    dmg: 6,
    dmg_type: DmgType::Stab,
    state: State::None
  };

  const AMMUNITION: [Ammunition; 3] = [IRON_ARROW, STEEL_ARROW, FIRE_ARROW];

  let adventurer = Character {
    name: String::from("Shadowlord"),
    class: Class::Rogue,
    lvl: 1,
    hp: 10,
    sp: 5,
    weapon: Weapon::Melee(2, DmgType::Slash, State::Bleeding)
  };

  fn print_class(character: Character) {
    match character.class {
      Class::Rogue => println!("{} ha crecido robando en las calles y es así como se convirtió en un hábil ladrón.", character.name.clone()),
      Class::Mage => println!("A {} le apasionaba el conocimiento, y es así como se convirtió en un poderoso mago.", character.name.clone()),
      Class::Priest => println!("{} juró su lealtad a los dioses y se transformó en un importante sacerdote.", character.name.clone()),
      Class::Warrior => println!("{} desde joven se dedicó a vivir lleno de aventuras, y es así como se transformó en guerrero.", character.name.clone())
    }
  }

  fn get_status(state: State) -> String {
    return match state {
      State::Bleeding => String::from("Produce un gran sangrado."),
      State::Burn => String::from("Produce quemaduras."),
      State::Stun => String::from("Aturde al oponent."),
      State::Poison => String::from("El oponente está envenenado."),
      State::Paralized => String::from("El oponente está paralizado!"),
      State::None => String::from("")
    }
  }

  fn get_dmg_type(dmg:u8, dmg_type: DmgType) -> String {
    let str_dmg = match dmg_type {
      DmgType::Blunt => format!("acestando un poderoso golpe contundente, haciendo {} puntos de daño.", dmg),
      DmgType::Slash => format!("acestando un rápido corte, haciendo {} puntos de daño.", dmg),
      DmgType::Stab => format!("acestando un golpe perforante, causando {} puntos de daño.", dmg),
      DmgType::Fire => format!("acestando un golpe de fuego, causando {} puntos de daño.", dmg),
      DmgType::Dark => format!("acestando un golpe de oscuridad, causando {} puntos de daño.", dmg)
    };
    return String::from(str_dmg)
  }

  fn print_weapon (character: Character) {
    match character.weapon {
      Weapon::Melee(dmg, dmg_type, state) => {
        println!("{} ataca sin piedad con su espada {} {}", character.name, get_dmg_type(dmg, dmg_type), get_status(state))
      },
      Weapon::Magic(dmg, dmg_type, state, sp) => {
        println!("{} canaliza su magia con {} puntos de su energía, {} {}", character.name, sp, get_dmg_type(dmg, dmg_type), get_status(state))
      },
      Weapon::Ranged(dmg, ammunition, ammo) => {
        println!("{} apunta y dispara con su arco, {} {} Le quedan {} flechas en su karkaj", character.name, get_dmg_type(dmg + ammunition.dmg, ammunition.dmg_type), get_status(ammunition.state), ammo)
      }
      Weapon::None => {
        println!("{} ataca sin un arma y no hace nada de daño, mientras su oponente piensa como darle el golpe final.", character.name)
      }
    };
  }

  println!("Esta es la historia de {}, un aventurero de nivel {}.", adventurer.name, adventurer.lvl);
  print_class(adventurer.clone());
  println!("Caminando por el bosque, con {} puntos de vida y {} puntos de magia, se encuentra con un poderoso mago malvado.", adventurer.hp, adventurer.sp);
  print_weapon(adventurer.clone());
}