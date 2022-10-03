#[macro_export]
macro_rules! vec2 {
    () => {
        Vec::new()
    };

    // , is just the seperator
    // + means 1 or more separate expressions.
    // * means 0 or more.
    // $(,)? means that after every expression 0 or 1 commas can be present.
    ($($value:expr),+) => {{
        let mut vs = Vec::new();
        $(vs.push($value);)*
        vs
    }};

    ($value:expr; $amount:expr) => {{
        let mut vs = Vec::with_capacity($amount);
        for i in 0..$amount {
            vs.push($value);
        }

        vs
    }}
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = vec2![];
    assert!(x.is_empty());
}

#[test]
fn single_vec() {
    let x: Vec<u32> = vec2![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn triple_vec() {
    let x: Vec<u32> = vec2![42, 43, 44];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
    assert_eq!(x[2], 44)
}

#[test]
fn trailing_comma_vec() {
    let x: Vec<u32> = vec2![1, 2, 3, 4, 5, 6, 7];
}

#[test]
fn auto_generate_vec_of_size_with_value() {
    let x: Vec<u32> = vec2!(5; 3);

    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 5);
    assert_eq!(x[1], 5);
    assert_eq!(x[2], 5);
}