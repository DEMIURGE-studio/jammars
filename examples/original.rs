use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let (width, height) = visualize::get_term_size();
    let mut grid = Grid::new(glam::uvec2(width, height), "BAC");
    let mut rules = sequence![
        one![R:RBB > RBR],
        one![RBRB > RURB],
        one![R > U],
        one![UUU / UBU / UUU > UUU / UUU / UUU],
        one![R:RU > RR],
        one![U > B],
    ];
    visualize::runner(&mut grid, &mut rules);
}