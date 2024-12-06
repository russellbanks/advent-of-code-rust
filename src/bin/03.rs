advent_of_code::solution!(3);

const MUL_START: &[u8; 4] = b"mul(";

fn parse_multiplication(line: &[u8], index: &mut usize) -> Option<(u32, u32)> {
    *index += MUL_START.len();

    let mut left = 0;
    let mut right = 0;
    let mut is_valid = true;
    let mut comma_found = false;

    while line.get(*index) != Some(&b')') {
        if line.get(*index) == Some(&b',') {
            comma_found = true;
            *index += 1;
        } else if let Some(digit) = line
            .get(*index)
            .and_then(|&b_char| char::from(b_char).to_digit(10))
        {
            if comma_found {
                right = right * 10 + digit;
            } else {
                left = left * 10 + digit;
            }
            *index += 1;
        } else {
            is_valid = false;
            break;
        }
    }

    if is_valid && comma_found {
        Some((left, right))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    for line in input.lines().map(str::as_bytes) {
        let mut index = 0;

        while index < line.len() {
            if line.get(index..index + MUL_START.len()) == Some(MUL_START) {
                if let Some((left, right)) = parse_multiplication(line, &mut index) {
                    total += left * right;
                }
            }
            index += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    const DO: &[u8; 4] = b"do()";
    const DONT: &[u8; 7] = b"don't()";

    let mut total = 0;
    let mut mul_enabled = true;

    for line in input.lines().map(str::as_bytes) {
        let mut index = 0;

        while index < line.len() {
            if line.get(index..index + DO.len()) == Some(DO) {
                mul_enabled = true;
                index += DO.len();
                continue;
            }

            if line.get(index..index + DONT.len()) == Some(DONT) {
                mul_enabled = false;
                index += DONT.len();
                continue;
            }

            if line.get(index..index + MUL_START.len()) == Some(MUL_START) {
                if let Some((left, right)) = parse_multiplication(line, &mut index) {
                    if mul_enabled {
                        total += left * right;
                    }
                }
            }
            index += 1;
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
