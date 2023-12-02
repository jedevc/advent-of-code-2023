use advent_of_code_2023::{main, Solver};

struct Day2 {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    number: u32,
    sets: Vec<Set>,
}

#[derive(Debug, PartialEq)]
struct Set {
    red: u32,
    blue: u32,
    green: u32,
}

impl Default for Set {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Solver<u32> for Day2 {
    fn new(problem: &str) -> Self {
        let games = problem
            .lines()
            .filter_map(|line| {
                let (game, sets) = line.split_once(":")?;
                let (_, game) = game.split_once(" ")?;
                let game = game.parse().ok()?;

                let sets = sets
                    .split(";")
                    .map(|set| {
                        let result = set.split(",").fold(
                            Set::default(),
                            |Set { red, green, blue }, item| {
                                let (count, color) =
                                    item.trim().split_once(" ").unwrap_or(("", ""));
                                let count = count.parse().unwrap_or(0);
                                match color {
                                    "red" => Set {
                                        red: red + count,
                                        green,
                                        blue,
                                    },
                                    "green" => Set {
                                        red,
                                        green: green + count,
                                        blue,
                                    },
                                    "blue" => Set {
                                        red,
                                        green,
                                        blue: blue + count,
                                    },
                                    _ => Set::default(),
                                }
                            },
                        );
                        result
                    })
                    .collect();

                Some(Game { number: game, sets })
            })
            .collect();
        Day2 { games }
    }

    fn solve(&self) -> (Option<u32>, Option<u32>) {
        let possible = Set {
            red: 12,
            green: 13,
            blue: 14,
        };
        let part1 = self
            .games
            .iter()
            .filter(|game| {
                game.sets.iter().all(|set| {
                    set.red <= possible.red && set.green <= possible.green && set.blue <= possible.blue
                })
            })
            .map(|game| game.number).sum();

        let part2 = self
            .games
            .iter()
            .map(|game| {
                game.sets.iter().fold(Set::default(), |prev, set| {
                    Set{
                        red: u32::max(prev.red, set.red),
                        green: u32::max(prev.green, set.green),
                        blue: u32::max(prev.blue, set.blue),
                    }
                })
            })
            .map(|set| set.red * set.blue * set.green).sum();

        (Some(part1), Some(part2))
    }
}

#[cfg(test)]
mod day2tests {
    use super::*;
    use advent_of_code_2023::test_example;

    #[test]
    fn test_example_part1() {
        test_example!(Day2, "day2.example.txt", (Some(8), None));
    }

    #[test]
    fn test_example_part2() {
        test_example!(Day2, "day2.example.txt", (None, Some(2286)));
    }
}

main!(Day2, "day2.txt");
