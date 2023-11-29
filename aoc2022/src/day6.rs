use itertools::Itertools;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    solve(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    solve(input, 14)
}

#[aoc(day6, part2, iterator)]
fn part2_iterator(input: &str) -> usize {
    solve_slow(input, 14)
}

fn solve(input: &str, window_size: usize) -> usize {
    let mut state: Vec<u8> = vec![0; 26];
    let input = input.as_bytes();
    for i in 0..window_size {
        state[(input[i] - 97) as usize] += 1;
    }

    for i in 0..input.len() - window_size {
        state[(input[i] - 97) as usize] -= 1;
        state[(input[i + window_size] - 97) as usize] += 1;

        if state.iter().filter(|&&s| s > 0).count() == window_size {
            return i + window_size + 1;
        }
    }

    panic!("No solution")
}

fn solve_slow(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find_map(|(i, values)| {
            if values.iter().unique().count() == window_size {
                Some(i + window_size)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn p2() {
        assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
