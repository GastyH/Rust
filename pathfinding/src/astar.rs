extern crate piston_window;

use crate::*;
use std::cmp::Ordering;

#[derive(PartialEq, Clone, Copy)]
struct NodeInfo {
    distance: u32,
    heuristic: u32,
    antecedant: usize,
}

impl PartialOrd for NodeInfo {
    fn partial_cmp(&self, other: &NodeInfo) -> Option<Ordering> {
        let self_is_max = self.distance == u32::MAX || self.heuristic == u32::MAX;
        let other_is_max = other.distance == u32::MAX || other.heuristic == u32::MAX;
        if self_is_max && other_is_max {
            return Some(Ordering::Equal);
        }
        if self_is_max {
            return Some(Ordering::Greater);
        }
        if other_is_max {
            return Some(Ordering::Less);
        }
        (self.distance + self.heuristic).partial_cmp(&(other.distance + other.heuristic))
    }
}

impl From<(u32, u32, usize)> for NodeInfo {
    fn from(tuple: (u32, u32, usize)) -> Self {
        NodeInfo { distance: tuple.0, heuristic: tuple.1, antecedant: tuple.2 }
    }
}

pub struct AStar<'a> {
    start: usize,
    finish: usize,
    maze: &'a Vec<Node>,
    
    solution: Vec<usize>,
    
    current: usize,
    visited: Vec<usize>,
    node_infos: Vec<NodeInfo>,
}

impl AStar<'_> {
    fn draw_current_state(&mut self, context: &Context, g: &mut G2d, is_solved: bool) {
        if is_solved {
            if self.node_infos[self.finish].distance != u32::MAX {
                //if there is indeed a solution
                draw_path(context, g, &self.solution);
                
                draw_node_indicator(context, g, self.start, [0.0, 0.0, 1.0, 1.0]);
                draw_node_indicator(context, g, self.finish, [0.0, 0.0, 1.0, 1.0]);
            }
        } else {
            self.node_infos.iter().enumerate().for_each(|(index, mem)| if mem.distance < u32::MAX { draw_link(context, g, index, mem.antecedant) });
            
            draw_node_indicator(context, g, self.start, [0.0, 0.0, 1.0, 1.0]);
            draw_node_indicator(context, g, self.finish, [0.0, 0.0, 1.0, 1.0]);
            draw_node_indicator(context, g, self.current, [1.0, 0.5, 0.0, 1.0]);
        }
    }

    fn build_solution(&mut self) {
        assert!(self.node_infos[self.finish].distance != u32::MAX);

        let mut index = self.finish;
        while index != self.start {
            self.solution.push(index);
            index = self.node_infos[index].antecedant;
        }
        self.solution.push(self.start);
    }
}

impl<'a> CreateAlgo<'a> for AStar<'a> {
    fn new(maze: &'a Vec<Node>, start: usize, finish: usize) -> AStar<'_> {
        let mut astar = AStar { maze, start, finish, current: start, solution: Vec::new(), visited: vec![finish], node_infos: Vec::with_capacity(SIZE as usize) };
        let (mut fx, mut fy) = (0, 0);
        get_coord_vals(finish, &mut fx, &mut fy);
        for i in 0..SIZE {            
            let (mut x, mut y) = (0, 0); 
            get_coord_vals(i as usize, &mut x, &mut y);
            astar.node_infos.push((u32::MAX, ((x - fx).abs() + (y - fy).abs()) as u32, usize::MAX).into());
        }
        astar.node_infos[start].distance = 0;
        astar.node_infos[start].antecedant = start;
        astar
    }
}

impl SearchAlgo for AStar<'_> {
    fn solve(&mut self) -> Option<Vec<usize>> {
        while self.iter_solve().is_none() {}

        if self.node_infos[self.finish].distance != u32::MAX {
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
                            if r {
                                self.build_solution();
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
        if self.start == self.finish {
            return Some(true);
        }
        
        let search_result = self.node_infos.iter().enumerate().reduce(|accum, item| {
            if (item.1 < accum.1 && !self.visited.contains(&item.0)) || self.visited.contains(&accum.0) { item } else { accum }
        });

        if search_result.is_none() { return Some(false); }

        let (a, b) = search_result.unwrap();
        let (node_index, node_info) = (a, *b);

        if node_info.distance == u32::MAX || self.visited.len() == SIZE as usize {
            if self.node_infos[self.finish].distance != u32::MAX {                
                //we reach the goal
                return Some(true);
            } else {
                //unreachable
                return Some(false);
            }
        }
    
        let mut is_solved = false;
        if node_info.distance + 1 < self.node_infos[self.finish].distance {
            let node: &Node = &self.maze[node_index];
            node.neighbours.iter().for_each( |d| {
                    let neightbour_index = get_array_val_from_pos_and_dir(node.x, node.y, *d);
                    let neightbour_node_info = &mut self.node_infos[neightbour_index];
                    if node_info.distance + 1 < neightbour_node_info.distance {
                        neightbour_node_info.distance = node_info.distance + 1;
                        neightbour_node_info.antecedant = node_index;
                    }
                    if neightbour_index == self.finish {
                        is_solved = true;
                    }
                }
            );
        }

        if is_solved {
            return Some(true);
        }

        self.visited.push(node_index);
        self.current = node_index; 
        None
    }
}
