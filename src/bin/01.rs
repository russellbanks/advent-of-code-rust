advent_of_code::solution!(1);

fn create_left_right_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (left_list, right_list) = input
        .lines()
        .map(str::split_ascii_whitespace)
        .filter_map(|mut parts| parts.next().zip(parts.next()))
        .filter_map(|(left, right)| left.parse::<u32>().ok().zip(right.parse::<u32>().ok()))
        .fold(
            (Vec::new(), Vec::new()),
            |(mut left_list, mut right_list), (left, right)| {
                left_list.push(left);
                right_list.push(right);
                (left_list, right_list)
            },
        );

    (left_list, right_list)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = create_left_right_lists(input);

    left_list.sort_unstable();
    right_list.sort_unstable();

    let sum = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, &right)| left.abs_diff(right))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = create_left_right_lists(input);

    let similarity_score = left_list
        .iter()
        .map(|left| {
            left * right_list
                .iter()
                .copied()
                .filter(|right| left == right)
                .count() as u32
        })
        .sum();

    Some(similarity_score)
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
