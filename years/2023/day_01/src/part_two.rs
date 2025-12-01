const STRING_DIGITS: [&str; 9] =
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = (0..line.len()).filter_map(|i| match line.chars().nth(i) {
                Some(c) if c.is_ascii_digit() => Some(c as usize - '0' as usize),
                _ => STRING_DIGITS.iter().enumerate().find_map(|(digit_index, digit)| {
                    line[i..].starts_with(digit).then_some(digit_index + 1)
                }),
            });
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);

            (first * 10) as u32 + last as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::INPUT;

    const PART_TWO_EXAMPLE: &str = indoc::indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn part_two() {
        let result = crate::part_two(PART_TWO_EXAMPLE);

        assert_eq!(result, 281);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        b.iter(|| crate::part_two(&INPUT));
    }
}
