use itertools::Itertools;

advent_of_code::solution!(2);

fn is_report_safe<I>(report: I) -> bool
where
    I: IntoIterator<Item = u8>,
{
    let mut all_ascending = true;
    let mut all_descending = true;

    let valid_adjacent_differ = report.into_iter().tuple_windows().all(|(a, b)| {
        all_ascending &= a < b;
        all_descending &= a > b;
        (1..=3).contains(&(a.abs_diff(b)))
    });

    valid_adjacent_differ && (all_ascending || all_descending)
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|line| is_report_safe(line.split_ascii_whitespace().flat_map(str::parse)))
        .count();

    u32::try_from(count).ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|line| {
            let report = line.split_ascii_whitespace().flat_map(str::parse);

            if is_report_safe(report.clone()) {
                return true;
            }

            let report = report.collect::<Vec<_>>();
            (0..report.len()).any(|index| {
                let modified_report =
                    report.iter().enumerate().filter_map(|(sub_index, &level)| {
                        if index == sub_index {
                            None
                        } else {
                            Some(level)
                        }
                    });

                is_report_safe(modified_report)
            })
        })
        .count();

    u32::try_from(count).ok()
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
