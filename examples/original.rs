use jammars::*;

mod visualize;

fn main() {
    let mut rules = sequence![
        one![R:RBB > RBR],
        all![RBRB > RURB],
        all![R > U],
        all![UUU / UBU / UUU > UUU / UUU / UUU],
        one![R:RU > RR],
        all![U > B],
    ];
    visualize::runner(&mut rules);
}