use std::ops::RangeInclusive;

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

    fn range(&self) -> RangeInclusive<u8> {
        self.start..=self.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.contains_start(&other) || self.contains_end(&other)
            || other.contains_start(self) || other.contains_end(self)
    }

    fn contains_start(&self, other: &Assignment) -> bool {
        
        self.range().contains(&other.start)
    }
    
    fn contains_end(&self, other: &Assignment) -> bool {
        self.range().contains(&other.end)
    }

    #[allow(dead_code)]
    fn contains(&self, other: &Assignment) -> bool {
        self.contains_start(&other) && self.contains_end(&other)
    }

    #[allow(dead_code)]
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

        if pair.0.overlaps(&pair.1) {
            counter += 1;
        }
    }

    println!("DAY 4 PART 1 ANSWER: {counter}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assignment_overlaps() {
        let a1 = Assignment { start: 6, end: 10 };
        let a2 = Assignment { start: 8, end: 12 };
        let a3 = Assignment { start: 2, end: 7 };
        let a4 = Assignment { start: 9, end: 9 };
        let a5 = Assignment { start: 1, end: 9 };

        // overlaps
        assert!(a1.overlaps(&a2));
        assert!(a1.overlaps(&a3));
        assert!(a1.overlaps(&a4));
        assert!(a1.overlaps(&a5));
        assert!(a2.overlaps(&a4));
        assert!(a5.overlaps(&a4), "Should be inclusive range");
        
        // opposite should also overlap
        assert!(a2.overlaps(&a1));
        assert!(a3.overlaps(&a1));
        assert!(a4.overlaps(&a1));
        assert!(a5.overlaps(&a1));
        assert!(a4.overlaps(&a2));
        assert!(a4.overlaps(&a5));

        //should not overlap
        assert!(!a4.overlaps(&a3));
        assert!(!a2.overlaps(&a3));

        // opposite also should not overlap
        assert!(!a3.overlaps(&a4));
        assert!(!a3.overlaps(&a2));
    }
}