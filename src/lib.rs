mod helper;

pub fn solve() {
    let lines = helper::get_file_lines_iter("inputs/input.txt");

    for line in lines {
        println!("{}", line.unwrap());
    }
}