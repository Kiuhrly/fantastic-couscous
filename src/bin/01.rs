pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n") // Split per elf
        .map(|str| {
            str.split('\n') // Split into individual calorie values
                .map(|str| str.parse::<u32>().unwrap()) // Parse strings as u32
                .sum()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counts: Vec<_> = input
        .split("\n\n")
        .map(|str| str.split('\n').map(|str| str.parse::<u32>().unwrap()).sum())
        .collect();
    counts.sort();
    Some(counts.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
