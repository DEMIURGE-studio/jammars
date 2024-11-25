use ndarray::prelude::*;

use chain::*;

mod performance;

const WIDTH: usize = 100;
const HEIGHT: usize = 25;

fn main() {
    let mut grid = Grid::new(Vec2 { x: WIDTH, y: HEIGHT }, "BRGW");
    let mut rule = standard![
        one![
            rule![
                array![['R', 'B', 'B']],
                array![['G', 'G', 'R']],
                origin='R',
            ],
        ],
    ];
    let mut count = 0;
    if !rule.apply(&mut grid) {
        count += 1;
    }
    for ((x, y), tile) in grid.tiles.indexed_iter() {
        #[cfg(feature = "colors")]
        {
            let [r, g, b] = alphabet_color(*tile);
            print!("\x1B[{};{}f\x1B[48;2;{};{};{}m ", y + 1, x + 1, r, g, b);
        }
        #[cfg(not(feature = "colors"))]
        print!("\x1B[{};{}f{}", y + 1, x + 1, tile);
    }
}

fn _main() {
    
    #[cfg(not(feature = "seeded"))]
    let mut grid = Grid::new(
        Vec2 { x: WIDTH, y: HEIGHT },
        "BRGW",
    );
    let mut rules = standard![
        rules!(
            array![['R', 'B', 'B']],
            array![['G', 'G', 'R']],
            origin='R',
        ),
        rules!(
            array![['R', 'G', 'G']],
            array![['W', 'W', 'R']],
        ),
    ];

    print!("\x1B[2J");
    let ups = performance::UpdatesCounter::new();
    loop {
        rules.apply(&mut grid);
        print!("\x1B[27;0f{} Updates Per Second\x1B[H", ups.update());
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Some(cell) = grid.get(x, y) {
                    #[cfg(feature = "colors")]
                    {
                        let [r, g, b] = alphabet_color(*cell);
                        print!("\x1B[{};{}f\x1B[48;2;{};{};{}m ", y + 1, x + 1, r, g, b);
                    }
                    #[cfg(not(feature = "colors"))]
                    print!("\x1B[{};{}f{}", y + 1, x + 1, cell);
                }
            }
            print!("\n");
        }
        print!("\x1B[0m");
    }
    print!("\nTook {:.2?}", ups.start.elapsed());
    print!("\nAverage {:.2?} Microseconds Per Iteration", ups.start.elapsed().as_micros() as f32 / 10000.);
}
