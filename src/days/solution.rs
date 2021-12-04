use std::fs;

#[derive(Debug, PartialEq)]
enum Diff {
  Inc,
  Dec,
  NoChange,
}

fn load_input() -> Vec<u32> {
  fs::read_to_string("src/days/inputs.txt")
    .unwrap()
    .lines()
    .map(|x| x.parse().unwrap())
    .collect()
}

pub fn run() {
  let numbers = load_input();
  let mut previous_number: Option<u32> = None;

  let differences: Vec<Diff> = numbers.iter()
    .map(|number| {
      match &previous_number {
        Some(x) => {
          if number >= x {
            previous_number = Some(*number);
            Diff::Inc
          } else {
            previous_number = Some(*number);
            Diff::Dec
          }
        },
        None => {
          previous_number = Some(*number);
          Diff::NoChange
        },
      }
    })
    .filter(|diff| diff == &Diff::Inc)
    .collect();

    println!("Answer: {:?}", differences.len());









}
