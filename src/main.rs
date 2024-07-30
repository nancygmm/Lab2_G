use minifb::{Window, WindowOptions, Key, Scale, ScaleMode};

mod framebuffer;
use crate::framebuffer::Framebuffer;

struct GameOfLife {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl GameOfLife {
    fn new(width: usize, height: usize, initial_state: Vec<(usize, usize)>) -> Self {
        let mut cells = vec![false; width * height];
        for &(x, y) in &initial_state {
            cells[y * width + x] = true;
        }
        GameOfLife { width, height, cells }
    }

    fn step(&mut self) {
        let mut new_cells = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let live_neighbors = self.live_neighbor_count(x, y);
                if self.cells[idx] {
                    new_cells[idx] = live_neighbors == 2 || live_neighbors == 3;
                } else {
                    new_cells[idx] = live_neighbors == 3;
                }
            }
        }
        self.cells = new_cells;
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in [-1, 0, 1].iter().cloned() {
            for dx in [-1, 0, 1].iter().cloned() {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as isize + dx + self.width as isize) % self.width as isize;
                let ny = (y as isize + dy + self.height as isize) % self.height as isize;
                if self.cells[(ny as usize) * self.width + (nx as usize)] {
                    count += 1;
                }
            }
        }
        count
    }

    fn render(&self, framebuffer: &mut Framebuffer) {
        framebuffer.clear();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y * self.width + x] {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "UVG Graphixs",
        window_width,
        window_height,
        WindowOptions {
            borderless: true,
            resize: true,
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    ).unwrap();

    framebuffer.set_background_color(0x99CCFF);
    framebuffer.set_current_color(0xFFFF99);

    let glider = vec![(1, 2), (2, 2), (3, 2), (3, 1), (2, 0)];
    let small_exploder = vec![
        (20, 15), (21, 15), (22, 15), (19, 16), (23, 16), (20, 17), (22, 17), (21, 18),
    ];
    let exploder = vec![
        (40, 20), (41, 20), (42, 20), (43, 20), (44, 20),
        (40, 22), (44, 22),
        (40, 24), (41, 24), (42, 24), (43, 24), (44, 24),
    ];
    let lightweight_spaceship = vec![
        (60, 30), (61, 30), (62, 30), (63, 30), 
        (60, 31), (60, 33), 
        (63, 31), (63, 32), (62, 33)
    ];

    let block = vec![(80, 40), (81, 40), (80, 41), (81, 41)];
    let bee_hive = vec![(83, 40), (84, 39), (85, 39), (86, 40), (85, 41), (84, 41)];
    let loaf = vec![(88, 40), (89, 39), (90, 39), (91, 40), (91, 41), (90, 42), (89, 41)];
    let boat = vec![(93, 40), (94, 40), (93, 41), (94, 42), (92, 41)];
    let tub = vec![(96, 40), (97, 41), (96, 42), (95, 41)];
    let blinker = vec![(100, 40), (100, 41), (100, 42)];
    let beacon = vec![(105, 40), (106, 40), (105, 41), (107, 42), (108, 42), (107, 43)];
    let pulsar = vec![
        (110, 35), (111, 35), (112, 35), (110, 39), (111, 39), (112, 39),
        (110, 37), (112, 37),
        (108, 37), (108, 36), (108, 38),
        (114, 37), (114, 36), (114, 38),
        (115, 35), (116, 35), (117, 35), (115, 39), (116, 39), (117, 39),
        (115, 37), (117, 37)
    ];
    let pentadecathlon = vec![
        (120, 35), (121, 35), (122, 35), (124, 35), (125, 35), (126, 35),
        (123, 34), (123, 36), (122, 33), (122, 37), (124, 33), (124, 37)
    ];
    let middleweight_spaceship = vec![
        (130, 50), (131, 50), (132, 50), (133, 50), (134, 50),
        (130, 51), (134, 51), (134, 52), (134, 53), (133, 54), (130, 54), (132, 54)
    ];
    let heavyweight_spaceship = vec![
        (140, 50), (141, 50), (142, 50), (143, 50), (144, 50), 
        (140, 51), (144, 51), (144, 52), (144, 53), (143, 54), (140, 54), (141, 54), (142, 54)
    ];

    let mut initial_state = Vec::new();
    initial_state.extend(glider);
    initial_state.extend(small_exploder);
    initial_state.extend(exploder);
    initial_state.extend(lightweight_spaceship);
    initial_state.extend(block);
    initial_state.extend(bee_hive);
    initial_state.extend(loaf);
    initial_state.extend(boat);
    initial_state.extend(tub);
    initial_state.extend(blinker);
    initial_state.extend(beacon);
    initial_state.extend(pulsar);
    initial_state.extend(pentadecathlon);
    initial_state.extend(middleweight_spaceship);
    initial_state.extend(heavyweight_spaceship);

    let mut game = GameOfLife::new(framebuffer_width, framebuffer_height, initial_state);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        game.step();
        game.render(&mut framebuffer);
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
