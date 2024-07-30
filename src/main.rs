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
            resize: false,
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    ).unwrap();

    framebuffer.set_background_color(0x99CCFF);
    framebuffer.set_current_color(0xFFFF99);

    let glider = vec![(1, 2), (2, 2), (3, 2), (3, 1), (2, 0)];
    let small_exploder = vec![
        (5, 5), (6, 5), (7, 5), (4, 6), (8, 6), (5, 7), (7, 7), (6, 8),
    ];
    let exploder = vec![
        (10, 10), (11, 10), (12, 10), (13, 10), (14, 10),
        (10, 12), (14, 12),
        (10, 14), (11, 14), (12, 14), (13, 14), (14, 14),
    ];
    let lightweight_spaceship = vec![
        (20, 20), (21, 20), (22, 20), (23, 20), 
        (20, 21), (20, 23), 
        (23, 21), (23, 22), (22, 23)
    ];

    let block = vec![(30, 30), (31, 30), (30, 31), (31, 31)];
    let bee_hive = vec![(33, 30), (34, 29), (35, 29), (36, 30), (35, 31), (34, 31)];
    let loaf = vec![(38, 30), (39, 29), (40, 29), (41, 30), (41, 31), (40, 32), (39, 31)];
    let boat = vec![(43, 30), (44, 30), (43, 31), (44, 32), (42, 31)];
    let tub = vec![(46, 30), (47, 31), (46, 32), (45, 31)];
    let blinker = vec![(50, 30), (50, 31), (50, 32)];
    let beacon = vec![(55, 30), (56, 30), (55, 31), (57, 32), (58, 32), (57, 33)];
    let pulsar = vec![
        (60, 25), (61, 25), (62, 25), (60, 29), (61, 29), (62, 29),
        (60, 27), (62, 27),
        (58, 27), (58, 26), (58, 28),
        (64, 27), (64, 26), (64, 28),
        (65, 25), (66, 25), (67, 25), (65, 29), (66, 29), (67, 29),
        (65, 27), (67, 27)
    ];
    let pentadecathlon = vec![
        (70, 25), (71, 25), (72, 25), (74, 25), (75, 25), (76, 25),
        (73, 24), (73, 26), (72, 23), (72, 27), (74, 23), (74, 27)
    ];
    let middleweight_spaceship = vec![
        (30, 40), (31, 40), (32, 40), (33, 40), (34, 40),
        (30, 41), (34, 41), (34, 42), (34, 43), (33, 44), (30, 44), (32, 44)
    ];
    let heavyweight_spaceship = vec![
        (50, 40), (51, 40), (52, 40), (53, 40), (54, 40), 
        (50, 41), (54, 41), (54, 42), (54, 43), (53, 44), (50, 44), (51, 44), (52, 44)
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
