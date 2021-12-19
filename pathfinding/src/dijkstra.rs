extern crate piston_window;

use crate::*;
use std::cmp::Ordering;

#[derive(PartialEq, Clone, Copy)]
struct Mem {
    distance: u32,
    antecedant: usize,
}

impl PartialOrd for Mem {
    fn partial_cmp(&self, other: &Mem) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl From<(u32, usize)> for Mem {
    fn from(tuple: (u32, usize)) -> Self {
        Mem { distance: tuple.0, antecedant: tuple.1 }
    }
}

pub struct Dijkstra<'a> {
    start: usize,
    finish: usize,
    maze: &'a Vec<Node>,

    solution: Vec<usize>,
    
    visited: Vec<usize>,
    distances: Vec<Mem>,
    current_node: usize,
}

impl Dijkstra<'_> {
    fn draw_current_state(&mut self, context: &Context, g: &mut G2d, is_solved: bool) {
        if is_solved {
            if self.distances[self.finish].distance != u32::MAX {
                //if there is indeed a solution
                draw_path(context, g, &self.solution);
                
                draw_node_indicator(context, g, self.start, [0.0, 0.0, 1.0, 1.0]);
                draw_node_indicator(context, g, self.finish, [0.0, 0.0, 1.0, 1.0]);
            }
        } else {
            self.distances.iter().enumerate().for_each(|(index, mem)| if mem.distance < u32::MAX { draw_link(context, g, index, mem.antecedant) });
            
            draw_node_indicator(context, g, self.start, [0.0, 0.0, 1.0, 1.0]);
            draw_node_indicator(context, g, self.finish, [0.0, 0.0, 1.0, 1.0]);
            draw_node_indicator(context, g, self.current_node, [1.0, 0.5, 0.0, 1.0]);
        }
    }

    fn build_solution(&mut self) {
        assert!(self.distances[self.finish].distance != u32::MAX);

        let mut index = self.finish;
        while index != self.start {
            self.solution.push(index);
            index = self.distances[index].antecedant;
        }
        self.solution.push(self.start);
    }
}


impl<'a> CreateAlgo<'a> for Dijkstra<'a> {
    fn new(maze: &'a Vec<Node>, start: usize, finish: usize) -> Dijkstra<'_> {
        let mut dij = Dijkstra { start, finish, maze, 
            solution: Vec::new(),
            visited: vec![finish],
            distances: vec![Mem { distance: u32::MAX, antecedant: usize::MAX }; SIZE as usize], 
            current_node: start,
        };
        dij.distances[start] = (0 as u32, start).into();
        return dij;
    }

}

impl SearchAlgo for Dijkstra<'_> {
    fn solve(&mut self) -> Option<Vec<usize>> {
        while self.iter_solve().is_none() {}

        if self.distances[self.finish].distance != u32::MAX {
            //we reach the goal
            self.build_solution();
            return Some(self.solution.clone());
        } else {
            //unreachable
            return None;
        }
    }

    fn control_solve(&mut self, window: &mut PistonWindow) {
        let mut is_solved = false;
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {                            
                clear([1.0, 1.0, 1.0, 1.0], g);
                draw_labyrinth(&c, g, self.maze);
                self.draw_current_state(&c, g, is_solved);
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
                            self.build_solution();
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
        if self.start == self.finish {
            return Some(true);
        }
        
        let search_result = self.distances.iter().enumerate().reduce(|accum, item| {
            if (item.1 < accum.1 && !self.visited.contains(&item.0)) || self.visited.contains(&accum.0) { item } else { accum }
        });

        if search_result.is_none() { return Some(false); }

        let (a, b) = search_result.unwrap();
        let (node_index, node_info) = (a, *b);
            
        if node_info.distance == u32::MAX || self.visited.len() == SIZE as usize {
            if self.distances[self.finish].distance != u32::MAX {
                //we reach the goal
                return Some(true);
            } else {
                //unreachable
                return Some(false);
            }
        }
    
        if node_info.distance + 1 < self.distances[self.finish].distance {
            let node: &Node = &self.maze[node_index];
            for d in node.neighbours.iter() {
                let neightbour_index = get_array_val_from_pos_and_dir(node.x, node.y, *d);
                let neightbour_node_info = &mut self.distances[neightbour_index];
                if node_info.distance + 1 < neightbour_node_info.distance {
                    neightbour_node_info.distance = node_info.distance + 1;
                    neightbour_node_info.antecedant = node_index;
                }
            }
        }

        self.visited.push(node_index);
        self.current_node = node_index; 
        None
    }
}