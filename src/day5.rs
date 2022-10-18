use std::num::ParseIntError;

use aoc_runner_derive::aoc;

fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input
        .lines()
        .map(|x| str::parse(x))
        .collect::<Result<_, _>>()
}

fn simulate<F>(stack: &mut [i32], f: F) -> u32
where
    F: Fn(i32) -> i32,
{
    let mut ip = 0usize;
    for i in 0u32.. {
        match stack.get_mut(ip) {
            Some(x) => {
                let val = *x;
                ip = ((ip as isize) + (val as isize)) as usize;
                *x = f(val);
            }
            None => return i,
        }
    }
    unreachable!()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> anyhow::Result<u32> {
    let mut result = parse(input)?;
    Ok(simulate(&mut result, |x| x + 1))
}

#[aoc(day5, part2)]
fn part2(input: &str) -> anyhow::Result<u32> {
    let mut result = parse(input)?;
    Ok(simulate(
        &mut result,
        |x| if x >= 3 { x - 1 } else { x + 1 },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("0\n3\n0\n1\n-3").unwrap(), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("0\n3\n0\n1\n-3").unwrap(), 10);
    }
}
