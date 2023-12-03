use advent_of_code_2023::{main, Solver};

struct Day3 {
    // grid is just all the characters
    grid: Vec<Vec<char>>,

    // nums has the integer representation of each number at the start of each
    // number
    nums: Vec<Vec<u32>>,

    // numlens has the length of each number at each point in the number, so we
    // can find the start of the number to look up in nums
    numlens: Vec<Vec<u8>>,
}

impl Solver<u32> for Day3 {
    fn new(problem: &str) -> Self {
        let grid: Vec<Vec<char>> = problem.lines().map(|line| line.chars().collect()).collect();
        let numlens: Vec<Vec<u8>> = grid
            .iter()
            .map(|line| {
                let mut v = vec![];
                for ch in line {
                    v.push(if ch.is_digit(10) {
                        v.last().unwrap_or(&0) + 1
                    } else {
                        0
                    });
                }
                v
            })
            .collect();
        let nums = grid
            .iter()
            .enumerate()
            .map(|(y, line)| {
                let mut v = vec![0; line.len()];
                for (x, ch) in line.iter().enumerate() {
                    let len = numlens[y][x];
                    if len > 0 {
                        v[(x as isize - len as isize + 1) as usize] *= 10;
                        v[(x as isize - len as isize + 1) as usize] += ch.to_digit(10).unwrap();
                    }
                }
                v
            })
            .collect();
        Day3 {
            grid,
            nums,
            numlens,
        }
    }

    fn solve(&self) -> (Option<u32>, Option<u32>) {
        let mut part_numbers = vec![];
        for (y, line) in self.grid.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if ch.is_digit(10) || *ch == '.' {
                    continue;
                }

                let mut neighbors = self.neighbors((y, x));
                part_numbers.append(&mut neighbors);
            }
        }
        part_numbers.sort();
        part_numbers.dedup();
        let part1 = part_numbers.iter().map(|(y, x)| self.nums[*y][*x]).sum();

        let part2 = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().map(move |(x, ch)| {
                    if *ch != '*' {
                        return 0;
                    }
                    let gears = self.neighbors((y, x));
                    if gears.len() == 2 {
                        self.nums[gears[0].0][gears[0].1] * self.nums[gears[1].0][gears[1].1]
                    } else {
                        0
                    }
                })
            })
            .sum();

        (Some(part1), Some(part2))
    }
}

impl Day3 {
    fn neighbors(&self, (y, x): (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let (x, y) = (x as isize + i, y as isize + j);
                if y < 0
                    || x < 0
                    || y >= self.grid.len() as isize
                    || x >= self.grid[0].len() as isize
                {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);

                let len = self.numlens[y][x];
                if len > 0 {
                    result.push((y, (x as isize - len as isize + 1) as usize));
                }
            }
        }
        result.sort();
        result.dedup();
        result
    }
}

#[cfg(test)]
mod day3tests {
    use super::*;
    use advent_of_code_2023::test_example;

    #[test]
    fn test_example_part1() {
        test_example!(Day3, "day3.example.txt", (Some(4361), None));
    }

    #[test]
    fn test_example_part2() {
        test_example!(Day3, "day3.example.txt", (None, Some(467835)));
    }
}

main!(Day3, "day3.txt");
