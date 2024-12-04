#[macro_export]
macro_rules! rules {
    ($rule:expr $(,)?) => {
        Rules::Rule(
            $rule,
        )
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