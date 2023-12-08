use crate::parse_game_to_sets;

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .filter_map(|(game_index, game)| {
            parse_game_to_sets(game)
                .into_iter()
                .all(|(red, blue, green)| red <= 12 && blue <= 14 && green <= 13)
                .then_some(game_index + 1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::INPUT;

    const PART_ONE_EXAMPLE: &str = indoc::indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part_one() {
        let result = crate::part_one(PART_ONE_EXAMPLE);

        assert_eq!(result, 8);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        b.iter(|| crate::part_one(&INPUT));
    }
}
