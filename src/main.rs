use ndarray::prelude::*;
use std::time::{Duration, Instant};

use jammars::*;

mod performance;

const WIDTH: usize = 212;
const HEIGHT: usize = 52;

fn main() {
    let ups = performance::UpdatesCounter::new();
    let mut last = Instant::now();
    let mut grid = Grid::new(Vec2 { x: WIDTH, y: HEIGHT }, "BWRUGE");
    /*
    let mut rules = sequence![
        repeat![5, one![
            rule![array![['B']], array![['W']],],
        ]],
        repeat![5, one![
            rule![array![['B']], array![['R']],],
        ]],
        one![
            rule![array![['R', 'B']], array![['R', 'R']],],
            rule![array![['W', 'B']], array![['W', 'W']],],
        ],
        all![
            rule![array![['R', 'W']], array![['U', 'U']],],
        ],
        all![
            rule![array![['W']], array![['B']],],
            rule![array![['R']], array![['B']],],
        ],
        repeat![1, all![
            rule![array![['U', 'B']], array![['U', 'U']],],
        ]],
        all![
            rule![array![['B', 'U'], ['U', 'B']], array![['U', '*'], ['*', '*']],],
        ],
        all![
            rule![array![['U', 'B']], array![['*', 'G']],],
        ],
        repeat![13, one![
            rule![array![['B']], array![['E']],],
        ]],
        one![
            rule![array![['E', 'B']], array![['*', 'E']],],
            rule![array![['G', 'B']], array![['*', 'G']],],
        ],
    ];
    */
    let mut rules = standard![
        one![rule![array![['R', 'B', 'B']], array![['R', 'B', 'R']], origin='R']],
        one![rule![array![['R', 'B', 'R', 'B']], array![['R', 'A', 'R', 'B']]]],
        one![rule![array![['R']], array![['A']]]],
        one![rule![
            array![['A', 'A', 'A'], ['A', 'B', 'A'], ['A', 'A', 'A']],
            array![['A', 'A', 'A'], ['A', 'A', 'A'], ['A', 'A', 'A']],
        ]],
    ];
    print!("\x1B[2J");
    let mut temp = grid.tiles.clone();
    loop {
        if last.elapsed() >= Duration::from_millis(1) {
            print!("\x1B[0m\x1B[26;0f{} Updates Per Second\x1B[H", ups.update());
            last = Instant::now();
            if !rules.apply(&mut grid) {
                break;
            }
        }
        for ((x, y), tile) in grid.tiles.indexed_iter() {
            if let Some(old) = temp.get((x, y)) {
                if tile != old {
                    #[cfg(feature = "colors")]
                    {
                        let [r, g, b] = alphabet_color(*tile);
                        print!("\x1B[{};{}f\x1B[48;2;{};{};{}m ", y + 1, x + 1, r, g, b);
                    }
                    #[cfg(not(feature = "colors"))]
                    print!("\x1B[{};{}f{}", y + 1, x + 1, tile);
                }
            }
        }
        temp = grid.tiles.clone();
    }
    /*while rules.apply(&mut grid) {}
    for ((x, y), tile) in grid.tiles.indexed_iter() {
        #[cfg(feature = "colors")]
        {
            let [r, g, b] = alphabet_color(*tile);
            print!("\x1B[{};{}f\x1B[48;2;{};{};{}m ", y + 1, x + 1, r, g, b);
        }
        #[cfg(not(feature = "colors"))]
        print!("\x1B[{};{}f{}", y + 1, x + 1, tile);
    }*/
    print!("\x1B[27;0fTook {:.2?}", ups.start.elapsed());
}
