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
        (10, 5), (11, 5), (12, 5), (9, 6), (13, 6), (10, 7), (12, 7), (11, 8),
    ];
    let exploder = vec![
        (20, 10), (21, 10), (22, 10), (23, 10), (24, 10),
        (20, 12), (24, 12),
        (20, 14), (21, 14), (22, 14), (23, 14), (24, 14),
    ];
    let lightweight_spaceship = vec![
        (30, 20), (31, 20), (32, 20), (33, 20), 
        (30, 21), (30, 23), 
        (33, 21), (33, 22), (32, 23)
    ];

    let block = vec![(40, 30), (41, 30), (40, 31), (41, 31)];
    let bee_hive = vec![(43, 30), (44, 29), (45, 29), (46, 30), (45, 31), (44, 31)];
    let loaf = vec![(48, 30), (49, 29), (50, 29), (51, 30), (51, 31), (50, 32), (49, 31)];
    let boat = vec![(53, 30), (54, 30), (53, 31), (54, 32), (52, 31)];
    let tub = vec![(56, 30), (57, 31), (56, 32), (55, 31)];
    let blinker = vec![(60, 30), (60, 31), (60, 32)];
    let beacon = vec![(65, 30), (66, 30), (65, 31), (67, 32), (68, 32), (67, 33)];
    let pulsar = vec![
        (70, 25), (71, 25), (72, 25), (70, 29), (71, 29), (72, 29),
        (70, 27), (72, 27),
        (68, 27), (68, 26), (68, 28),
        (74, 27), (74, 26), (74, 28),
        (75, 25), (76, 25), (77, 25), (75, 29), (76, 29), (77, 29),
        (75, 27), (77, 27)
    ];
    let pentadecathlon = vec![
        (80, 25), (81, 25), (82, 25), (84, 25), (85, 25), (86, 25),
        (83, 24), (83, 26), (82, 23), (82, 27), (84, 23), (84, 27)
    ];
    let middleweight_spaceship = vec![
        (90, 40), (91, 40), (92, 40), (93, 40), (94, 40),
        (90, 41), (94, 41), (94, 42), (94, 43), (93, 44), (90, 44), (92, 44)
    ];
    let heavyweight_spaceship = vec![
        (100, 40), (101, 40), (102, 40), (103, 40), (104, 40), 
        (100, 41), (104, 41), (104, 42), (104, 43), (103, 44), (100, 44), (101, 44), (102, 44)
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
