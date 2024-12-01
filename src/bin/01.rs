use std::collections::HashMap;

advent_of_code::solution!(1);

fn get_sorted_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();

    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let l: i32 = parts[0].parse().unwrap();
            let r: i32 = parts[1].parse().unwrap();
            left.push(l);
            right.push(r);
        }
    }
    left.sort();
    right.sort();

    // Return the sorted columns
    (left, right)
}

fn get_similarlity(left: &Vec<i32>, right: &Vec<i32>) -> HashMap<i32, i32> {
    let mut similarity = HashMap::new();

    for l in left {
        let frequency = right.iter().filter(|&&r | r  == *l).count();
        similarity.insert(*l, frequency as i32);
    }
    similarity
}
pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = get_sorted_columns(input);
    let mut result: u32 = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        result = result + (l - r).abs() as u32;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = get_sorted_columns(input);
    let similarity = get_similarlity(&left, &right);
    let mut result: u32 = 0;
    for l in left.iter() {
        result = result + (l * similarity[&l]) as u32;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
