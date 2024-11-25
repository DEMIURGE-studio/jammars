/// Reimplementation of MarkovJunior's logic in rust, following [notes](https://gist.github.com/dogles/a926ab890552cc7e45400a930398449d).
use ndarray::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
use std::cell::Cell;

#[macro_use]
mod macros;

/// Rules is a tree structure where different nodes perform different types of operations and/or
/// influence which of their child nodes are executed at any point.
/// 
/// Each step asks the currently executing node to apply it's transformation, or
/// return false if the node has more work to do.
#[derive(Clone)]
pub enum Rules {
    /// Rewrite rule
    Rule(Rule),
    Custom(fn(grid: &mut Grid) -> bool),

    /// `One`` takes a set of rewrite rules.
    /// Each step, it will find all rules that have at least one match on the grid, and apply a random match to apply.
    /// `One` will end if there are no rules left that have any matches.
    One(Vec<Rule>, usize),
    All(Vec<Rule>, usize, Option<Vec<Match>>),

    Standard(Vec<Rules>),
    Sequence(Vec<Rules>, usize),
    Repeat(usize, Box<Rules>),

    // TODO: start, end, path
    //Path(char, char, char),
}

impl Rules {
    pub fn apply(&mut self, grid: &mut Grid) -> bool {
        match self {
            Self::Rule(rule) => {
                let mut matches = grid.find(rule.pattern.clone(), &rule.symmetry);
                rule.apply(grid, &mut matches)
            },
            Self::Custom(_rule) => _rule(grid),
            Self::One(rules, index) => {
                // TODO: Fix by cloning the rules first, shuffling, and removing from the cloned version to allow knowing when no more matches can be found
                if let Some(mut rule) = rules.choose(&mut grid.rng).cloned() {
                    let mut matches = grid.find(rule.pattern.clone(), &rule.symmetry);
                    if rule.apply(grid, &mut matches) {
                        if *index < rules.len() - 1 {
                            *index += 1;
                        } else {
                            *index = 0;
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            Self::All(rules, index, matches) => {
                // TODO: Fix this by initializing the matches with for loop so that all matches for all rules at this time are accounted for
                let mut rule = rules[*index].clone();
                match matches {
                    None => {
                        *matches = Some(grid.find(rule.pattern.clone(), &rule.symmetry));
                        if let Some(matches) = matches {
                            rule.apply(grid, matches)
                        } else {
                            false
                        }
                    },
                    Some(matches) => {
                        rule.apply(grid, matches)
                    },
                }
            },
            Self::Standard(rules) => {
                for rule in rules {
                    if rule.apply(grid) {
                        return true;
                    }
                }
                false
            },
            Self::Sequence(rules, index) => {
                if rules[*index].apply(grid) {
                    if *index < rules.len() {
                        *index += 1;
                        return true;
                    } else {
                        return false;
                    }
                }
                true
            },
            Self::Repeat(repeat, rules) => {
                if *repeat > 0 {
                    if rules.apply(grid) {
                        *repeat -= 1;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
        }
    }
}

#[derive(Clone)]
pub struct Rule {
    pub pattern: Pattern,
    pub origin: Cell<char>,
    pub symmetry: Vec<usize>,
}

impl Rule {
    pub fn apply(&mut self, grid: &mut Grid, matches: &mut Vec<Match>) -> bool {
        if self.origin.get() != ' ' {
            let x = grid.size.x / 2;
            let y = grid.size.y / 2;
            if let Some(tile) = grid.get_mut(x, y) {
                *tile = self.origin.get();
                self.origin.replace(' ');
            }
        }
        if !matches.is_empty() {
            let i = grid.rng.gen_range(0..matches.len());
            let choice = matches.remove(i);
            //self.pattern.rotate(choice.rot);
            if grid.fits(choice.pos, &self.pattern) {
                grid.replace(choice.pos, &self.pattern);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub struct Grid {
    pub alphabet: Vec<char>,
    pub size: Vec2,
    pub tiles: Array2<char>,
    pub rng: ChaChaRng,
}

impl Grid {
    pub fn new(size: Vec2, alphabet: &str) -> Self {
        let alphabet: Vec<char> = alphabet.to_uppercase().chars().collect();
        let start = alphabet[0];
        Self {
            alphabet, size,
            tiles: Array2::from_elem((size.x, size.y), start),
            rng: ChaChaRng::from_entropy(),
        }
    }

    pub fn find(&self, mut pattern: Pattern, symmetry: &Vec<usize>) -> Vec<Match> {
        let mut results = Vec::new();
        let mut rotations = vec![Rotation::None];
        if symmetry.contains(&0) {
            rotations.push(Rotation::Mirror);
        }
        if symmetry.contains(&1) {
            rotations.push(Rotation::Clockwise);
            rotations.push(Rotation::Counter);
        }
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                for rotation in &rotations {
                    pattern.rotate(*rotation);
                    if self.fits(Vec2 { x, y }, &pattern) {
                        results.push(Match {
                            pat: pattern.clone(),
                            pos: Vec2 { x, y },
                        });
                    }
                }
            }
        }
        results
    }

    pub fn fits(&self, pos: Vec2, pattern: &Pattern) -> bool {
        let mut matching = true;
        for ((x, y), &find) in pattern.find.indexed_iter() {
            if let Some(&tile) = self.get(pos.x + x, pos.y + y) {
                if tile != find && !(tile == '*' || find == '*') {
                    matching = false;
                    break;
                }
            } else {
                matching = false;
                break;
            }
        }
        matching
    }

    pub fn replace(&mut self, pos: Vec2, pattern: &Pattern) {
        for ((x, y), &replace) in pattern.replace.indexed_iter() {
            if let Some(tile) = self.get_mut(pos.x + x, pos.y + y) {
                if replace != '*' {
                    *tile = replace;
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.tiles.get((x, y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut char> {
        self.tiles.get_mut((x, y))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

/// Instance of a matching pattern on a grid
#[derive(Clone, Debug)]
pub struct Match {
    /// The Rule which this is a match for
    pub pat: Pattern,
    /// The position of the top left corner of match
    pub pos: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rotation {
    Clockwise, // 90
    Counter, // 270
    Mirror, // 180
    None, // 0
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Char {
    pub cell: char,
    pub rgb: [u8; 3],
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub current: Rotation,
    pub find: Array2<char>,
    pub replace: Array2<char>,
}

impl Pattern {
    pub fn rotate(&mut self, rotation: Rotation) {
        use Rotation::*;
        self._rotate(match (self.current, rotation) {
            (None, new) => new,
            (Clockwise, None) => Counter,
            (Clockwise, Mirror) => Clockwise,
            (Clockwise, Counter) => Mirror,
            (Mirror, None) => Mirror,
            (Mirror, Clockwise) => Counter,
            (Mirror, Counter) => Clockwise,
            (Counter, None) => Clockwise,
            (Counter, Clockwise) => Mirror,
            (Counter, Mirror) => Counter,
            _ => None,
        });
        self.current = rotation;
    }

    fn _rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Clockwise => {
                self.find.swap_axes(0, 1);
                self.replace.swap_axes(0, 1);
                self.find.invert_axis(Axis(0));
                self.replace.invert_axis(Axis(0));
            },
            Rotation::Counter => {
                self.find.swap_axes(0, 1);
                self.replace.swap_axes(0, 1);
                self.find.invert_axis(Axis(1));
                self.replace.invert_axis(Axis(1));
            },
            Rotation::Mirror => {
                self.find.invert_axis(Axis(1));
                self.replace.invert_axis(Axis(1));
                self.find.invert_axis(Axis(0));
                self.replace.invert_axis(Axis(0));
            },
            Rotation::None => {},
        }
    }
}

pub fn alphabet_color(cell: char) -> [u8; 3] {
    match cell {
        'B' => [0x00, 0x00, 0x00], //Black
        'I' => [0x1D, 0x2B, 0x53], //Indigo
        'P' => [0x7E, 0x25, 0x53], //Purple
        'E' => [0x00, 0x87, 0x51], //Emerald
        'N' => [0xAB, 0x52, 0x36], //browN
        'D' => [0x5F, 0x57, 0x4F], //Dead
        'A' => [0xC2, 0xC3, 0xC7], //Alive
        'W' => [0xFF, 0xF1, 0xE8], //White
        'R' => [0xFF, 0x00, 0x4D], //Red
        'O' => [0xFF, 0xA3, 0x00], //Orange
        'Y' => [0xFF, 0xEC, 0x27], //Yellow
        'G' => [0x00, 0xE4, 0x36], //Green
        'U' => [0x29, 0xAD, 0xFF], //blUe
        'S' => [0x83, 0x76, 0x9C], //Slate
        'K' => [0xFF, 0x77, 0xA8], //pinK
        'F' => [0xFF, 0xCC, 0xAA], //Fawn
        'C' => [0x00, 0xFF, 0xFF], //Cyan
        'H' => [0xE4, 0xBB, 0x40], //Honey
        'J' => [0x4B, 0x69, 0x2F], //Jungle
        'L' => [0x84, 0x7E, 0x87], //Light
        'M' => [0xFF, 0x00, 0xFF], //Magenta
        'Q' => [0x9B, 0xAD, 0xB7], //aQua
        'T' => [0x37, 0x94, 0x6E], //Teal
        'V' => [0x8F, 0x97, 0x4A], //oliVe
        'X' => [0xFF, 0x00, 0x00], //X
        'Z' => [0xFF, 0xFF, 0xFF], //Z
        _ => [0xFF, 0xFF, 0xFF],
    }
}