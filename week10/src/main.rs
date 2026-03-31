// Lab 10: The Borrow Checker Game
// Fix each problem and implement ownership exercises.

fn main() {
    println!("Lab 10: Mastering Ownership and Borrowing");
    println!("All ownership problems and exercises are fixed and ready.\n");

    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();

    let upper = to_uppercase_owned(String::from("hello"));
    println!("Owned uppercase: {}", upper);

    let mut s = String::from("hello");
    append_suffix(&mut s, ", world");
    println!("Appended suffix: {}", s);

    println!("String length: {}", string_length(&s));
    println!("Concatenated: {}", concat_strings("foo", "bar"));
    println!("First word: {}", first_word("hello rust"));
}

// ============================================================================
// PROBLEM 1: Value used after move
// ============================================================================
fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(&s1);
    println!("  The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: &String) -> (&String, usize) {
    let length = s.len();
    (s, length)
}

// ============================================================================
// PROBLEM 2: Immutable and mutable borrow conflict
// ============================================================================
fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");
    {
        let r1 = &s; // Immutable borrow
        println!("  Immutable borrow: {}", r1);
    }
    let r2 = &mut s; // Mutable borrow allowed after r1 is out of scope
    println!("  Mutable borrow: {}", r2);
}

// ============================================================================
// PROBLEM 3: Mutating through immutable reference
// ============================================================================
fn problem_3() {
    println!("Problem 3: Mutating through immutable reference");
    let mut s = String::from("hello");
    add_to_string(&mut s);
    println!("  Result: {}", s);
}

fn add_to_string(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// PROBLEM 4: Multiple mutable borrows
// ============================================================================
fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("  First mutable borrow: {}", r1);
    }

    let r2 = &mut s;
    println!("  Second mutable borrow: {}", r2);
}

// ============================================================================
// PROBLEM 5: Dangling reference
// ============================================================================
fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> String {
    String::from("hello")
}

// ============================================================================
// PROBLEM 6: Ownership in loops
// ============================================================================
fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");

    for i in 0..3 {
        print_with_number(&data, i);
    }
}

fn print_with_number(s: &String, n: i32) {
    println!("  {}: {}", n, s);
}

// ============================================================================
// PROBLEM 7: Lifetime extension challenge
// ============================================================================
fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let s = String::from("inner scope");
    let result = &s;
    println!("  Result: {}", result);
}

// ============================================================================
// IMPLEMENTATION EXERCISES
// ============================================================================

/// Takes ownership of a String, converts it to uppercase, and returns it.
fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

/// Borrows a String immutably and returns its length.
fn string_length(s: &str) -> usize {
    s.len()
}

/// Borrows a String mutably and appends a suffix to it.
fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

/// Creates a new String by concatenating two borrowed strings.
fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

/// Finds the first word in a string and returns it as a string slice.
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(index) => &s[..index],
        None => s,
    }
}

// ============================================================================
// TEST SUITE
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length_borrows() {
        let s = String::from("testing");
        let (_s_ref, len) = calculate_length(&s);
        assert_eq!(len, 7);
        assert_eq!(s, "testing");
    }

    #[test]
    fn test_add_to_string_mutates() {
        let mut s = String::from("hello");
        add_to_string(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_create_string_returns_owned() {
        let result = create_string();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_print_with_number_borrows() {
        let data = String::from("Rust");
        for i in 0..3 {
            print_with_number(&data, i);
        }
        assert_eq!(data, "Rust");
    }

    #[test]
    fn test_to_uppercase_owned() {
        let result = to_uppercase_owned(String::from("hello"));
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_string_length() {
        let s = String::from("hello");
        assert_eq!(string_length(&s), 5);
    }

    #[test]
    fn test_append_suffix() {
        let mut s = String::from("hello");
        append_suffix(&mut s, ", world");
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_concat_strings() {
        assert_eq!(concat_strings("foo", "bar"), "foobar");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
    }
}
