use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let mut rules = sequence![
        all![W:WBB > **I, IBB > **I],
        markov![
            one![RBI > KKR, RBK > GKY, RBW > WWW, WKK > WWW, YKG > YBU, UKK > IBU, UKY > IBR],
            one![I > R],
        ],
    ];
    visualize::runner(&mut rules);
}