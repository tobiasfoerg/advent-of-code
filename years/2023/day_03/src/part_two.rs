
pub fn part_two(input: &str) -> usize {
   todo!()
}

#[cfg(test)]
mod tests {
    use crate::INPUT;

    const PART_TWO_EXAMPLE: &str = indoc::indoc! {"

    "};

    #[test]
    fn part_two() {
        let result = crate::part_two(PART_TWO_EXAMPLE);

        assert_eq!(result, 2286);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        b.iter(|| crate::part_two(&INPUT));
    }
}
