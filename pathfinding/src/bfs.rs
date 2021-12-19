extern crate piston_window;

use crate::*;
use std::collections::VecDeque;

pub struct BFS<'a> {
    queue: VecDeque<usize>,
    explored: Vec<usize>,
    start: usize,
    finish: usize,
    maze: &'a Vec<Node>,
    links: Vec<[usize; 2]>,
    current: usize,
}

impl BFS<'_> {
    fn draw_current_state(&self, context: &Context, g: &mut G2d) {
        //draw all links
        self.links.iter().for_each(|[n, m]| {
            draw_link(context, g, *n, *m);
        });

        //draw start, finish and current node
        {
            draw_node_indicator(context, g, self.current, [1.0, 0.5, 0.0, 1.0]);
            draw_node_indicator(context, g, self.start, [0.0, 0.0, 1.0, 1.0]);
            draw_node_indicator(context, g, self.finish, [0.0, 0.0, 1.0, 1.0]);
        }
    }
}

impl<'a> CreateAlgo<'a> for BFS<'a> {
    fn new(maze: &'a Vec<Node>, start: usize, finish: usize) -> BFS<'_> {
        BFS {
            maze,
            start,
            finish,
            queue: VecDeque::from(vec![start]),
            explored: vec![start],
            links: Vec::new(),
            current: start,
        }
    }
}

impl SearchAlgo for BFS<'_> {
    fn solve(&mut self) -> Option<Vec<usize>> {
        while let Some(x) = self.queue.pop_front() {
            if x == self.finish {
                return None;
            }

            let node: &Node = &self.maze[x];
            for d in node.neighbours.iter() {
                let neighbour_index = match d {
                    Direction::Up => get_array_val(node.x, node.y - 1),
                    Direction::Down => get_array_val(node.x, node.y + 1),
                    Direction::Left => get_array_val(node.x - 1, node.y),
                    Direction::Right => get_array_val(node.x + 1, node.y),
                };

                if !self.explored.contains(&neighbour_index) {
                    self.queue.push_back(neighbour_index);
                    self.explored.push(neighbour_index);
                    self.links.push([x, neighbour_index]);

                    if neighbour_index == self.finish {
                        return None;
                    }
                }
            }
        }
        return None;
    }

    fn control_solve(&mut self, window: &mut PistonWindow) {
        let mut is_solved = false;
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                self.draw_current_state(&c, g);
            });

            if let Some(button) = e.press_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    if is_solved {
                        break;
                    } else {
                        if let Some(r) = self.iter_solve(){
                            let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
                            get_coord_vals(self.start, &mut a, &mut b);
                            get_coord_vals(self.finish, &mut c, &mut d);
                            if r {
                                println!("Success from [{}, {}] to [{}, {}]", a, b, c, d);
                            } else {
                                println!("Failure from [{}, {}] to [{}, {}]", a, b, c, d);
                            }
                            is_solved = true;
                        };
                    }
                } 
                else if button == Button::Mouse(MouseButton::Right) {
                    break;
                }
            }
        }
    }

    fn iter_solve(&mut self) -> Option<bool> {
        match self.queue.pop_front() {
            Some(x) => {
                if x == self.finish {
                    return Some(true);
                }

                self.current = x;
                let node: &Node = &self.maze[x];
                for d in node.neighbours.iter() {
                    let neighbour_index = match d {
                        Direction::Up => get_array_val(node.x, node.y - 1),
                        Direction::Down => get_array_val(node.x, node.y + 1),
                        Direction::Left => get_array_val(node.x - 1, node.y),
                        Direction::Right => get_array_val(node.x + 1, node.y),
                    };
    
                    if !self.explored.contains(&neighbour_index) {
                        self.queue.push_back(neighbour_index);
                        self.explored.push(neighbour_index);
                        self.links.push([x, neighbour_index]);
    
                        if neighbour_index == self.finish {
                            return Some(true);
                        }
                    }
                }
            }
            None => return Some(false),
        };

        return None;
    }
}
