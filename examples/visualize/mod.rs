use std::time::{Duration, Instant};

mod performance;

pub fn runner(grid: &mut jammars::Grid, rules: &mut jammars::Rules) {
    let ups = performance::UpdatesCounter::new();
    let mut last = Instant::now();
    print!("\x1B[?47h\x1B[?25l\x1B[2J");
    let mut temp = grid.tiles.clone();
    loop {
        if last.elapsed() >= Duration::from_millis(1) {
            print!("\x1B[0m\x1B[26;0f{} Updates Per Second\x1B[H", ups.update());
            last = Instant::now();
            if !rules.apply(grid) {
                break;
            }
        }
        for ((x, y), tile) in grid.tiles.indexed_iter() {
            if let Some(old) = temp.get((x, y)) {
                if tile != old {
                    #[cfg(feature = "colors")]
                    {
                        let [r, g, b] = jammars::alphabet_color(*tile);
                        print!("\x1B[{};{}f\x1B[48;2;{};{};{}m ", y + 1, x + 1, r, g, b);
                    }
                    #[cfg(not(feature = "colors"))]
                    print!("\x1B[{};{}f{}", y + 1, x + 1, tile);
                }
            }
        }
        temp = grid.tiles.clone();
    }
    print!("\x1B[?47l\x1B[?25h\x1B[27;0fTook {:.2?}", ups.start.elapsed());
}

pub fn get_term_size() -> (u32, u32) {
    use terminal_size::{Width, Height, terminal_size};

    if let Some((Width(w), Height(h))) = terminal_size() {
        (w as u32, h as u32)
    } else {
        (100, 100)
    }
}