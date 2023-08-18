use std::{
    collections::{HashMap, VecDeque, hash_map::Keys},
    fs,
    hash::Hash,
};

struct RingBuffer<T> {
    values: VecDeque<T>,
    size: usize,
}

impl<T> RingBuffer<T> {
    fn new(size: usize) -> RingBuffer<T> {
        RingBuffer {
            values: VecDeque::with_capacity(size),
            size,
        }
    }

    fn push(&mut self, value: T) -> Option<T> {
        let mut popped = None;
        if self.values.len() == self.size {
            popped = self.values.pop_back();
        }

        self.values.push_front(value);

        popped
    }
}

struct Counter<K>(HashMap<K, u32>);

impl<K> Counter<K>
where
    K: Eq + Hash + Clone,
{
    fn with_capacity(size: usize) -> Counter<K> {
        Counter(HashMap::with_capacity(size))
    }

    fn add(&mut self, value: K) {
        self.0
            .entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    fn subtract(&mut self, value: K) {
        self.0.entry(value.clone()).and_modify(|count| *count -= 1);

        if let Some(count) = self.0.get(&value) {
            if *count == 0 {
                self.0.remove(&value);
            }
        }
    }

    fn keys(&self) -> Keys<'_, K, u32>  {
        self.0.keys()
    }
}

pub fn solve() {
    let datastream = fs::read_to_string("inputs/day_six.txt").unwrap();
    let mut char_iter = datastream.chars().enumerate();
    // start of packet
    find_marker(4, &mut char_iter);

    // start of message
    find_marker(14, &mut char_iter);

}

fn find_marker(size: usize, char_iter: &mut impl Iterator<Item = (usize, char)>) -> Option<(usize, char)> {
    let mut sop_buffer: RingBuffer<char> = RingBuffer::new(size);
    let mut sop_char_count: Counter<char> = Counter::with_capacity(size);
    
    // find message beginning
    while let Some((i,c)) = char_iter.next() {
        sop_char_count.add(c.clone());

        if let Some(popped) = sop_buffer.push(c.clone()) {
            sop_char_count.subtract(popped);

            if sop_char_count.keys().len() == size {
                println!("Marker found after char {} at index {}", c, i+1);
                return Some((i, c));
            }
        }
    }

    None
}
