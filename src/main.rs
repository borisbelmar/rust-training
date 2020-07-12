mod helloworld;
mod variables;
mod types;
mod arrays;
mod functions;
mod control;
mod structs;

fn main() {
  let name: &str = "Boris";

  helloworld::hello_world(name);

  variables::variables();

  types::types();
  types::conversion();

  arrays::arrays();

  functions::operations(5, 5);
  functions::execute_switch(123, 321);

  control::if_is_7(8);
  control::exec_loop(30);
  control::exec_while(20);
  control::exec_for();
  control::match_number(132);
  control::exec_loop_return();
  control::exec_block_expressions();

  structs::get_length("Boris");
  structs::get_seacreatures();
  structs::print_location(50, 70);
  structs::get_player_state();
}
