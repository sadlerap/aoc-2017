use std::collections::{BTreeMap, BTreeSet};

use anyhow::anyhow;
use aoc_runner_derive::aoc;
use nom::{
    character::complete::{alpha1, newline, space1},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub struct Passphrase<'a> {
    data: Vec<&'a str>,
}

impl Passphrase<'_> {
    fn part1_valid(&self) -> bool {
        let mut words = BTreeSet::new();
        for word in self.data.iter() {
            if words.contains(word) {
                return false;
            }
            words.insert(word);
        }
        true
    }

    fn part2_valid(&self) -> bool {
        let mut character_frequencies = BTreeSet::new();
        for word in self.data.iter() {
            let mut character_freq = BTreeMap::new();
            for c in word.chars() {
                character_freq.entry(c).and_modify(|x| *x += 1).or_insert(1);
            }
            if character_frequencies.contains(&character_freq) {
                return false;
            }
            character_frequencies.insert(character_freq);
        }

        true
    }
}

pub fn generator(input: &str) -> anyhow::Result<Vec<Passphrase>> {
    let result: IResult<&str, Vec<Passphrase>> = many1(terminated(
        map(separated_list1(space1, alpha1), |data| Passphrase { data }),
        opt(newline),
    ))(input);

    result
        .map_err(|_| anyhow!("Unable to parse input!"))
        .map(|(_, x)| x)
}

#[aoc(day4, part1)]
fn part1(input: &str) -> anyhow::Result<u64> {
    let input = generator(input)?;
    Ok(input.iter().filter(|p| p.part1_valid()).count() as u64)
}

#[aoc(day4, part2)]
fn part2(input: &str) -> anyhow::Result<u64> {
    let input = generator(input)?;
    Ok(input.iter().filter(|p| p.part2_valid()).count() as u64)
}
