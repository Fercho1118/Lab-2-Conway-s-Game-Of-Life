use raylib::prelude::*;
use crate::framebuffer::FrameBuffer;

pub struct GameOfLife {
    width: usize,
    height: usize,
    current: Vec<Vec<bool>>,
    next: Vec<Vec<bool>>,
    pub generation: u32,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let current = vec![vec![false; height]; width];
        let next = vec![vec![false; height]; width];
        Self { width, height, current, next, generation: 0 }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width && y < self.height {
            self.current[x][y] = alive;
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = ((x as isize + dx + self.width as isize) % self.width as isize) as usize;
                let ny = ((y as isize + dy + self.height as isize) % self.height as isize) as usize;
                if self.current[nx][ny] { count += 1; }
            }
        }
        count
    }

    pub fn update(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.count_neighbors(x, y);
                let alive = self.current[x][y];
                self.next[x][y] = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        std::mem::swap(&mut self.current, &mut self.next);
        self.generation += 1;
    }

    pub fn render(&self, framebuffer: &mut FrameBuffer) {
        for x in 0..self.width {
            for y in 0..self.height {
                let color = if self.current[x][y] { Color::WHITE } else { Color::BLACK };
                framebuffer.set_pixel(x as i32, y as i32, color);
            }
        }
    }

    pub fn setup_creative_patterns(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.current[x][y] = false;
            }
        }

        let w = self.width;
        let h = self.height;

        //Centro: Cruz de Pulsars 
        let px = w / 2 - 6;
        let py = h / 2 - 6;
        self.add_pulsar(px, py); 
        self.add_pulsar(px - 16, py);
        self.add_pulsar(px + 16, py);
        self.add_pulsar(px, py - 16);
        self.add_pulsar(px, py + 16);

        //Bordes con Still Lifes
        for i in 0..(w / 10) {
            self.add_block(i * 10, 2);
            self.add_beehive(i * 10 + 3, h - 5);
            self.add_loaf(i * 10 + 5, h / 2);
        }

        for i in 0..(h / 10) {
            self.add_boat(2, i * 10);
            self.add_tub(w - 5, i * 10);
        }

        //Osciladores distribuidos
        for i in 0..(w / 10) {
            self.add_blinker(i * 10, h / 4);
            self.add_beacon(i * 10, h / 4 + 10);
            self.add_toad(i * 10, h / 4 + 20);
        }

        //Pentadecathlons en el centro horizontal
        for i in 0..4 {
            self.add_pentadecathlon(w / 2 - 20 + i * 10, 4);
        }

        //Espacioships (gliders y LWSS)
        self.add_glider(3, 3);
        self.add_glider(w - 6, 3);
        self.add_glider(3, h - 6);
        self.add_glider(w - 6, h - 6);

        self.add_lwss(5, h / 2 - 5);
        self.add_lwss(w - 15, h / 2 + 2);

        self.add_mwss(10, h / 3);
        self.add_hwss(w - 16, h / 3 * 2);
    }

    //Patrones

    fn add_block(&mut self, x: usize, y: usize) {
        let p = [(0, 0), (1, 0), (0, 1), (1, 1)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_beehive(&mut self, x: usize, y: usize) {
        let p = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_loaf(&mut self, x: usize, y: usize) {
        let p = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_boat(&mut self, x: usize, y: usize) {
        let p = [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_tub(&mut self, x: usize, y: usize) {
        let p = [(1, 0), (0, 1), (2, 1), (1, 2)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_blinker(&mut self, x: usize, y: usize) {
        let p = [(0, 0), (0, 1), (0, 2)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_beacon(&mut self, x: usize, y: usize) {
        let p = [(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_toad(&mut self, x: usize, y: usize) {
        let p = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_pulsar(&mut self, x: usize, y: usize) {
        let p = [
            (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
            (0,2),(5,2),(7,2),(12,2),
            (0,3),(5,3),(7,3),(12,3),
            (0,4),(5,4),(7,4),(12,4),
            (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
            (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
            (0,8),(5,8),(7,8),(12,8),
            (0,9),(5,9),(7,9),(12,9),
            (0,10),(5,10),(7,10),(12,10),
            (2,12),(3,12),(4,12),(8,12),(9,12),(10,12)
        ];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_pentadecathlon(&mut self, x: usize, y: usize) {
        let p = [
            (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1),
            (1, 0), (6, 0), (1, 2), (6, 2),
        ];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_glider(&mut self, x: usize, y: usize) {
        let p = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_lwss(&mut self, x: usize, y: usize) {
        let p = [(1,0),(4,0),(0,1),(0,2),(4,2),(0,3),(1,3),(2,3),(3,3)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_mwss(&mut self, x: usize, y: usize) {
        let p = [(1,0),(2,0),(3,0),(4,0),(0,1),(4,1),(4,2),(0,3),(3,3)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }

    fn add_hwss(&mut self, x: usize, y: usize) {
        let p = [(1,0),(2,0),(3,0),(4,0),(5,0),(0,1),(5,1),(5,2),(0,3),(4,3)];
        for (dx, dy) in p { self.set_cell(x + dx, y + dy, true); }
    }
}
