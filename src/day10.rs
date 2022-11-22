use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};
use std::fmt::Write;

#[aoc_generator(day10, part1)]
fn generator(input: &str) -> anyhow::Result<Vec<u8>> {
    let res: IResult<&str, Vec<u8>> = separated_list1(tag(","), complete::u8)(input);
    res.map(|x| x.1)
        .map_err(|_| anyhow::anyhow!("Failed to parse input!"))
}

#[derive(PartialEq, Eq, Debug)]
struct Hasher<const N: usize> {
    data: [u8; N],
    skip_size: usize,
    current_pos: usize,
}

impl<const N: usize> Default for Hasher<N> {
    fn default() -> Self {
        Hasher {
            data: std::array::from_fn(|i| i as u8),
            skip_size: 0,
            current_pos: 0,
        }
    }
}

impl<const N: usize> Hasher<N> {
    fn new() -> Self {
        Default::default()
    }

    fn write(&mut self, length: usize) {
        (0..length)
            .rev()
            .enumerate()
            .take_while(|(x, y)| x <= y)
            .map(|(x, y)| ((x + self.current_pos) % N, (y + self.current_pos) % N))
            .for_each(|(x, y)| self.data.swap(x, y));

        self.current_pos = (self.current_pos + length + self.skip_size) % N;
        self.skip_size = (self.skip_size + 1) % N;
    }

    fn finish(&self) -> &[u8; N] {
        &self.data
    }
}

#[aoc(day10, part1)]
fn part1(input: &[u8]) -> anyhow::Result<u32> {
    let mut hasher = Hasher::new();
    input.iter().for_each(|i| hasher.write(*i as usize));
    let data: &[u8; 256] = hasher.finish();
    Ok(data[0] as u32 * data[1] as u32)
}

#[aoc(day10, part2)]
fn part2(input: &[u8]) -> anyhow::Result<String> {
    let mut hasher: Hasher<256> = Hasher::new();
    for _ in 0..64 {
        input.iter()
            .chain([17, 31, 73, 47, 23].iter())
            .for_each(|i| hasher.write(*i as usize));
    }

    let sparse_hash = hasher.finish();
    let mut buffer = String::new();
    for byte in sparse_hash
        .chunks(16)
        .map(|x| {
            x.iter()
                .fold(0, |acc, b| acc ^ b)
        })
    {
        write!(buffer, "{:02x}", byte)?;
    }

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_data() {
        let mut hasher = Hasher::<5>::new();
        hasher.write(3);
        assert_eq!(
            hasher,
            Hasher {
                data: [2, 1, 0, 3, 4],
                current_pos: 3,
                skip_size: 1
            }
        );

        hasher.write(4);
        assert_eq!(
            hasher,
            Hasher {
                data: [4, 3, 0, 1, 2],
                current_pos: 3,
                skip_size: 2
            }
        );

        hasher.write(1);
        assert_eq!(
            hasher,
            Hasher {
                data: [4, 3, 0, 1, 2],
                current_pos: 1,
                skip_size: 3
            }
        );

        hasher.write(5);
        assert_eq!(
            hasher,
            Hasher {
                data: [3, 4, 2, 1, 0],
                current_pos: 4,
                skip_size: 4
            }
        );
    }

    macro_rules! knot_hash {
        ($x:expr, $res:expr) => {
            assert_eq!(part2($x).unwrap(), $res)
        };
    }

    #[test]
    fn test_knot_hash() {
        knot_hash!(b"", "a2582a3a0e66e6e86e3812dcb672a272");
        knot_hash!(b"AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd");
        knot_hash!(b"1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d");
        knot_hash!(b"1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
