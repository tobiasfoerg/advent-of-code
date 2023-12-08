use crate::parse_game_to_sets;

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|game| {
            let (red, blue, green) = parse_game_to_sets(game).into_iter().fold(
                (0, 0, 0),
                |(mut max_red, mut max_blue, mut max_green), (red, green, blue)| {
                    max_red = max_red.max(red);
                    max_blue = max_blue.max(blue);
                    max_green = max_green.max(green);
                    (max_red, max_blue, max_green)
                },
            );
            red * blue * green
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::INPUT;

    const PART_TWO_EXAMPLE: &str = indoc::indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
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
