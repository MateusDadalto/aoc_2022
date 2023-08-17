use super::helper;

struct Assignment {
    start: u8,
    end: u8,
}

impl Assignment {
    fn new(range: &str) -> Self {
        let numbers: Vec<u8> = range.split('-').map(|s| s.parse::<u8>().unwrap()).collect();

        Assignment {
            start: numbers[0],
            end: numbers[1],
        }
    }

    fn contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn is_contained(&self, other: &Assignment) -> bool {
        other.contains(self)
    }
}

pub fn solve() {
    let lines = helper::get_file_lines_iter("inputs/day_four.txt");
    let mut counter = 0;
    for line in lines.map(|l| l.unwrap()) {
        
        let mut pair: Vec<Assignment> = line.split(',').map(|range| Assignment::new(range)).collect();
        let pair: (Assignment, Assignment) = (pair.pop().unwrap(), pair.pop().unwrap());

        if pair.0.contains(&pair.1) || pair.0.is_contained(&pair.1) {
            counter += 1;
        }
    }

    println!("DAY 4 PART 1 ANSWER: {counter}");
}
