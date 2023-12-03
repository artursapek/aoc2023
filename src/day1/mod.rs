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

                dbg!(&line);

                for character in line.unwrap().chars() {
                    if character.is_digit(10) {
                        // Number

                        if let Some(digit) = word_to_digit(word) {
                            digits.push(digit);
                        }

                        digits.push(character.to_digit(10).unwrap());

                        word = String::new();
                    } else {
                        // Letter
                        word.push(character);
                    }
                }

                if let Some(digit) = word_to_digit(word) {
                    digits.push(digit);
                }

                let first = digits.first().unwrap();
                let last = digits.last().unwrap();

                dbg!(first, last);

                format!("{}{}", first, last).parse::<i32>().unwrap()
            })
            .sum();

        format!("{}", sum)
    }
}

fn word_to_digit(mut word: String) -> Option<u32> {
    while word.len() > 0 {
        if word.ends_with("one") {
            return Some(1)
        } else if word.ends_with("two") {
            return Some(2)
        } else if word.ends_with("three") {
            return Some(3)
        } else if word.ends_with("four") {
            return Some(4)
        } else if word.ends_with("five") {
            return Some(5)
        } else if word.ends_with("six") {
            return Some(6)
        } else if word.ends_with("seven") {
            return Some(7)
        } else if word.ends_with("eight") {
            return Some(8)
        } else if word.ends_with("nine") {
            return Some(9)
        }

        word.pop();
    }

    None
}
