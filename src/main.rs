use minifb::{Window, WindowOptions, Key};

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
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x000000);
    framebuffer.set_current_color(0xFFFFFF);

    // Patrón inicial (puedes cambiar esto)
    let initial_state = vec![(1, 2), (2, 2), (3, 2)]; // Glider pattern
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
