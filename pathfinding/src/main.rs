extern crate piston_window;

use piston_window::*;
use rand::Rng;

pub mod bfs;
pub mod dfs;
pub mod dijkstra;
pub mod astar;

const WIDTH: i32 = 640;
const HEIGTH: i32 = 480;
const LINES: i32 = 5;
const COLONES: i32 = 5;
const SIZE: i32 = LINES * COLONES;
const SIZEW: f64 = WIDTH as f64 / COLONES as f64;
const SIZEH: f64 = HEIGTH as f64 / LINES as f64;
const WALLWTHICK: f64 = SIZEW * 0.1;
const WALLHTHICK: f64 = SIZEH * 0.1;
const PWALL: f32 = 0.35;

#[derive(Copy, Clone)]
enum Algo {
    BFS,
    DFS,
    Dijkstra,
    AStar,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait CreateAlgo<'a> {
    fn new(maze: &'a Vec<Node>, start: usize, finish: usize) -> Self;
}

trait SearchAlgo {
    fn solve(&mut self) -> Option<Vec<usize>>;
    fn control_solve(&mut self, window: &mut PistonWindow);
    fn iter_solve(&mut self) -> Option<bool>;
}

#[derive(Clone)]
struct Node {
    x: i32,
    y: i32,
    neighbours: Vec<Direction>,
}

fn get_array_val(x: i32, y: i32) -> usize {
    return (x + y * COLONES) as usize;
}

fn get_array_val_from_pos_and_dir(x: i32, y: i32, d: Direction) -> usize {
    match d {
        Direction::Up => return get_array_val(x, y - 1),
        Direction::Down => return get_array_val(x, y + 1),
        Direction::Left => return get_array_val(x - 1, y),
        Direction::Right => return get_array_val(x + 1, y),
    };
}

fn get_coord_vals(index: usize, i: &mut i32, j: &mut i32) {
    *i = index as i32 % COLONES;
    *j = index as i32 / COLONES;
}

fn get_pos_node(index: usize, x: &mut f64, y: &mut f64) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    get_coord_vals(index, &mut i, &mut j);
    *x = i as f64 * SIZEW;
    *y = j as f64 * SIZEH;
}

fn generate_node(index: usize, labyrinth: &mut Vec<Node>, rng: &mut rand::rngs::ThreadRng) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    get_coord_vals(index, &mut x, &mut y);
    labyrinth[index].x = x;
    labyrinth[index].y = y;
    if x > 0 {
        if labyrinth[get_array_val(x - 1, y)]
            .neighbours
            .contains(&Direction::Right)
        {
            labyrinth[index].neighbours.push(Direction::Left);
        }
    }
    if x < COLONES - 1 {
        if rng.gen::<f32>() >= PWALL {
            labyrinth[index].neighbours.push(Direction::Right);
        }
    }
    if y > 0 {
        if labyrinth[get_array_val(x, y - 1)]
            .neighbours
            .contains(&Direction::Down)
        {
            labyrinth[index].neighbours.push(Direction::Up);
        }
    }
    if y < LINES - 1 {
        if rng.gen::<f32>() >= PWALL {
            labyrinth[index].neighbours.push(Direction::Down);
        }
    }
}

fn create_labyrinth() -> Vec<Node> {
    let mut labyrinth = vec![
        Node {
            x: 0,
            y: 0,
            neighbours: Vec::new()
        };
        SIZE as usize
    ];
    let mut rng = rand::thread_rng();

    for i in 0..labyrinth.len() {
        generate_node(i, &mut labyrinth, &mut rng);
    }

    return labyrinth;
}

fn draw_node(c: &Context, g: &mut G2d, index: usize, node: &Node) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    get_pos_node(index, &mut x, &mut y);
    let pos = [x + WALLWTHICK, y + WALLHTHICK, SIZEW -  2.0 * WALLWTHICK, SIZEH - 2.0 * WALLHTHICK];
    rectangle([1.0, 0.0, 0.0, 1.0], pos, c.transform, g);

    if !node.neighbours.contains(&Direction::Up) {
        rectangle([0.0, 0.0, 0.0, 1.0], [x, y, SIZEW, WALLHTHICK], c.transform, g);
    }

    if !node.neighbours.contains(&Direction::Down) {
        rectangle([0.0, 0.0, 0.0, 1.0], [x, y + SIZEH - WALLHTHICK, SIZEW, WALLHTHICK], c.transform, g);
    }

    if !node.neighbours.contains(&Direction::Left) {
        rectangle([0.0, 0.0, 0.0, 1.0], [x, y, WALLWTHICK, SIZEH], c.transform, g);
    }

    if !node.neighbours.contains(&Direction::Right) {
        rectangle([0.0, 0.0, 0.0, 1.0], [x + SIZEW - WALLWTHICK, y, WALLWTHICK, SIZEH], c.transform, g);
    }
}

fn draw_labyrinth(c: &Context, g: &mut G2d, labyrinth: &Vec<Node>) {
    labyrinth.iter().enumerate().for_each(|(i, node)| draw_node(c, g, i, node));
}

fn draw_link(context: &Context, g: &mut G2d, n: usize, m: usize) {
    if n == m { return; }

    let (mut a, mut b, mut c, mut d) = (0.0, 0.0, 0.0, 0.0);
    if n <= m {
        get_pos_node(n, &mut a, &mut b);
        get_pos_node(m, &mut c, &mut d);
    } else {
        get_pos_node(m, &mut a, &mut b);
        get_pos_node(n, &mut c, &mut d);
    }
    a = a + SIZEW / 2.0 - SIZEW * 0.05;
    b = b + SIZEH / 2.0 - SIZEH * 0.05;
    c = c + SIZEW / 2.0 + SIZEW * 0.05;
    d = d + SIZEH / 2.0 + SIZEH * 0.05;
    rectangle(
        [0.0, 1.0, 0.0, 1.0],
        [a, b, c - a, d - b],
        context.transform,
        g,
    );
}

fn draw_path(context: &Context, g: &mut G2d, path: &Vec<usize>) {
    for i in 0..(path.len() - 1) {
        draw_link(context, g, path[i], path[i+1]);
    }
}

fn draw_node_indicator(context: &Context, g: &mut G2d, index: usize, color: [f32; 4]) {
    let (mut x, mut y) = (0.0, 0.0);
    get_pos_node(index, &mut x, &mut y);
    rectangle(
        color,
        [
            x + SIZEW / 2.0 - WALLWTHICK,
            y + SIZEH / 2.0 - WALLHTHICK,
            WALLWTHICK,
            WALLHTHICK,
        ],
        context.transform,
        g,
    );
}

fn create_algo(algo: Algo, start: usize, end: usize, maze: &Vec<Node>) -> Box<dyn SearchAlgo + '_> {
    match algo {
        Algo::BFS => {
          return Box::<bfs::BFS>::new(CreateAlgo::new(maze, start, end));
        },
        Algo::DFS => {
            return Box::<dfs::DFS>::new(CreateAlgo::new(maze, start, end));
        },
        Algo::Dijkstra => {
            return Box::<dijkstra::Dijkstra>::new(CreateAlgo::new(maze, start, end));
        },
        Algo::AStar => {
            return Box::<astar::AStar>::new(CreateAlgo::new(maze, start, end));
        }
    }
}

fn solve_with_algo(window: &mut PistonWindow, algo: Algo, start: usize, end: usize, maze: &Vec<Node>) {
    let mut pathfinder = create_algo(algo, start, end, maze);
    
    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);     
    get_coord_vals(start, &mut a, &mut b);
    get_coord_vals(end, &mut c, &mut d);

   
    let path = pathfinder.solve();
    match path {
        Some(_) => println!("Success from [{}, {}] to [{}, {}]", a, b, c, d),
        None => println!("Failure from [{}, {}] to [{}, {}]", a, b, c, d),
    }

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, g, _| {

            draw_node_indicator(&context, g, start, [0.0, 0.0, 1.0, 1.0]);
            draw_node_indicator(&context, g, end, [0.0, 0.0, 1.0, 1.0]);

            if path.is_some() {
                draw_path(&context, g, &(path.as_ref().unwrap()));
            }
        });

        
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) || button == Button::Mouse(MouseButton::Right) {
                break;
            }
        }
    }
}

fn control_solve_with_algo(window: &mut PistonWindow, algo: Algo, start: usize, end: usize, maze: &Vec<Node>) {
    create_algo(algo, start, end, maze).control_solve(window);
}

fn main() {
    let mut labyrinth = create_labyrinth();
    let mut rng = rand::thread_rng();
    let mut algo = Algo::AStar;

    println!("SIZEW : {},   SIZEH : {},    WALLWTHICK : {},    WALLHTHICK : {}", SIZEW, SIZEH, WALLWTHICK, WALLHTHICK);

    let title = "THE MAZE";
    let mut window: PistonWindow = WindowSettings::new(title, [WIDTH as f64, HEIGTH as f64])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            draw_labyrinth(&c, g, &labyrinth);
        });

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Right) {
                let start = rng.gen_range(0..labyrinth.len());
                let end = rng.gen_range(0..labyrinth.len());                
                solve_with_algo(&mut window, algo, start, end, &labyrinth);
            }
            else if button == Button::Mouse(MouseButton::Left) {
                let start = rng.gen_range(0..labyrinth.len());
                let end = rng.gen_range(0..labyrinth.len());
                control_solve_with_algo(&mut window, algo, start, end, &labyrinth);
            }
            else if button == Button::Mouse(MouseButton::Middle) {
                labyrinth = create_labyrinth();
            }
            else if button == Button::Keyboard(Key::F1) {
                algo = Algo::BFS;
                println!("Using BFS");
            } else if button == Button::Keyboard(Key::F2) {
                algo = Algo::DFS;
                println!("Using DFS");
            } else if button == Button::Keyboard(Key::F3) {
                algo = Algo::Dijkstra;
                println!("Using Dijkstra");            
            } else if button == Button::Keyboard(Key::F4) {
                algo = Algo::AStar;
                println!("Using A Star");
            }
        }
    }
}