use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    character::complete::{digit1, newline, space1},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Row {
    data: Vec<u32>,
}

impl Row {
    fn parse<'a>(input: &'a str) -> IResult<&'a str, Self> {
        map(
            terminated(
                separated_list1(space1, map(digit1, |s| str::parse::<u32>(s).unwrap())),
                opt(newline),
            ),
            |data| Row { data },
        )(input)
    }

    fn matching_pair(&self) -> Option<u32> {
        for (i, n) in self.data.iter().enumerate() {
            for m in self.data[(i+1)..].iter() {
                if n % m == 0 {
                    return Some(n / m);
                } else if m % n == 0 {
                    return Some(m / n);
                }
            }
        }
        dbg!("no matching pairs found");
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Spreadsheet {
    rows: Vec<Row>,
}

impl Spreadsheet {
    fn parse<'a>(input: &'a str) -> IResult<&str, Spreadsheet> {
        map(many1(terminated(Row::parse, opt(newline))), |rows| {
            Spreadsheet { rows }
        })(input)
    }

    fn new(input: &str) -> anyhow::Result<Spreadsheet> {
        match Spreadsheet::parse(input) {
            Ok((_, spreadsheet)) => Ok(spreadsheet),
            Err(_) => Err(anyhow!("Unable to parse spreadsheet")),
        }
    }

    fn checksum(&self) -> u32 {
        self.rows
            .iter()
            .map(|row| match row.data.iter().minmax() {
                itertools::MinMaxResult::NoElements => unreachable!(),
                itertools::MinMaxResult::OneElement(_) => 0,
                itertools::MinMaxResult::MinMax(min, max) => max - min,
            })
            .sum()
    }

    fn even_divisors(&self) -> u32 {
        self.rows.iter().filter_map(|row| row.matching_pair()).sum()
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> anyhow::Result<Spreadsheet> {
    Spreadsheet::new(input)
}

#[aoc(day2, part1)]
fn part_1(input: &Spreadsheet) -> u32 {
    input.checksum()
}

#[aoc(day2, part2)]
fn part_2(input: &Spreadsheet) -> u32 {
    input.even_divisors()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_row() {
        let input = "1 2 3 4";
        assert_eq!(
            Row::parse(input).unwrap(),
            (
                "",
                Row {
                    data: vec![1, 2, 3, 4]
                }
            )
        );
    }

    #[test]
    fn test_parse_spreadsheet() {
        let input = "1 2 3 4\n5 6 7 8\n";
        assert_eq!(
            Spreadsheet::parse(input).unwrap(),
            (
                "",
                Spreadsheet {
                    rows: vec![
                        Row {
                            data: vec![1, 2, 3, 4]
                        },
                        Row {
                            data: vec![5, 6, 7, 8]
                        },
                    ]
                }
            )
        )
    }
}
