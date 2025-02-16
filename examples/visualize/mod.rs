use rand::SeedableRng;
use terminal_size::{Width, Height, terminal_size};
use wyrand::WyRand;
use jammars::Grid;

pub mod performance;

struct DefaultGrid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<char>,
}

impl Grid for DefaultGrid {
    const DEPTH: usize = 2;

    fn bounds(&self) -> Vec<usize> {
        vec![self.width, self.height]
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.tiles.get(y * self.width + x).copied()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut char> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.tiles.get_mut(y * self.width + x)
    }
}

pub fn runner(rules: &mut jammars::Rules) {
    let (width, height) = if let Some((Width(w), Height(h))) = terminal_size() {
        (w as u32, h as u32)
    } else {
        (100, 100)
    };
    let ups = performance::UpdatesCounter::new();
    let mut grid = DefaultGrid {
        width: width as usize,
        height: height as usize,
        tiles: vec!['B'; width as usize * height as usize],
    };
    let mut rng = WyRand::from_os_rng();
    print!("\x1B[?47h\x1B[?25l\x1B[2J");
    let mut temp = grid.tiles.clone();
    loop {
        print!("\x1B[0m\x1B[{};0f{} Updates Per Second\x1B[H", height - 2, ups.update());
        if !rules.apply(&mut grid, &mut rng) {
            break;
        }
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                if let Some(tile) = grid.get(x, y) {
                    if let Some(old) = temp.get(y * grid.width() + x) {
                        if tile != *old {
                            let [r, g, b] = jammars::alphabet_color(tile);
                            print!("\x1B[{};{}f\x1B[48;2;{};{};{}m ", y + 1, x + 1, r, g, b);
                        }
                    }
                }
            }
        }
        temp = grid.tiles.clone();
    }
    print!("\x1B[?47l\x1B[?25h\x1B[{};0fTook {:.2?}", height - 1, ups.start.elapsed());
}