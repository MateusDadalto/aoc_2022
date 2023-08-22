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

    fn should_draw_pixel(&self, col: usize) -> bool {
        (col as i32).abs_diff(self.during) <= 1
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

    draw_crt(states)
}

fn draw_crt(states: Vec<Cpu>) {
    let n_lines = states.len()/4;
    let states = &states[1..]; // initial state is meaningless
    let width = 40;
    for line in 0..n_lines {
        let mut line_drawing = String::new();
        for position in 0..width {
            let cycle = states.get(line*width + position).expect("Should have a cycle for each pixel");

            match cycle.should_draw_pixel(position) {
                true => line_drawing = line_drawing + "# ",
                false => line_drawing = line_drawing + ". ",
            }
        }

        println!("{line_drawing}");
    }
}
