use std::{cell::RefCell, fmt::Debug, rc::Rc, collections::HashSet, hash::{Hash, Hasher}};

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
    id: (usize, usize),
    height: u8,
    connection: Vec<NodeRef>,
    is_starting: bool,
    is_ending: bool,
    distance: u32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("id", &self.id)
            .field("value", &self.height)
            .field("connection", &self.connection.iter().map(|n| n.borrow().get_node_char()).collect::<Vec<_>>())
            .field("is_starting", &self.is_starting)
            .field("is_ending", &self.is_ending)
            .field("distance", if &self.distance == &u32::MAX {&"MAX"} else {&self.distance})
            .finish()
    }
}

impl Node {
    fn new(c: char, line: usize, col: usize) -> Self {
        Node {
            id: (line, col),
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
            .enumerate()
            .map(|(i, l)| {
                l.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(c_i, c)| Rc::new(RefCell::new(Node::new(c, i, c_i))))
                    .collect()
            })
            .collect(),
    );

    grid.link_nodes();

    let path:u32 = find_shortest_path(&grid, grid.get_starting_node().unwrap(), grid.get_ending_node().unwrap());
    
    println!("{path}")
}

fn find_shortest_path(grid: &Grid, starting_node: NodeRef, ending_node: NodeRef) -> u32 {
    let mut unvisited = HashSet::new();
    unvisited.extend(grid.0.iter().flatten().map(|node_ref| node_ref.borrow().id.clone()));

    let mut current_node = Rc::clone(&starting_node);

    while current_node.borrow().id != ending_node.borrow().id {
        let mut min_distance: u32 = u32::MAX;
        let distance = current_node.borrow().distance + 1;
        
        for neighbor in current_node.borrow().connection.iter() {
            if unvisited.contains(&neighbor.borrow().id) {

                if distance < neighbor.borrow().distance {
                    neighbor.borrow_mut().distance = distance;
                }

                if min_distance > neighbor.borrow().distance {
                    min_distance = neighbor.borrow().distance;
                }
            }

        }

        let next = Rc::clone(
            if let Some(node) = current_node.borrow().connection.iter()
                .find(|node| node.borrow().distance == min_distance && !unvisited.contains(&node.borrow().id)) {
                    node
                } else {
                    unvisited
                        .iter()
                        .filter_map(|&(line, col)| grid.0.get(line).and_then(|row| row.get(col)))
                        .find(|node_ref| node_ref.borrow().distance < u32::MAX)
                        .unwrap_or(&ending_node)
                }
            );

        current_node = next;
    }
    
    ending_node.borrow().distance
}
