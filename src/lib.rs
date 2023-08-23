use std::{collections::VecDeque, fmt::Debug};

mod helper;

struct ThrowCmd {
    monkey_id: usize,
    item: u64
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
    inspections: u64,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspections", &self.inspections)
            .finish()
    }
}

impl Monkey {
    fn new<T, U>(items: VecDeque<u64>, operation: T, test: U) -> Self
    where
        T: Fn(u64) -> u64 + 'static,
        U: Fn(u64) -> usize + 'static,
    {
        Monkey {
            items,
            operation: Box::new(operation),
            test: Box::new(test),
            inspections: 0
        }
    }

    fn throw_item(&mut self) -> Option<ThrowCmd> {

        self.items.pop_front().map(|item| {
            let super_modulo = 2*3*5*7*11*13*17*19; // module arithmetics, don't ask me about it
            let item = (self.operation)(item)%super_modulo;
            self.inspections += 1;
            
            let monkey_id = (self.test)(item);

            ThrowCmd { monkey_id, item }
        })
    }

    fn receive_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

pub fn solve() {
    let mut monkeys = get_monkeys();

    for _ in 0..10_000 {
        monkeys = play_round(monkeys);
    }

    monkeys.sort_by(|m, n| n.inspections.cmp(&m.inspections));
    let monkey_business = monkeys[0].inspections * monkeys[1].inspections;
    println!("Day 11 part 1: {:?}", monkey_business);
}

fn play_round(mut monkeys: [Monkey;8]) -> [Monkey;8] 
{
    for i in 0..monkeys.len() {
        loop {
            let monkey = monkeys.get_mut(i).unwrap();
            let Some(cmd) = monkey.throw_item() else {
                break;
            };
            
            let monkey = monkeys.get_mut(cmd.monkey_id).unwrap();
            monkey.receive_item(cmd.item);
        }
    }

    monkeys
}

fn get_monkeys() -> [Monkey; 8] {
    [
        Monkey::new(
            VecDeque::from([54, 61, 97, 63, 74]),
            |old| old * 7,
            |item| if item % 17 == 0 { 5 } else { 3 },
        ),
        Monkey::new(
            VecDeque::from([61, 70, 97, 64, 99, 83, 52, 87]),
            |old| old + 8,
            |item| if item % 2 == 0 { 7 } else { 6 },
        ),
        Monkey::new(
            VecDeque::from([60, 67, 80, 65]),
            |old| old * 13,
            |item| if item % 5 == 0 { 1 } else { 6 },
        ),
        Monkey::new(
            VecDeque::from([61, 70, 76, 69, 82, 56]),
            |old| old + 7,
            |item| if item % 3 == 0 { 5 } else { 2 },
        ),
        Monkey::new(
            VecDeque::from([79, 98]),
            |old| old + 2,
            |item| if item % 7 == 0 { 0 } else { 3 },
        ),
        Monkey::new(
            VecDeque::from([72, 79, 55]),
            |old| old + 1,
            |item| if item % 13 == 0 { 2 } else { 1 },
        ),
        Monkey::new(
            VecDeque::from([63]),
            |old| old + 4,
            |item| if item % 19 == 0 { 7 } else { 4 },
        ),
        Monkey::new(
            VecDeque::from([72, 51, 93, 63, 80, 86, 81]),
            |old| old * old,
            |item| if item % 11 == 0 { 0 } else { 4 },
        ),
    ]
}

// site example
// fn get_monkeys() -> [Monkey; 4] {
//     [
//         Monkey::new(
//             VecDeque::from([79, 98]),
//             |old| old*19, 
//             |item| if item % 23 == 0 { 2 } else { 3 }, 
//         ),
//         Monkey::new(
//             VecDeque::from([54, 65, 75, 74]),
//             |old| old + 6, 
//             |item| if item % 19 == 0 { 2 } else { 0 }, 
//         ),
//         Monkey::new(
//             VecDeque::from([79, 60, 97]),
//             |old| old*old, 
//             |item| if item % 13 == 0 { 1 } else { 3 }, 
//         ),
//         Monkey::new(
//             VecDeque::from([74]),
//             |old| old + 3, 
//             |item| if item % 17 == 0 { 0 } else { 1 }, 
//         )
//     ]
// }
