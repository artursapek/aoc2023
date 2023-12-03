use super::*;

pub struct Part1();
pub struct Part2();

impl Solver for Part1 {
    fn solve(self, input: io::BufReader<File>) -> String {
        let sum: i32 = input
            .lines()
            .map(|line| {
                let digits: Vec<char> = line
                    .unwrap()
                    .chars()
                    .filter(|character| character.is_digit(10))
                    .collect();

                let first = digits.first().unwrap();
                let last = digits.last().unwrap();

                format!("{}{}", first, last).parse::<i32>().unwrap()
            })
            .sum();

        format!("{}", sum)
    }
}

impl Solver for Part2 {
    fn solve(self, input: io::BufReader<File>) -> String {
        let sum: i32 = input
            .lines()
            .map(|line| {
                let mut digits: Vec<u32> = Vec::new();
                let mut word = String::new();

                for character in line.unwrap().chars() {
                    if character.is_digit(10) {
                        // Number

                        digits.append(&mut word_to_digits(word));

                        digits.push(character.to_digit(10).unwrap());

                        word = String::new();
                    } else {
                        // Letter
                        word.push(character);
                    }
                }

                digits.append(&mut word_to_digits(word));

                let first = digits.first().unwrap();
                let last = digits.last().unwrap();

                format!("{}{}", first, last).parse::<i32>().unwrap()
            })
            .sum();

        format!("{}", sum)
    }
}

fn word_to_digits(mut word: String) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    while word.len() > 0 {
        if word.ends_with("one") {
            digits.push(1);
        } else if word.ends_with("two") {
            digits.push(2);
        } else if word.ends_with("three") {
            digits.push(3);
        } else if word.ends_with("four") {
            digits.push(4);
        } else if word.ends_with("five") {
            digits.push(5);
        } else if word.ends_with("six") {
            digits.push(6);
        } else if word.ends_with("seven") {
            digits.push(7);
        } else if word.ends_with("eight") {
            digits.push(8);
        } else if word.ends_with("nine") {
            digits.push(9);
        }

        word.pop();
    }

    digits.reverse();

    digits
}
