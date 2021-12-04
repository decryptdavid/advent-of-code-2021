use std::fs;

#[derive(Debug, PartialEq)]
enum Diff {
  Inc,
  Dec,
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
  let mut previous_number: u32 = 0;

  let differences: Vec<Diff> = numbers.iter()
    .map(|number| {
      if number >= &previous_number {
        previous_number = *number;
        Diff::Inc
      } else {
        previous_number = *number;
        Diff::Dec
      }
    })
    .filter(|diff| diff == &Diff::Inc)
    .collect();

    println!("Answer: {:?}", differences.len() - 1);
}
