#[macro_export]
macro_rules! rules {
    ($find:expr, $replace:expr$(,)?) => {
        Rules::Rule(Rule {
            pattern: Pattern {
                find: $find,
                replace: $replace,
                current: Rotation::None,
            },
            origin: std::cell::Cell::new(' '),
            symmetry: vec![0, 1],
        })
    };
    ($find:expr, $replace:expr$(, origin=$origin:expr)?$(, symmetry=$symmetry:expr)?$(,)?) => {
        Rules::Rule(Rule {
            pattern: Pattern {
                find: $find,
                replace: $replace,
                current: Rotation::None,
            },
            origin: [$(std::cell::Cell::new($origin),)? std::cell::Cell::new(' ')][0].clone(),
            symmetry: [$($symmetry,)? vec![0, 1]][0].clone(),
        })
    };
}

#[macro_export]
macro_rules! rule {
    ($find:expr, $replace:expr$(,)?) => {
        Rule {
            pattern: Pattern {
                find: $find,
                replace: $replace,
                current: Rotation::None,
            },
            origin: std::cell::Cell::new(' '),
            symmetry: vec![0, 1],
        }
    };
    ($find:expr, $replace:expr$(, origin=$origin:expr)?$(, symmetry=$symmetry:expr)?$(,)?) => {
        Rule {
            pattern: Pattern {
                find: $find,
                replace: $replace,
                current: Rotation::None,
            },
            origin: [$(std::cell::Cell::new($origin),)? std::cell::Cell::new(' ')][0].clone(),
            symmetry: [$($symmetry,)? vec![0, 1]][0].clone(),
        }
    };
}

#[macro_export]
macro_rules! standard {
    ($($rule:expr),+ $(,)?) => {
        Rules::Standard(vec![
            $( $rule, )*
        ])
    };
}

#[macro_export]
macro_rules! sequence {
    ($($rule:expr),+ $(,)?) => {
        Rules::Sequence(vec![
            $( $rule, )*
        ], 0)
    };
}

#[macro_export]
macro_rules! one {
    ($($rule:expr),+ $(,)?) => {
        Rules::One(vec![
            $( $rule, )*
        ],)
    };
}

#[macro_export]
macro_rules! all {
    ($($rule:expr),+ $(,)?) => {
        Rules::All(vec![
            $( $rule, )*
        ], None)
    };
}

#[macro_export]
macro_rules! repeat {
    ($limit:expr, $rule:expr $(,)?) => {
        Rules::Repeat($limit, Box::new($rule))
    };
}

#[macro_export]
macro_rules! path {
    ($start:expr, $end:expr $(,)?) => {
        Rules::Path($start, $end, None)
    };
    ($start:expr, $end:expr, $path:expr $(,)?) => {
        Rules::Path($start, $end, Some($path))
    };
}

#[macro_export]
macro_rules! custom {
    ($rule_fn:path) => {
        Rules::Custom($rule_fn)
    };
}