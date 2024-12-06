advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    const DIRECTIONS: [(isize, isize); 8] = [
        (1, 0),
        (0, 1),
        (1, 1),
        (1, -1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
    ];

    const WORD: &[u8; 4] = b"XMAS";

    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for (direction_x, direction_y) in DIRECTIONS {
                let mut found = true;
                for xmas_index in 0..WORD.len() as isize {
                    let x = row + xmas_index * direction_x;
                    let y = col + xmas_index * direction_y;
                    if x < 0 || y < 0 || x >= rows || y >= cols {
                        found = false;
                        break;
                    }
                    if grid[x as usize][y as usize] != WORD[xmas_index as usize] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    const M_S_DIFF: u8 = b'M'.abs_diff(b'S');

    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for x in 1..rows - 1 {
        for y in 1..cols - 1 {
            if grid[x][y] == b'A' {
                let upper_left = grid[x.saturating_sub(1)][y.saturating_sub(1)];
                let upper_right = grid[x.saturating_add(1)][y.saturating_sub(1)];
                let bottom_left = grid[x.saturating_sub(1)][y.saturating_add(1)];
                let bottom_right = grid[x.saturating_add(1)][y.saturating_add(1)];

                count += (upper_left.abs_diff(bottom_right) == M_S_DIFF
                    && upper_right.abs_diff(bottom_left) == M_S_DIFF)
                    as u32
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
