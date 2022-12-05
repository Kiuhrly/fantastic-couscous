#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,   
}

impl Shape {
    fn points_for_game(&self, opponent: &Shape) -> u32 {
        // Draw
        if *self == *opponent {
            return *self as u32 + 3
        }
        
        // Winning conditions
        if (*self == Shape::Rock && *opponent == Shape::Scissors) ||
            (*self == Shape::Paper && *opponent == Shape::Rock) ||
            (*self == Shape::Scissors && *opponent == Shape::Paper)
            {
            return *self as u32 + 6
        }

        // Losing conditions
        if (*self == Shape::Rock && *opponent == Shape::Paper) ||
            (*self == Shape::Paper && *opponent == Shape::Scissors) ||
            (*self == Shape::Scissors && *opponent == Shape::Rock)
            {
            return *self as u32
        }
        unreachable!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_score: u32 = 0;
    let games: Vec<_> = input.lines().collect();
    for game in games {
        let opposing_shape = match game.split(' ').nth(0).unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => unreachable!()
        };

        let your_shape = match game.split(' ').nth(1).unwrap() {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => unreachable!()
        };
        
        total_score += your_shape.points_for_game(&opposing_shape);
    }
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score: u32 = 0;
    let games: Vec<_> = input.lines().collect();
    for game in games {
        let opposing_shape = match game.split(' ').nth(0).unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => unreachable!()
        };

        let your_shape = match game.split(' ').nth(1).unwrap() {
            // Lose
            "X" => {
                match opposing_shape {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                }
            },
            // Draw
            "Y" => opposing_shape,
            // Win
            "Z" => {
                match opposing_shape {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                }
            },
            _ => unreachable!()
        };
        
        total_score += your_shape.points_for_game(&opposing_shape);
    }
    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
