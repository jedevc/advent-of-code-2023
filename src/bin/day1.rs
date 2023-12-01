use std::collections::HashMap;

use advent_of_code_2023::{main, Solver};

struct Day1 {
    lines: Vec<String>,
}

impl Solver<u32> for Day1 {
    fn new(problem: &str) -> Self {
        let lines = problem.lines().map(str::to_owned).collect();
        Day1 { lines: lines }
    }

    fn solve(&self) -> (Option<u32>, Option<u32>) {
        let mut part1lookup = HashMap::new();
        for i in 0..10 {
            part1lookup.insert(i.to_string(), i);
        }
        let part1 = self.lines.iter().filter_map(|line| {
            let digits = find(line, &part1lookup);
            Some(digits.first()? * 10 + digits.last()?)
        });

        let mut part2lookup = part1lookup.clone();
        part2lookup.insert("one".to_string(), 1);
        part2lookup.insert("two".to_string(), 2);
        part2lookup.insert("three".to_string(), 3);
        part2lookup.insert("four".to_string(), 4);
        part2lookup.insert("five".to_string(), 5);
        part2lookup.insert("six".to_string(), 6);
        part2lookup.insert("seven".to_string(), 7);
        part2lookup.insert("eight".to_string(), 8);
        part2lookup.insert("nine".to_string(), 9);
        let part2 = self.lines.iter().filter_map(|line| {
            let digits = find(line, &part2lookup);
            Some(digits.first()? * 10 + digits.last()?)
        });
        (Some(part1.sum()), Some(part2.sum()))
    }
}

fn find(line: &str, lookup: &HashMap<String, u32>) -> Vec<u32> {
    return line
        .char_indices()
        .filter_map(|(i, _)| {
            let (_, rest) = line.split_at(i);
            for (key, value) in lookup {
                if rest.starts_with(key) {
                    return Some(*value);
                }
            }
            None
        })
        .collect();
}

#[cfg(test)]
mod day1tests {
    use super::*;
    use advent_of_code_2023::test_example;

    #[test]
    fn test_example_part1() {
        test_example!(Day1, "day1.part1.example.txt", (Some(142), None));
    }

    #[test]
    fn test_example_part2() {
        test_example!(Day1, "day1.part2.example.txt", (None, Some(281)));
    }
}

main!(Day1, "day1.txt");
