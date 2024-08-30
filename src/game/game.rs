use rand::Rng;

use crate::{framebuffer::prelude::PlatformFramebuffer, Framebuffer};
use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

const SET_COLOR: u32 = 0xFFFFFF;
const UNSET_COLOR: u32 = 0x000000;

enum CurrentGrid {
    Grid1,
    Grid2,
}

pub struct Game {
    fb: Rc<PlatformFramebuffer>,

    grid_1: Box<[bool]>,
    grid_2: Box<[bool]>,
    current_grid: CurrentGrid,

    grid_width: u16,
    grid_height: u16,

    tile_pixel_size: u16,

    sim_update_ms: u64,
    last_sim_update_ms: u64,
}

impl Game {
    pub fn new(
        fb: Rc<PlatformFramebuffer>,
        width: u16,
        height: u16,
        tile_pixel_size: u16,
        sim_update_ms: u64,
    ) -> Self {
        let grid_1 = vec![false; (width * height) as usize].into_boxed_slice();
        let grid_2 = grid_1.clone();

        Self {
            fb,
            grid_1,
            grid_2,
            current_grid: CurrentGrid::Grid1,
            grid_width: width,
            grid_height: height,
            tile_pixel_size,
            sim_update_ms,
            last_sim_update_ms: 0,
        }
    }

    pub fn generate(&mut self) {
        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let is_alive = rand::thread_rng().gen_bool(0.15);
                self.set_tile(is_alive, x, y);
            }
        }
    }

    fn swap_grids(&mut self) {
        let (current_grid, next_grid) = match self.current_grid {
            CurrentGrid::Grid1 => (&self.grid_1, &mut self.grid_2),
            CurrentGrid::Grid2 => (&self.grid_2, &mut self.grid_1),
        };

        next_grid.copy_from_slice(current_grid);

        self.current_grid = match self.current_grid {
            CurrentGrid::Grid1 => CurrentGrid::Grid2,
            CurrentGrid::Grid2 => CurrentGrid::Grid1,
        };
    }

    #[inline(always)]
    fn calc_grid_idx(&self, x: u16, y: u16) -> usize {
        let idx = (y * self.grid_width + x) as usize;
        debug_assert!(idx < self.grid_1.len());
        idx
    }

    fn current_millis() -> u64 {
        let start = SystemTime::now();
        start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }

    #[inline(always)]
    fn get_current_grid(&self) -> &[bool] {
        match self.current_grid {
            CurrentGrid::Grid1 => &self.grid_1,
            CurrentGrid::Grid2 => &self.grid_2,
        }
    }

    #[inline(always)]
    fn get_current_grid_mut(&mut self) -> &mut [bool] {
        match self.current_grid {
            CurrentGrid::Grid1 => &mut self.grid_1,
            CurrentGrid::Grid2 => &mut self.grid_2,
        }
    }

    #[inline(always)]
    fn get_tile(&self, x: u16, y: u16) -> bool {
        self.get_current_grid()[self.calc_grid_idx(x, y)]
    }

    fn set_tile(&mut self, is_alive: bool, x: u16, y: u16) {
        let idx = self.calc_grid_idx(x, y);
        self.get_current_grid_mut()[idx] = is_alive;

        let color = if is_alive { SET_COLOR } else { UNSET_COLOR };

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
        let current_time = Self::current_millis();

        if self.last_sim_update_ms + self.sim_update_ms > current_time {
            return;
        }

        self.last_sim_update_ms = current_time;

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

        self.swap_grids();
    }
}
