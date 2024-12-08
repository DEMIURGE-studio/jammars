use std::time::{Duration, Instant};
use terminal_size::{Width, Height, terminal_size};

pub mod performance;

pub fn runner(rules: &mut jammars::Rules) {
    let (width, height) = if let Some((Width(w), Height(h))) = terminal_size() {
        (w as u32, h as u32)
    } else {
        (100, 100)
    };
    let ups = performance::UpdatesCounter::new();
    let mut last = Instant::now();
    let mut grid = jammars::Grid::new(glam::uvec2(width, height), "B");
    print!("\x1B[?47h\x1B[?25l\x1B[2J");
    let mut temp = grid.tiles.clone();
    loop {
        if last.elapsed() >= Duration::from_millis(1) {
            print!("\x1B[0m\x1B[{};0f{} Updates Per Second\x1B[H", height - 2, ups.update());
            last = Instant::now();
            if !rules.apply(&mut grid) {
                break;
            }
        }
        for ((x, y), tile) in grid.tiles.indexed_iter() {
            if let Some(old) = temp.get((x, y)) {
                if tile != old {
                    let [r, g, b] = jammars::alphabet_color(*tile);
                    print!("\x1B[{};{}f\x1B[48;2;{};{};{}m ", y + 1, x + 1, r, g, b);
                }
            }
        }
        temp = grid.tiles.clone();
    }
    print!("\x1B[?47l\x1B[?25h\x1B[{};0fTook {:.2?}", height - 1, ups.start.elapsed());
}