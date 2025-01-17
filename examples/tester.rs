use jammars::*;

fn main() {
    let mut rules = one![W:WBB > WAW];
    if let Rules::One(v) = rules {
        let mut grammar = v[0].pattern.clone();
        for y in 0..grammar.find.height() {
            for x in 0..grammar.find.width() {
                if let Some(c) = grammar.find.get(x, y) {
                    print!("{}", c);
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
        grammar.rotate(Rotation::Clockwise);
        for y in 0..grammar.find.height() {
            for x in 0..grammar.find.width() {
                if let Some(c) = grammar.find.get(x, y) {
                    print!("{}", c);
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
    }
}