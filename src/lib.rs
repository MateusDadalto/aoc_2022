mod helper;

enum Command {
    Noop,
    Add(i32)
}

impl Command {
    fn parse(line: &str) -> Self{
        let args: Vec<&str> = line.split_whitespace().collect();

        match args[0] {
            "noop" => Self::Noop,
            "addx" => Self::Add(args.get(1).expect("should have second arg").parse::<i32>().expect("should be a number")),
            _ => panic!("I don't know this command")
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Cpu {
    cycle: u32,
    during: i32,
    after: i32
}

impl Cpu {
    fn new() -> Self {
        Cpu { during: 1, after: 1 , cycle: 0}
    }

    fn execute(self, cmd: Command) -> Vec<Self> {
        match cmd {
            Command::Noop => vec![self.next_cycle(None)],
            Command::Add(x) => {
                let first_cycle = self.next_cycle(None);
                vec![
                    first_cycle,
                    first_cycle.next_cycle(Some(x))
                ]
            },
        }
    }

    fn next_cycle(self, x: Option<i32>) -> Self {
        Cpu {
            during: self.after,
            after: x.map_or(self.after, |n| self.after + n),
            cycle: self.cycle + 1
        }
    }

    fn signal_strength(&self) -> i32 {
        self.during * (self.cycle as i32)
    }
}

pub fn solve() {
    let lines = helper::get_file_lines_iter("inputs/input.txt");
    
    let initial_state = Cpu::new();
    let mut states = vec![initial_state];

    for line in lines {
        let cmd = Command::parse(line.unwrap().as_str());
        
        states.extend(states.last().unwrap().execute(cmd));
    }

    let mut answer = 0;
    for i in [20, 60, 100, 140, 180, 220] {
        answer += states[i].signal_strength();
    }
    
    println!("Day 10 part 1: {}", answer);
}
