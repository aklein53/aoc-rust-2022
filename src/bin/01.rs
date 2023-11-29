pub fn part_one(input: &str) -> Option<u32> {
    //print input
    
    input.split("\n\n")
        .map(|g| {
            g.trim()
                .lines()
                .map(|l|{
                    l.parse::<u32>().unwrap()
                })
                .sum()
        }).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves: Vec<u32> =
        input.split("\n\n")
        .map(|g| {
            g.trim()
                .lines()
                .map(|l|{
                    l.parse::<u32>().unwrap()
                })
                .sum()
        }).collect();
        elves.sort_unstable();
        return Some(elves.into_iter().rev().take(3).sum());
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(71124));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(204639));
    }
}
