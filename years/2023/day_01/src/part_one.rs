pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = (0..line.len()).filter_map(|i| match line.chars().nth(i) {
                Some(c) if c.is_ascii_digit() => Some(c as usize - '0' as usize),
                _ => None,
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

    const PART_ONE_EXAMPLE: &str = indoc::indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn part_one() {
        let result = crate::part_one(PART_ONE_EXAMPLE);

        assert_eq!(result, 142);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        b.iter(|| crate::part_one(&INPUT));
    }
}
