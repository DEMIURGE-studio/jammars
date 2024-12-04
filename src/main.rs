use ndarray::prelude::*;
use std::time::{Duration, Instant};

use jammars::*;

mod performance;

const WIDTH: u32 = 212;
const HEIGHT: u32 = 52;

fn main() {
    let ups = performance::UpdatesCounter::new();
    let mut last = Instant::now();
    let mut grid = Grid::new(glam::uvec2(WIDTH, HEIGHT), "BAC");
    /*let mut rules = sequence![
        repeat![5, one![B > W]],
        repeat![5, one![B > R]],
        one![RB > RR, WB > WW],
        all![RW > UU],
        all![W > B, R > B],
        repeat![1, all![UB > UU]],
        all![BU / UB > U* / **],
        all![UB > *G],
        repeat![13, one![B > E]],
        one![EB > *E, GB > *G],
    ];
    let mut rules = sequence![
        one![R:RBB > RBR],
        one![RBRB > RURB],
        one![R > U],
        one![UUU / UBU / UUU > UUU / UUU / UUU],
        one![R:RU > RR],
        one![U > B],
    ];*/
    let mut rules = sequence![
        one![;*B* > *A*],
        one![;BA > UU, ;AB > GG],
        one![;UA > UU, ;AG > GG],
        one![U / G > B / B],
        one![G > U],
        one![BU / UB > BB / BB],
        all![B / U > B / B],
        all![B / U > B / B],
        all![*B* / BUB > *** / *B*],
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
