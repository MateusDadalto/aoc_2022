use std::{cell::RefCell, fmt::Debug, rc::Rc};

mod helper;

const STARTING_CHAR: char = 'S';
const ENDING_CHAR: char = 'E';

type NodeRef = Rc<RefCell<Node>>;
struct Grid(Vec<Vec<NodeRef>>);

impl Grid {
    fn link_nodes(&self) {
        for (l_index, line) in self.0.iter().enumerate() {
            for (c_index, node) in line.iter().enumerate() {
                node.as_ref()
                    .borrow_mut()
                    .link_node(self.get_neighbors(l_index as i32, c_index as i32));
            }
        }
    }

    fn get_neighbors(&self, line: i32, col: i32) -> Vec<NodeRef> {
        let neighbors = [
            (line - 1, col), // Up
            (line + 1, col), // Down
            (line, col - 1), // Left
            (line, col + 1), // Right
        ];

        neighbors
            .into_iter()
            .filter(|(l, c)| (l.is_positive() || l == &0) && (c.is_positive() || c == &0))
            .filter_map(|(l, c)| {
                self.0
                    .get(l as usize)
                    .and_then(|nodes| nodes.get(c as usize))
                    .map(|reference| Rc::clone(reference))
            })
            .collect()

        // neighbors.into_iter()
        //     .map(|(l, c)| {
        //         self.0.get(l)
        //             .and_then(|nodes| nodes.get(c))
        //     })
        //     .flatten()
        //     .map(|rc|Rc::clone(rc))
        //     .collect()
    }

    fn get_starting_node(&self) -> Option<NodeRef> {
        self.0
            .iter()
            .flatten()
            .find(|node| node.borrow().is_starting)
            .map(|node| Rc::clone(node))
    }

    fn get_ending_node(&self) -> Option<NodeRef> {
        self.0
            .iter()
            .flatten()
            .find(|node| node.borrow().is_ending)
            .map(|node| Rc::clone(node))
    }
}
struct Node {
    height: u8,
    connection: Vec<NodeRef>,
    is_starting: bool,
    is_ending: bool,
    distance: u32,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("value", &self.height)
            .field("connection", &self.connection.iter().map(|n| n.borrow().get_node_char()).collect::<Vec<_>>())
            .field("is_starting", &self.is_starting)
            .field("is_ending", &self.is_ending)
            .field("distance", if &self.distance == &u32::MAX {&"MAX"} else {&self.distance})
            .finish()
    }
}

impl Node {
    fn new(c: char) -> Self {
        Node {
            height: Node::build_node_value(c.clone()),
            connection: vec![],
            is_starting: c.eq(&STARTING_CHAR),
            is_ending: c.eq(&ENDING_CHAR),
            distance: if c.eq(&STARTING_CHAR) {0} else {u32::MAX},
        }
    }

    fn build_node_value(c: char) -> u8 {
        if c.is_uppercase() {
            return match c {
                STARTING_CHAR => 'a' as u8 - 96,
                ENDING_CHAR => 'z' as u8 - 96,
                _ => panic!("Not a valid character"),
            };
        }

        (c as u8) - 96
    }

    fn get_node_char(&self) -> char {
        if self.is_starting {
            return STARTING_CHAR;
        } else if self.is_ending {
            return ENDING_CHAR;
        }

        (self.height + 96) as char
    }

    fn link_node(&mut self, neighbors: Vec<NodeRef>) {
        for node_ref in neighbors {
            if (node_ref.borrow().height as i8) - (self.height as i8) <= 1 {
                self.connection.push(Rc::clone(&node_ref))
            }
        }
    }
}

pub fn solve() {
    let grid: Grid = Grid(
        helper::get_file_lines_iter("inputs/input.txt")
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| Rc::new(RefCell::new(Node::new(c))))
                    .collect()
            })
            .collect(),
    );

    grid.link_nodes();

    dbg!(grid.get_starting_node());
    dbg!(grid.get_ending_node());
}
