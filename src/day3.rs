use std::num::NonZeroU32;

use aoc_runner_derive::aoc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Position(i32, i32);

impl Position {
    fn find(num: i32) -> Position {
        let (low_root, mid_root, high_root) = {
            let sqrt = ((num as f32).sqrt()) as i32;
            if sqrt & 1 == 0 {
                (sqrt - 1, sqrt, sqrt + 1)
            } else if sqrt * sqrt == num {
                (sqrt - 2, sqrt - 1, sqrt)
            } else {
                (sqrt, sqrt + 1, sqrt + 2)
            }
        };

        let (low, mid, high) = (
            low_root * low_root,
            mid_root * mid_root,
            high_root * high_root,
        );
        let c1 = ((low + mid) / 2) + 1;
        let c2 = mid + 1;
        let c3 = (mid + high + 1) / 2;
        let c4 = high;

        let ring_distance = mid_root / 2;

        //figure out which corners `num` is between
        if ((low + 1)..=c1).contains(&num) {
            let midpoint = (low + c1) / 2;
            Position(ring_distance, num - midpoint)
        } else if ((c1 + 1)..=c2).contains(&num) {
            let midpoint = (c1 + c2) / 2;
            Position(midpoint - num, ring_distance)
        } else if ((c2 + 1)..=c3).contains(&num) {
            let midpoint = (c2 + c3) / 2;
            Position(-ring_distance, midpoint - num)
        } else {
            let midpoint = (c3 + c4) / 2;
            Position(num - midpoint, -ring_distance)
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    fn num(&self) -> i32 {
        if self.0 == 0 && self.1 == 0 {
            return 1;
        }

        let ring = self.0.abs().max(self.1.abs());
        let (low_root, mid_root, high_root) = (2 * ring - 1, 2 * ring, 2 * ring + 1);
        let (low, mid, high) = (
            low_root * low_root,
            mid_root * mid_root,
            high_root * high_root,
        );
        let c1 = ((low + mid) / 2) + 1;
        let c2 = mid + 1;
        let c3 = (mid + high + 1) / 2;
        let c4 = high;

        // if we're at a corner, exit early
        if ring == self.0 && ring == self.1 {
            return c1;
        } else if ring == -self.0 && ring == self.1 {
            return c2;
        } else if ring == -self.0 && ring == -self.1 {
            return c3;
        } else if ring == self.0 && ring == -self.1 {
            return c4;
        }

        // figure out which corners `num` is between
        if ring == self.0 {
            let midpoint = (low + c1) / 2;
            self.1 + midpoint
        } else if ring == self.1 {
            let midpoint = (c1 + c2) / 2;
            -self.0 + midpoint
        } else if ring == -self.0 {
            let midpoint = (c2 + c3) / 2;
            -self.1 + midpoint
        } else {
            let midpoint = (c3 + c4) / 2;
            self.0 + midpoint
        }
    }
}

#[derive(Debug)]
struct Memory {
    data: Vec<Option<NonZeroU32>>,
}

impl Memory {
    fn new_filled(size: usize) -> Memory {
        let mut memory = Memory {
            data: vec![None; size],
        };
        memory.set(1, NonZeroU32::new(1).unwrap());
        memory
    }

    fn get(&self, index: usize) -> Option<&Option<NonZeroU32>> {
        self.data.get(index)
    }

    fn set(&mut self, index: usize, value: NonZeroU32) {
        *self.data.get_mut(index).unwrap() = Some(value);
    }

    fn get_pos(&self, pos: Position) -> Option<&Option<NonZeroU32>> {
        let index = pos.num() as usize;
        self.get(index)
    }

    fn value_at_pos(&self, pos: &Position) -> u32 {
        pos.num();
        ([-1, 0, 1])
            .iter()
            .flat_map(|&dx| {
                [-1, 0, 1]
                    .iter()
                    .filter(move |&dy| !(dx == 0 && *dy == 0))
                    .map(move |&dy| Position(pos.0 + dx, pos.1 + dy))
            })
            .filter_map(|p| {
                if let Some(value) = self.get_pos(p) {
                    *value
                } else {
                    None
                }
            })
            .map(u32::from)
            .fold(0, std::ops::Add::add)
    }
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let num = input
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("unable to parse input {input}"));
    let position = Position::find(num);
    position.manhattan_distance()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let num = input
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("unable to parse input {input}"));
    let mut memory = Memory::new_filled(num as usize);
    (2..num)
        .map(|index| {
            let pos = Position::find(index as i32);
            let value = memory.value_at_pos(&pos);
            memory.set(index as usize, NonZeroU32::new(value).unwrap());
            value
        })
        .find(|&value| value > num)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(part1("25"), 4);
        assert_eq!(part1("22"), 3);
        assert_eq!(part1("11"), 2);
        assert_eq!(part1("14"), 3);
        assert_eq!(part1("1024"), 31);
    }

    #[test]
    fn position_test() {
        assert_eq!(Position::find(25), Position(2, -2));
        assert_eq!(Position::find(11), Position(2, 0));
        assert_eq!(Position::find(16), Position(-1, 2));
        assert_eq!(Position::find(14), Position(1, 2));
        assert_eq!(Position::find(18), Position(-2, 1));
    }

    #[test]
    fn position_as_num() {
        assert_eq!(Position(0, 0).num(), 1);
        assert_eq!(Position(1, 0).num(), 2);
    }

    #[test]
    fn memory_test() {
        let mut memory = Memory::new_filled(1024);

        memory.set(1, NonZeroU32::new(1).unwrap());
        assert_eq!(u32::from(memory.get(1).unwrap().unwrap()), 1_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(0, 0)).unwrap().unwrap()),
            1_u32
        );

        memory.set(2, NonZeroU32::new(1).unwrap());
        assert_eq!(u32::from(memory.get(2).unwrap().unwrap()), 1_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(1, 0)).unwrap().unwrap()),
            1_u32
        );

        memory.set(3, NonZeroU32::new(2).unwrap());
        assert_eq!(u32::from(memory.get(3).unwrap().unwrap()), 2_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(1, 1)).unwrap().unwrap()),
            2_u32
        );

        memory.set(4, NonZeroU32::new(4).unwrap());
        assert_eq!(u32::from(memory.get(4).unwrap().unwrap()), 4_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(0, 1)).unwrap().unwrap()),
            4_u32
        );

        memory.set(5, NonZeroU32::new(5).unwrap());
        assert_eq!(u32::from(memory.get(5).unwrap().unwrap()), 5_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(-1, 1)).unwrap().unwrap()),
            5_u32
        );

        memory.set(6, NonZeroU32::new(10).unwrap());
        assert_eq!(u32::from(memory.get(6).unwrap().unwrap()), 10_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(-1, 0)).unwrap().unwrap()),
            10_u32
        );

        memory.set(7, NonZeroU32::new(11).unwrap());
        assert_eq!(u32::from(memory.get(7).unwrap().unwrap()), 11_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(-1, -1)).unwrap().unwrap()),
            11_u32
        );

        memory.set(8, NonZeroU32::new(23).unwrap());
        assert_eq!(u32::from(memory.get(8).unwrap().unwrap()), 23_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(0, -1)).unwrap().unwrap()),
            23_u32
        );

        memory.set(9, NonZeroU32::new(25).unwrap());
        assert_eq!(u32::from(memory.get(9).unwrap().unwrap()), 25_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(1, -1)).unwrap().unwrap()),
            25_u32
        );

        memory.set(10, NonZeroU32::new(26).unwrap());
        assert_eq!(u32::from(memory.get(10).unwrap().unwrap()), 26_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(2, -1)).unwrap().unwrap()),
            26_u32
        );

        memory.set(11, NonZeroU32::new(54).unwrap());
        assert_eq!(u32::from(memory.get(11).unwrap().unwrap()), 54_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(2, 0)).unwrap().unwrap()),
            54_u32
        );

        memory.set(12, NonZeroU32::new(57).unwrap());
        assert_eq!(u32::from(memory.get(12).unwrap().unwrap()), 57_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(2, 1)).unwrap().unwrap()),
            57_u32
        );

        memory.set(13, NonZeroU32::new(59).unwrap());
        assert_eq!(u32::from(memory.get(13).unwrap().unwrap()), 59_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(2, 2)).unwrap().unwrap()),
            59_u32
        );

        memory.set(14, NonZeroU32::new(122).unwrap());
        assert_eq!(u32::from(memory.get(14).unwrap().unwrap()), 122_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(1, 2)).unwrap().unwrap()),
            122_u32
        );

        memory.set(15, NonZeroU32::new(133).unwrap());
        assert_eq!(u32::from(memory.get(15).unwrap().unwrap()), 133_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(0, 2)).unwrap().unwrap()),
            133_u32
        );

        memory.set(16, NonZeroU32::new(142).unwrap());
        assert_eq!(u32::from(memory.get(16).unwrap().unwrap()), 142_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(-1, 2)).unwrap().unwrap()),
            142_u32
        );

        memory.set(17, NonZeroU32::new(147).unwrap());
        assert_eq!(u32::from(memory.get(17).unwrap().unwrap()), 147_u32);
        assert_eq!(
            u32::from(memory.get_pos(Position(-2, 2)).unwrap().unwrap()),
            147_u32
        );
    }

    #[test]
    fn value_at_pos_test() {
        let mut memory = Memory::new_filled(8);
        memory.set(1, NonZeroU32::new(1).unwrap());
        memory.set(2, NonZeroU32::new(1).unwrap());
        memory.set(3, NonZeroU32::new(2).unwrap());
        memory.set(4, NonZeroU32::new(4).unwrap());
        memory.set(5, NonZeroU32::new(5).unwrap());
        assert_eq!(memory.value_at_pos(&Position(-1, 0)), 10);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("277678"), 279138)
    }
}
