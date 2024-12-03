advent_of_code::solution!(2);

fn read_file_to_int_slices(input: &str) -> Vec<Vec<i32>> {
    let mut lines: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        lines.push(numbers);
    }
    lines
}

fn in_range(i: i32, j: i32) -> bool {
    let diff = (i - j).abs();
    1 <= diff && diff <= 3
}

fn line_safe(line: &Vec<i32>) -> bool {
    let ascending = line[0] < line[1];
    for w in line.windows(2) {
        if !in_range(w[0], w[1]) {
            return false;
        }
        if (ascending && w[1] < w[0]) ||
            (!ascending && w[1] > w[0]) {
            return false;
        }
    }
    true
}

fn permissive_line_safe(line: &Vec<i32>) -> bool {
    for i in 0..line.len() {
        let mut new_line = Vec::new();
        new_line.extend_from_slice(&line[..i]);
        new_line.extend_from_slice(&line[i + 1..]);
        if line_safe(&new_line) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = read_file_to_int_slices(input);
    let mut safe_lines = 0;
    for line in lines.iter() {
        if line_safe(line) {
            safe_lines += 1;
        }
    }

    Some(safe_lines)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = read_file_to_int_slices(input);
    let mut safe_lines = 0;
    for line in lines.iter() {
        if permissive_line_safe(line) {
            safe_lines += 1;
        }
    }
    Some(safe_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
