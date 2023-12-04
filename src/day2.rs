use super::*;

pub struct Part1();
pub struct Part2();

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

impl Solver for Part1 {
    fn solve(self, input: io::BufReader<File>) -> String {
        let sum: u32 = input
            .lines()
            .filter_map(|line| {
                let line = line.unwrap();
                let mut parts = line.split(": ");

                let game_id = String::from(parts.next().unwrap());
                let mut game_id = game_id.split(' ');
                // Skip the word "Game"
                game_id.next();
                let id = game_id.next().unwrap();
                let id = id.parse::<u32>().unwrap();

                let dice_counts = parts.next().unwrap().split(&[';', ','][..]);
                for count in dice_counts {
                    let mut split = count.trim().split(' ');
                    let count = split.next().unwrap().parse::<u32>().unwrap();
                    let color = split.next().unwrap();

                    let valid = match color {
                        "red" => count <= MAX_RED,
                        "green" => count <= MAX_GREEN,
                        "blue" => count <= MAX_BLUE,
                        _ => true, // shouldn't happen given valid input, so ignore
                    };

                    if !valid {
                        return None;
                    }
                }

                Some(id)
            })
            .sum();

        format!("{}", sum)
    }
}

impl Solver for Part2 {
    fn solve(self, input: io::BufReader<File>) -> String {
        let sum: u32 = input
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut parts = line.split(": ");

                let game_id = String::from(parts.next().unwrap());
                let mut game_id = game_id.split(' ');
                // Skip the word "Game"
                game_id.next();

                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;

                let dice_counts = parts.next().unwrap().split(&[';', ','][..]);
                for count in dice_counts {
                    let mut split = count.trim().split(' ');
                    let count = split.next().unwrap().parse::<u32>().unwrap();
                    let color = split.next().unwrap();

                     match color {
                        "red" => max_red = max_red.max(count),
                        "green" => max_green = max_green.max(count),
                        "blue" => max_blue = max_blue.max(count),
                        _ => {},
                    };
                }

                max_red * max_green * max_blue
            })
            .sum();

        format!("{}", sum)

    }
}
