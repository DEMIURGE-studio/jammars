use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let mut rules = sequence![
        one![BB* / BBB / *B* > *** / *I* / ***],
        all![*I* / IBI > *** / *I*],
        all![*B* / BIB / *B* > *** / *W* / ***],
        repeat![2, one![I > E]],
        markov![
            all![EI > *E],
            one![EBI / EBI > **E / **E],
        ],
        all![E*W > **E],
        all![I > B, W > B],
    ];
    visualize::runner(&mut rules);
}