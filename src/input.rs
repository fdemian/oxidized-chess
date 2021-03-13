extern crate regex;

pub mod input {

  use std::io::{stdin,stdout,Write};
  use regex::Regex;

  const INPUT_REGEX:&str = "[A-F]+[1-8]+,[A-F]+[1-8]+";
  const INPUT_EXAMPLE:&str = "A3,D5 (First part is current position, second part is the destination).";

  fn get_user_input() -> String {

    let mut input = String::new();
    print!("Move ({}): ", INPUT_REGEX);
    let _=stdout().flush();
    stdin().read_line(&mut input).expect("Please enter a valid string.");

    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }

    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }

    return input;

 }


  pub fn get_move_from_user() -> String {

      let mut valid:bool = false;
      let mut valid_input:String = String::new();

      while !valid {

        let input = &get_user_input();
        valid = validate_user_input(&input);

        if !valid {
          println!("Entered string in invalid format. Enter a valid chess move, EJ:{0}", INPUT_EXAMPLE);
        }
        else {
          valid_input = String::from(input);
        }
      }

      return valid_input;
  }

  // AB, CD
  pub fn validate_user_input(input: &String) -> bool {
     let re = Regex::new(r"^[A-F][1-8],[A-F][1-8]$").unwrap();
     return re.is_match(input);
  }

}
