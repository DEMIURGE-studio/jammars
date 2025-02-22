use jammars::*;

mod visualize;

fn main() {
    let mut rules = sequence![
        one![;*B* > *A*],
        one![;BA > UU, ;AB > GG],
        one![;UA > UU, ;AG > GG],
        one![U / G > A / A],
        one![G > U],
        one![AU / UA > AA / AA],
        steps![2, all![A / U > A / A]],
        all![*A* / AUA > *** / *A*],
        one![;A / * > B / *],
        markov![
            steps![1, rules![rule![;UUAUU > BBRBB]]],
            rules![rule![x;UA > UU]],
        ],
    ];
    visualize::runner(&mut rules);
}