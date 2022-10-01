#[macro_export]
macro_rules! vec2 {
    () => {
        Vec::new()
    };

    // , is just the seperator
    // + means 1 or more separate expressions.
    // * means 0 or more.
    // $(,)? means that after every expression 0 or 1 commas can be present.
    ($($element:expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        $(vs.push($element);)*
        vs
    }};
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
fn double_vec() {
    let x: Vec<u32> = vec2![42, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing_comma_vec() {
    let x: Vec<u32> = vec2![1, 2, 3, 4, 5, 6, 7,];
}