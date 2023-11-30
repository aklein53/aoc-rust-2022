pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.lines().map(|line: &str| {
        let opp = (line.chars().nth(0).unwrap() as u8) - ('A' as u8);
        let mine = (line.chars().nth(2).unwrap() as u8) - ('X' as u8);
        let mut score = 4 + mine;
        if (opp + 1) % 3 == mine
        {
            score += 3;
        }
        else if (mine + 1) % 3 == opp
        {
            score -= 3;
        }
        return score as u32;
    }).sum();

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input.lines().map(|line: &str| {
        let opp = (line.chars().nth(0).unwrap() as u8) - ('A' as u8);
        let round_result = (line.chars().nth(2).unwrap() as u8) - ('X' as u8);
        let mut score = 0;
        match round_result {
            0 => score += 1 + ((opp - 1) % 3),
            1 => score += 4 + opp,
            2 => score += 7 + ((opp + 1) % 3),
            _ => panic!("Invalid round result"),
        }
        return score as u32;
    }).sum();

    return Some(sum);
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(12));
    }
}
