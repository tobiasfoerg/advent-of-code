pub fn part_one(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for y in 0..lines.len() {
        let row = &lines[y];
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::INPUT;

    const PART_ONE_EXAMPLE: &str = indoc::indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part_one() {
        let result = crate::part_one(PART_ONE_EXAMPLE);

        assert_eq!(result, 4361);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        b.iter(|| crate::part_one(&INPUT));
    }
}
