use advent_of_code_2023::{main, Solver};

struct Day4 {
    cards: Vec<Card>,
}

struct Card {
    current: Vec<u8>,
    winners: Vec<u8>,
}

impl Card {
    fn count_winners(&self) -> u32 {
        self.current
            .iter()
            .filter(|i| self.winners.contains(i))
            .count() as u32
    }
}

impl Solver<u32> for Day4 {
    fn new(problem: &str) -> Self {
        let cards = problem
            .lines()
            .filter_map(|line| {
                let (_, line) = line.split_once(": ")?;
                let (winners, current) = line.split_once(" | ")?;
                let winners = winners
                    .split_whitespace()
                    .map(str::trim)
                    .flat_map(str::parse)
                    .collect();
                let current = current
                    .split_whitespace()
                    .map(str::trim)
                    .flat_map(str::parse)
                    .collect();

                Some(Card { current, winners })
            })
            .collect();
        Day4 { cards }
    }

    fn solve(&self) -> (Option<u32>, Option<u32>) {
        let part1 = self
            .cards
            .iter()
            .map(Card::count_winners)
            .filter(|count| *count != 0)
            .map(|count| 2_u32.pow(count - 1))
            .sum();

        let mut counts = vec![1; self.cards.len()];
        for (i, card) in self.cards.iter().enumerate() {
            for j in 0..card.count_winners() {
                counts[i + j as usize + 1] += counts[i];
            }
        }
        let part2 = counts.iter().sum();

        (Some(part1), Some(part2))
    }
}

#[cfg(test)]
mod day4tests {
    use super::*;
    use advent_of_code_2023::test_example;

    #[test]
    fn test_example_part1() {
        test_example!(Day4, "day4.example.txt", (Some(13), None));
    }

    #[test]
    fn test_example_part2() {
        test_example!(Day4, "day4.example.txt", (None, Some(30)));
    }
}

main!(Day4, "day4.txt");
