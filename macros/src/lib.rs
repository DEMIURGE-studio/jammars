use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn rule(input: TokenStream) -> TokenStream {
    let mut input = input.to_string();
    let mut find = vec![];
    let mut replace = vec![];
    let mut current = &mut find;

    let mut row = vec![];

    let origin = if let Some((v, i)) = input.clone().split_once(':') {
        // Idea: allow multiple chars as origin, when setting origin, select random.
        input = i.to_string();
        let mut vals = v.chars();
        let first = vals.next();
        if let Some('"') = first {
            vals.next().unwrap()
        } else {
            first.unwrap()
        }
    } else {
        ' '
    };

    let symmetry = if let Some((v, i)) = input.clone().split_once(';') {
        // Idea: allow multiple chars as origin, when setting origin, select random.
        input = i.to_string();
        let mut res = vec![];
        for c in v.to_uppercase().chars() {
            match c {
                'X' => res.push(0usize),
                'Y' => res.push(1usize),
                'Z' => res.push(2usize),
                _ => {},
            }
        }
        res
    } else {
        // Future proofing when 3D arrays are supported
        // This should indicate default behavior
        // All symmetry enabled by default
        // Regardless of which dimension we are currently working in
        vec![4usize]
    };

    for c in input.to_uppercase().chars() {
        match c {
            'A'..'Z' | '0'..'9' | '*' => {
                row.push(quote!{ #c });
            },
            '/' => {
                current.push(quote!{[#(#row),*]});
                row.clear();
            },
            '>' => {
                current.push(quote!{[#(#row),*]});
                row.clear();
                current = &mut replace;
            },
            // Ignore
            ' ' | '"' => {},
            x => panic!("Unexpected input `{}`", x),
        }
    }

    if !row.is_empty() {
        current.push(quote!{[#(#row),*]});
    }

    let output = quote!{
        Rule {
            pattern: Pattern {
                current: jammars::Rotation::None,
                find: array![#(#find),*],
                replace: array![#(#replace),*],
            },
            origin: #origin,
            symmetry: vec![#(#symmetry),*],
        }
    };
    
    TokenStream::from(output)
}

#[proc_macro]
pub fn one(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut rules = vec![];
    
    for rule in input.split(',') {
        rules.push(quote!{
            rule!(#rule)
        });
    }

    let output = quote!{
        Rules::One(vec![
            #(#rules),*
        ])
    };

    TokenStream::from(output)
}

#[proc_macro]
pub fn all(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut rules = vec![];
    
    for rule in input.split(',') {
        rules.push(quote!{
            rule!(#rule)
        });
    }

    let output = quote!{
        Rules::All(vec![
            #(#rules),*
        ], None)
    };

    TokenStream::from(output)
}