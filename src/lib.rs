use std::collections::HashMap;

const _EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");
// these limits represent the borders that the rock can reach
//
// Right        Left
// 01000000     00000001
// 01000000     00000001
// 01000000     00000001
// 01000000     00000001
const LEFT_LIMIT: u32 = 0x40404040;
const RIGHT_LIMIT: u32 = 0x01010101;

pub fn solve() {
    let mut winds = INPUT.trim().chars().map(|c| Direction::parse(c)).enumerate().cycle();

    let mut chamber = Chamber::with_capacity(10000);

    let total_rocks: usize = 1_000_000_000_000;
    let mut rocks_added = 0;
    let mut accumulated_height = 0;
    let mut rocks = Rock::all().into_iter().cycle();

    let mut seen = HashMap::new();

    while rocks_added < total_rocks {
        let rock = rocks.next().unwrap();

        let idx = chamber.add_rock(&mut winds, rock);

        rocks_added +=1;

        if chamber.height() < 8 { continue; }

        let state = (chamber.skyline(), rock, idx);

        if let Some((prev_rocks_added, prev_height)) = seen.get(&state) {
            let rocks_per_cycle: usize = rocks_added - prev_rocks_added;

            let cycles_left: usize = (total_rocks - rocks_added) / rocks_per_cycle;

            rocks_added += rocks_per_cycle * cycles_left;

            accumulated_height += cycles_left * (chamber.height() - prev_height);

            seen.clear();
            continue;
        }

        seen.insert(state, (rocks_added, chamber.height()));
    }

    // chamber.draw();
    println!("{}", chamber.height() + accumulated_height);
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            any => panic!("Wtf is your input: {any}"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Rock(u32);

impl Rock {
    // return all rock values (big-endian representation)
    /// 00000000 | 00000000 | 00000000 | 00010000 | 00000000
    /// 00000000 | 00001000 | 00000100 | 00010000 | 00000000
    /// 00000000 | 00011100 | 00000100 | 00010000 | 00011000
    /// 00011110 | 00001000 | 00011100 | 00010000 | 00011000
    const fn all() -> [Self; 5] {
        [
            Self(0x0000001E), // - 00000000_00000000_00000000_00011110
            Self(0x00081C08), // + 00000000_00001000_00011100_00001000
            Self(0x0004041C), // L 00000000_00000100_00000100_00011100
            Self(0x10101010), // | 00010000_00010000_00010000_00010000
            Self(0x00001818), // # 00000000_00000000_00011000_00011000
        ]
    }

    fn shove(&mut self, wind: Direction, layers: u32) {
        let mut possible_position = self.0;

        match wind {
            Direction::Right => {
                if self.0 & RIGHT_LIMIT == 0 {
                    possible_position = self.0 >> 1;
                }
            },
            Direction::Left => {
                if self.0 & LEFT_LIMIT == 0 {
                    possible_position = self.0 << 1;
                }
            },
        }

        if possible_position & layers == 0 {
            self.0 = possible_position;
        }
    }

    fn collides(&self, layers_with_rocks: u32) -> bool {
        self.0 & layers_with_rocks != 0
    }

    fn bytes(&self) -> impl DoubleEndedIterator<Item = u8> {
        self.0.to_be_bytes().into_iter().filter(|b| *b > 0)
    }

    fn draw(&self) {
        for b in self.bytes() {
            let s = format!("{:0width$b}", b, width = 7)
                .replace("0", ".")
                .replace("1", "#");
            println!("{s}");
        }
        println!("_______");
    }
}

struct Chamber {
    layers: Vec<u8>
}

impl Chamber {
    fn with_capacity(n: usize) -> Self {
        Self {
            layers: Vec::with_capacity(n)
        }
    }

    fn height(&self) ->  usize {
        self.layers.len()
    }

    fn add_top_layer(&mut self, layer: u8) {
        self.layers.push(layer);
    }

    // return a u32 (4 layers) from the chamber starting at level x
    fn get_chunk_at(&self, level: usize) -> u32 {
        if level >= self.height() {
            return 0;
        }

        // Starting at `level`, take up to four bytes from the chamber, reverse
        // the production (so that the chunk is right-side up) of bytes, then 
        // convert the four bytes into a single u32 by shifting existing bits
        // left (acc << 8) and adding (bitwise "or" | operator) each new byte 
        // to the first 8 bits after the shift.
        self.layers
            .iter()
            .skip(level)
            .take(4)
            .rev()
            .fold(0, |acc, layer| (acc << 8) | (*layer as u32))
    }

    fn add_rock(&mut self, winds: &mut impl Iterator<Item = (usize, Direction)>, mut rock: Rock) -> usize {
        let mut level = self.height() + 3;
        let mut wind_index = 0;

        loop {
            let layers = self.get_chunk_at(level);

            let (idx, wind) = winds.next().unwrap();
            wind_index = idx;

            rock.shove(wind, layers);

            if level > self.height() {
                level -= 1;
                continue;
            }

            let layers_with_rocks = self.get_chunk_at(level.saturating_sub(1));

            if level == 0 || rock.collides(layers_with_rocks) {
                for byte in rock.bytes().rev() {
                    if level < self.height() {
                        self.layers[level] |= byte;
                    } else {
                        self.add_top_layer(byte);
                    }
                    level += 1;
                }
                break;
            }

            level -= 1;
        }

        wind_index
    }

    fn draw(&self) {
        for layer in self.layers.iter().rev() {
            let s = format!("{:0width$b}", layer, width = 7)
                .replace("0", ".")
                .replace("1", "#");
            println!("{s}");
        }
        println!("_______");
    }

    fn skyline(&self) -> Option<u64>{
        if self.height() < 8{
            return None;
        }

        // similar to chunk, but the size is 8 layers (64 bits)
        let result = self.layers
            .iter()
            .rev()
            .take(8)
            .fold(0, |acc, layer| (acc << 8) | (*layer as u64));

        Some(result)
    }
}

fn draw(chunk: u32) {
    for layer in chunk.to_be_bytes() {
        let s = format!("{:0width$b}", layer, width = 7)
            .replace("0", ".")
            .replace("1", "#");
        println!("{s}");
    }
    println!("_______");
}