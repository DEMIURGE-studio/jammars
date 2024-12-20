#[macro_export]
macro_rules! rules {
    ($rule:expr $(,)?) => {
        Rules::Rule(
            $rule,
        )
    };
}

#[macro_export]
macro_rules! markov {
    ($($rule:expr),+ $(,)?) => {
        Rules::Markov(vec![
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
macro_rules! steps {
    ($limit:expr, $rule:expr $(,)?) => {
        Rules::Steps($limit, $limit, Box::new($rule))
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