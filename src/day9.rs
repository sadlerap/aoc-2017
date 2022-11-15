use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::{tag, is_not},
    character::complete::{space0, char, anychar},
    combinator::map,
    multi::{fold_many0, separated_list0},
    sequence::{delimited, tuple, pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Garbage(Garbage),
    Group(Group),
}

impl Element {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(Garbage::parse, |g| Element::Garbage(g)),
            map(Group::parse, |g| Element::Group(g)),
        ))(input)
    }

    #[allow(dead_code)]
    fn garbage(contents: &str) -> Self {
        Element::Garbage(Garbage {
            contents_len: contents.len(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Group {
    elements: Vec<Element>,
}

impl Group {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("{"),
                separated_list0(tuple((tag(","), space0)), Element::parse),
                tag("}"),
            ),
            |elements| Group { elements },
        )(input)
    }

    fn score(&self, score: u32) -> u32 {
        self.elements
            .iter()
            .filter_map(|e| {
                if let Element::Group(group) = e {
                    Some(group)
                } else {
                    None
                }
            })
            .map(|group| group.score(score + 1))
            .sum::<u32>()
            + score
    }

    // clean up the garbage
    fn recycle(&self) -> usize {
        self.elements
            .iter()
            .map(|e| match e {
                Element::Garbage(garbage) => garbage.contents_len,
                Element::Group(group) => group.recycle(),
            })
            .sum()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Garbage {
    contents_len: usize,
}

impl Garbage {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("<"),
                fold_many0(
                    alt((ignore, is_not("!>"))),
                    || 0,
                    |acc, x| acc + x.len(),
                ),
                tag(">"),
            ),
            |contents| Garbage {
                contents_len: contents,
            },
        )(input)
    }
}

fn ignore(input: &str) -> IResult<&str, &str> {
    map(pair(char('!'), anychar), |_| "")(input)
}

#[aoc_generator(day9)]
fn generate(input: &str) -> anyhow::Result<Group> {
    Group::parse(input)
        .map(|(_, x)| x)
        .map_err(|_| anyhow!("Failed to parse group!"))
}

#[aoc(day9, part1)]
fn part1(input: &Group) -> u32 {
    input.score(1)
}

#[aoc(day9, part2)]
fn part2(input: &Group) -> usize {
    input.recycle()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore() {
        assert_eq!(ignore("!!"), Ok(("", "")))
    }

    macro_rules! garbage {
        ( $(($input:expr, $expected:expr)),* ) => {
            $({
                assert_eq!(Garbage::parse($input), Ok(("", Garbage{ contents_len: $expected.len() })));
            })*
        };
    }

    #[test]
    fn test_garbage_parse() {
        garbage! {
            ("<>", ""),
            ("<random characters>", "random characters"),
            ("<<<<>", "<<<"),
            ("<{!>}>", "{}"),
            ("<!!>", ""),
            ("<!!!>>", ""),
            ("<{o\"i!a,<{i<a>", "{o\"i,<{i<a")
        };
    }

    #[test]
    fn test_group_parse() {
        assert_eq!(Group::parse("{}").unwrap().1, Group::default());
        assert_eq!(
            Group::parse("{{}}").unwrap().1,
            Group {
                elements: vec![Element::Group(Group::default())]
            }
        );
        assert_eq!(
            Group::parse("{{}, {}}").unwrap().1,
            Group {
                elements: vec![
                    Element::Group(Group::default()),
                    Element::Group(Group::default())
                ]
            }
        );
        assert_eq!(
            Group::parse("{<a>,<a>,<a>,<a>}").unwrap().1,
            Group {
                elements: vec![
                    Element::garbage("a"),
                    Element::garbage("a"),
                    Element::garbage("a"),
                    Element::garbage("a"),
                ]
            }
        );
        assert_eq!(
            Group::parse("{{<a!>},{<a!>},{<a!>},{<ab>}}").unwrap().1,
            Group {
                elements: vec![Element::Group(Group {
                    elements: vec![Element::garbage("a},{<a},{<a},{<ab")]
                })]
            }
        );
        assert_eq!(
            Group::parse("{{<ab>},{<ab>},{<ab>},{<ab>}}").unwrap().1,
            Group {
                elements: vec![
                    Element::Group(Group {
                        elements: vec![Element::garbage("ab")]
                    }),
                    Element::Group(Group {
                        elements: vec![Element::garbage("ab")]
                    }),
                    Element::Group(Group {
                        elements: vec![Element::garbage("ab")]
                    }),
                    Element::Group(Group {
                        elements: vec![Element::garbage("ab")]
                    }),
                ]
            }
        );
    }

    macro_rules! score {
        ( $(($input:expr, $expected:expr)),* ) => {
            $({
                let group = Group::parse($input).unwrap().1;
                assert_eq!(group.score(1), $expected);
            })*
        };
    }

    #[test]
    fn score_test() {
        score! {
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3)
        };
    }
}
