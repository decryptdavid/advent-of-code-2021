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

fn process_to_diff(numbers: &Vec<u32>, mut previous_number: Option<u32>) -> Vec<Diff> {
  numbers.iter()
    .map(|number| {
      match &previous_number {
        Some(x) => {
          if number > x {
            previous_number = Some(*number);
            Diff::Inc
          } else if number == x {
            Diff::NoChange
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
    .collect()
}

pub fn run() {
  let numbers = load_input();
  let len = numbers.len();
  let mut previous_number: Option<u32> = None;

  let differences: Vec<Diff> = process_to_diff(&numbers, previous_number);

  println!("Answer part 1: {:?}", differences.len());
  previous_number = None;

  let groups = 2;
  let mut david: Vec<u32> = vec![];

  for (index, number) in numbers.iter().enumerate() {
    if index < (len - groups) {
      let value = number + numbers[index+1] + numbers[index+2];
      david.push(value);
    }
  }
  
  let ldifferences: Vec<Diff> = process_to_diff(&david, previous_number);

  println!("Answer part 2: {:?}", ldifferences.len());
}
