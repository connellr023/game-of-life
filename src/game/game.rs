use rand::Rng;

use crate::{framebuffer::prelude::PlatformFramebuffer, Framebuffer};
use std::{
    rc::Rc,
    time::{Duration, Instant},
};

enum GridSelector {
    Grid1,
    Grid2,
}

pub struct Game {
    fb: Rc<PlatformFramebuffer>,

    grid_1: Box<[bool]>,
    grid_2: Box<[bool]>,
    current_grid: GridSelector,

    grid_width: u16,
    grid_height: u16,

    tile_pixel_size: u16,

    update_interval: Duration,
    last_update: Instant,
    start_time: Instant,
}

impl Game {
    pub fn new(
        fb: Rc<PlatformFramebuffer>,
        width: u16,
        height: u16,
        tile_pixel_size: u16,
        update_interval: Duration,
    ) -> Self {
        let size = width as usize * height as usize;

        Self {
            fb,
            grid_1: vec![false; size].into_boxed_slice(),
            grid_2: vec![false; size].into_boxed_slice(),
            current_grid: GridSelector::Grid1,
            grid_width: width,
            grid_height: height,
            tile_pixel_size,
            update_interval,
            last_update: Instant::now(),
            start_time: Instant::now(),
        }
    }

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> u32 {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = match h {
            0.0..=60.0 => (c, x, 0.0),
            60.0..=120.0 => (x, c, 0.0),
            120.0..=180.0 => (0.0, c, x),
            180.0..=240.0 => (0.0, x, c),
            240.0..=300.0 => (x, 0.0, c),
            300.0..=360.0 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let r = ((r + m) * 255.0) as u32;
        let g = ((g + m) * 255.0) as u32;
        let b = ((b + m) * 255.0) as u32;

        (r << 16) | (g << 8) | b
    }

    fn gradient(&self, x: u16, y: u16, shift: f32) -> u32 {
        let hue_x = x as f32 / self.grid_width as f32;
        let hue_y = y as f32 / self.grid_height as f32;
        let hue = ((hue_x + hue_y + shift) % 1.0) * 360.0;

        Self::hsv_to_rgb(hue, 1.0, 1.0)
    }

    pub fn generate(&mut self) {
        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let is_alive = rand::thread_rng().gen_bool(0.04);
                self.set_tile(is_alive, x, y);
            }
        }
    }

    fn swap_grids(&mut self) {
        let (current_grid, next_grid) = match self.current_grid {
            GridSelector::Grid1 => (&self.grid_1, &mut self.grid_2),
            GridSelector::Grid2 => (&self.grid_2, &mut self.grid_1),
        };

        next_grid.copy_from_slice(current_grid);

        self.current_grid = match self.current_grid {
            GridSelector::Grid1 => GridSelector::Grid2,
            GridSelector::Grid2 => GridSelector::Grid1,
        };
    }

    #[inline(always)]
    fn calc_grid_idx(&self, x: u16, y: u16) -> usize {
        let idx = (y as usize * self.grid_width as usize) + x as usize;
        debug_assert!(idx < self.grid_1.len());
        idx
    }

    #[inline(always)]
    fn get_current_grid(&self) -> &[bool] {
        match self.current_grid {
            GridSelector::Grid1 => &self.grid_1,
            GridSelector::Grid2 => &self.grid_2,
        }
    }

    #[inline(always)]
    fn get_next_grid(&self) -> &[bool] {
        match self.current_grid {
            GridSelector::Grid1 => &self.grid_2,
            GridSelector::Grid2 => &self.grid_1,
        }
    }

    #[inline(always)]
    fn get_current_grid_mut(&mut self) -> &mut [bool] {
        match self.current_grid {
            GridSelector::Grid1 => &mut self.grid_1,
            GridSelector::Grid2 => &mut self.grid_2,
        }
    }

    #[inline(always)]
    fn get_tile(&self, x: u16, y: u16) -> bool {
        self.get_current_grid()[self.calc_grid_idx(x, y)]
    }

    fn set_tile(&mut self, is_alive: bool, x: u16, y: u16) {
        let idx = self.calc_grid_idx(x, y);
        self.get_current_grid_mut()[idx] = is_alive;
    }

    fn render_tile(&self, color: u32, x: u16, y: u16) {
        for i in 0..self.tile_pixel_size {
            for j in 0..self.tile_pixel_size {
                self.fb.write_pixel(
                    (x * self.tile_pixel_size + i) as u32,
                    (y * self.tile_pixel_size + j) as u32,
                    color,
                );
            }
        }
    }

    fn count_alive_neighbors(&self, x: u16, y: u16) -> u8 {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let x = x as i32 + i;
                let y = y as i32 + j;

                if x < 0 || x >= self.grid_width as i32 || y < 0 || y >= self.grid_height as i32 {
                    continue;
                }

                if self.get_tile(x as u16, y as u16) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        if now.duration_since(self.last_update) >= self.update_interval {
            self.swap_grids();
            self.last_update = now;

            for x in 0..self.grid_width {
                for y in 0..self.grid_height {
                    let alive_neighbors = self.count_alive_neighbors(x, y);
                    let is_alive = self.get_tile(x, y);

                    if is_alive {
                        if alive_neighbors < 2 || alive_neighbors > 3 {
                            self.set_tile(false, x, y);
                        }
                    } else if alive_neighbors == 3 {
                        self.set_tile(true, x, y);
                    }
                }
            }
        }
    }

    pub fn render(&self) {
        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let idx = self.calc_grid_idx(x, y);
                let is_alive = self.get_next_grid()[idx];

                let next_color = if is_alive {
                    let elapsed = self.start_time.elapsed().as_secs_f32();
                    let shift = elapsed * 0.7;

                    self.gradient(x, y, shift)
                } else {
                    0x000000
                };

                self.render_tile(next_color, x, y);
            }
        }
    }
}
