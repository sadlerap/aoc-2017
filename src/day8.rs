use anyhow::anyhow;
use aoc_runner_derive::aoc;
use fxhash::FxHashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space0, space1},
    combinator::map,
    sequence::tuple,
    IResult,
};

#[derive(PartialEq, Eq, Debug)]
struct Command<'input> {
    dest_reg: &'input str,
    dest_num: i32,

    test_reg: &'input str,
    test_num: i32,
    test_type: Test,
}

#[derive(PartialEq, Eq, Debug)]
enum Test {
    LessThan,
    LessThanEqual,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
}

enum Op {
    Increment,
    Decrement,
}

impl<'input> Command<'input> {
    fn parse(line: &'input str) -> anyhow::Result<Self> {
        let res: IResult<_, Command> = map(
            tuple((
                alpha1,
                space1,
                map(
                    tuple((
                        alt((
                            map(tag("inc"), |_| Op::Increment),
                            map(tag("dec"), |_| Op::Decrement),
                        )),
                        space1,
                        nom::character::complete::i32,
                    )),
                    |(op, _, num)| match op {
                        Op::Decrement => -num,
                        Op::Increment => num,
                    },
                ),
                space1,
                tag("if"),
                space1,
                alpha1,
                space0,
                alt((
                    map(tag("<="), |_| Test::LessThanEqual),
                    map(tag("<"), |_| Test::LessThan),
                    map(tag("=="), |_| Test::Equal),
                    map(tag("!="), |_| Test::NotEqual),
                    map(tag(">="), |_| Test::GreaterThanEqual),
                    map(tag(">"), |_| Test::GreaterThan),
                )),
                space0,
                nom::character::complete::i32,
            )),
            |(dest_reg, _, dest_num, _, _, _, test_reg, _, test_type, _, test_num)| Command {
                dest_reg,
                dest_num,
                test_reg,
                test_num,
                test_type,
            },
        )(line);

        res.map(|(_, c)| c)
            .map_err(|e| anyhow!("failed to parse input: {e}"))
    }
}

#[derive(Default)]
struct Cpu<'reg> {
    registers: FxHashMap<&'reg str, i32>,
    max_seen_value: i32,
}

impl<'reg> Cpu<'reg> {
    fn process_command(&mut self, command: &Command<'reg>) {
        let test_reg_value = self.get_register_value(command.test_reg);
        let test_result = match command.test_type {
            Test::LessThan => test_reg_value < command.test_num,
            Test::LessThanEqual => test_reg_value <= command.test_num,
            Test::Equal => test_reg_value == command.test_num,
            Test::NotEqual => test_reg_value != command.test_num,
            Test::GreaterThan => test_reg_value > command.test_num,
            Test::GreaterThanEqual => test_reg_value >= command.test_num,
        };
        if test_result {
            let value = self.get_register_value(command.dest_reg);
            self.set_register_value(command.dest_reg, value + command.dest_num);
        }
    }

    fn get_register_value(&mut self, reg: &'reg str) -> i32 {
        if let Some(num) = self.registers.get(reg) {
            *num
        } else {
            self.registers.insert(reg, 0);
            0
        }
    }

    fn set_register_value(&mut self, reg: &'reg str, value: i32) {
        self.registers.insert(reg, value);
        self.max_seen_value = self.max_seen_value.max(value);
    }

    fn max_reg_value(&self) -> i32 {
        self.registers.values().max().cloned().unwrap_or_default()
    }

    fn max_seen_value(&self) -> i32 {
        self.max_seen_value
    }
}

#[aoc(day8, part1)]
fn part1(input: &str) -> anyhow::Result<i32> {
    let mut processor = Cpu::default();
    for line in input.lines() {
        let command = Command::parse(line)?;
        processor.process_command(&command);
    }
    Ok(processor.max_reg_value())
}

#[aoc(day8, part2)]
fn part2(input: &str) -> anyhow::Result<i32> {
    let mut processor = Cpu::default();
    for line in input.lines() {
        let command = Command::parse(line)?;
        processor.process_command(&command);
    }
    Ok(processor.max_seen_value())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_command() {
        let input = "b inc 5 if a > 1";
        let command = Command {
            dest_reg: "b",
            dest_num: 5,
            test_reg: "a",
            test_num: 1,
            test_type: Test::GreaterThan,
        };
        assert_eq!(Command::parse(input).unwrap(), command);
    }

    #[test]
    fn parse_command2() {
        let input = "c dec -10 if a >= 1";
        let command = Command {
            dest_reg: "c",
            dest_num: 10,
            test_reg: "a",
            test_num: 1,
            test_type: Test::GreaterThanEqual,
        };
        assert_eq!(Command::parse(input).unwrap(), command);
    }

    #[test]
    fn given_input_part1() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(part1(input).unwrap(), 1);
    }

    #[test]
    fn given_input_part2() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(part2(input).unwrap(), 10);
    }
}
