use jammars::*;
use ndarray::array;

use std::time::{Duration, Instant};
use terminal_size::{Width, Height, terminal_size};

pub fn main() {
    let (width, height) = if let Some((Width(w), Height(h))) = terminal_size() {
        (w as u32, h as u32)
    } else {
        (100, 100)
    };
    let mut last = Instant::now();
    let mut grid = jammars::Grid::new(width, height, "B");
    // Initialize the red tiles
    while all![R:RBB > RBR].apply(&mut grid) {}
    // Fill in the initial maze
    while all![RBRB > RURB].apply(&mut grid) {}
    steps![1, one![R > W]].apply(&mut grid);
    print!("\x1B[?25l\x1B[2J");
    loop {
        if last.elapsed() >= Duration::from_millis(1) {
            last = Instant::now();
            // First, we attempt to move the white tile to a red tile
            if !rules![rule![WR > UW]].apply(&mut grid) {
                // If we can't move white to a red tile, we try to move it to a blue tile
                rules![rule![WU > RW]].apply(&mut grid);
            }
            // Fill in the maze again
            all![RBRB > RURB].apply(&mut grid);
        }
        if let Some((cx, cy)) = find_white(&mut grid) {
            for ((x, y), tile) in grid.tiles.indexed_iter_mut() {
                if !in_circle(cx, cy, x, y, 12) {
                    if *tile != 'R' {
                        *tile = 'B';
                    }
                }
                if in_circle(cx, cy, x, y, 14) {
                    let [r, g, b] = jammars::alphabet_color(*tile);
                    print!("\x1B[{};{}f\x1B[48;2;{};{};{}m ", y + 1, x + 1, r, g, b);
                }
            }
        };
    }
}

fn wipe_circle(grid: &mut Grid) -> bool {
    // Extremely bad performance, but it is just a proof of concept
    // Locate red tile
    let Some((cx, cy)) = find_white(grid) else {
        return false;
    };

    // Set all tiles with distance larger than 10 away from red tile
    for ((x, y), tile) in grid.tiles.indexed_iter_mut() {
        if !in_circle(cx, cy, x, y, 12) {
            *tile = 'B';
        }
    }
    // a bool has to be returned...
    false
}

fn in_circle(x1: usize, y1: usize, x2: usize, y2: usize, r: usize) -> bool {
    let dx = (x1 as i32 - x2 as i32).abs() as usize;
    let dy = (y1 as i32 - y2 as i32).abs() as usize;
    dx * dx + dy * dy <= r * r
}

fn find_white(grid: &mut Grid) -> Option<(usize, usize)> {
    for ((x, y), tile) in grid.tiles.indexed_iter() {
        if *tile == 'W' {
            return Some((x, y));
        }
    }
    None
}