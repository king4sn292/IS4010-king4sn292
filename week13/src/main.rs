use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

fn main() {
    println!("Lab 13: Idiomatic Rust\n");

    let text = "Rust is fast, safe, and productive.";
    let (count, avg, longest) = analyze_text(text);
    println!(
        "Text analysis: count={}, average_length={:.2}, longest=\"{}\"",
        count, avg, longest
    );

    let numbers = [1, 2, 3, 4, 5, 6];
    println!("Processed numbers result: {}", process_numbers(&numbers));

    let mut counter = make_counter();
    println!("Counter calls: {}, {}, {}", counter(), counter(), counter());

    let tree = BinaryTree::node(
        10,
        BinaryTree::leaf(5),
        BinaryTree::node(15, BinaryTree::leaf(12), BinaryTree::leaf(20)),
    );
    println!("Binary tree: {:?}", tree);

    let empty_tree: BinaryTree<i32> = BinaryTree::new();
    println!("Empty tree: {:?}", empty_tree);

    demonstrate_rc();
    demonstrate_refcell();

    println!("10 / 2 = {:?}", divide(10.0, 2.0));
    println!("10 / 0 = {:?}\n", divide(10.0, 0.0));

    println!("Parse positive: {:?}", parse_positive_number("42"));
    println!("Parse invalid: {:?}\n", parse_positive_number("abc"));

    let config = Rc::new(RefCell::new(Config {
        min_length: 3,
        max_length: 10,
    }));
    let lines = vec!["rust".to_string(), "is".to_string(), "awesome".to_string()];

    match process_lines(&lines, Rc::clone(&config)) {
        Ok(output) => println!("Processed lines: {:?}", output),
        Err(e) => println!("Processing error: {}", e),
    }
}

fn analyze_text(text: &str) -> (usize, f64, String) {
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return (0, 0.0, String::new());
    }

    let word_count = words.len();
    let total_length: usize = words.iter().map(|word| word.len()).sum();
    let average_length = total_length as f64 / word_count as f64;
    let longest_word = words
        .iter()
        .fold(words[0], |longest, &word| {
            if word.len() > longest.len() {
                word
            } else {
                longest
            }
        })
        .to_string();

    (word_count, average_length, longest_word)
}

fn process_numbers(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum()
}

fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

#[derive(Debug, PartialEq)]
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree::Empty
    }

    fn leaf(value: T) -> Self {
        BinaryTree::Node {
            value,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }
    }

    fn node(value: T, left: BinaryTree<T>, right: BinaryTree<T>) -> Self {
        BinaryTree::Node {
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
struct SharedData {
    value: i32,
}

fn demonstrate_rc() {
    let data = Rc::new(SharedData { value: 42 });
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);
    println!("Rc strong count after clones: {}", Rc::strong_count(&data));
    println!("Shared value: {}", owner1.value);
    drop(owner2);
    println!("Rc strong count after drop: {}", Rc::strong_count(&data));
}

#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Rc<RefCell<Counter>> {
        Rc::new(RefCell::new(Counter { value: 0 }))
    }

    fn increment(counter: &Rc<RefCell<Counter>>) {
        counter.borrow_mut().value += 1;
    }

    fn get_value(counter: &Rc<RefCell<Counter>>) -> i32 {
        counter.borrow().value
    }
}

fn demonstrate_refcell() {
    let counter = Counter::new();
    let counter_ref1 = Rc::clone(&counter);
    let counter_ref2 = Rc::clone(&counter);

    Counter::increment(&counter_ref1);
    Counter::increment(&counter_ref2);
    println!("Counter value: {}", Counter::get_value(&counter));
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ParseError {
    EmptyInput,
    InvalidNumber(String),
    OutOfRange(i32),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Input string is empty"),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::OutOfRange(n) => write!(f, "Number out of range: {}", n),
        }
    }
}

fn parse_positive_number(input: &str) -> Result<i32, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let num: i32 = input
        .trim()
        .parse()
        .map_err(|_| ParseError::InvalidNumber(input.to_string()))?;

    if !(1..=100).contains(&num) {
        return Err(ParseError::OutOfRange(num));
    }

    Ok(num)
}

#[derive(Debug, Clone)]
struct Config {
    min_length: usize,
    max_length: usize,
}

#[derive(Debug)]
enum ProcessError {
    LineTooShort(String),
    LineTooLong(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessError::LineTooShort(line) => write!(f, "Line too short: {}", line),
            ProcessError::LineTooLong(line) => write!(f, "Line too long: {}", line),
        }
    }
}

fn process_lines(
    lines: &[String],
    config: Rc<RefCell<Config>>,
) -> Result<Vec<String>, ProcessError> {
    lines
        .iter()
        .map(|line| {
            let cfg = config.borrow();
            let len = line.len();

            if len < cfg.min_length {
                Err(ProcessError::LineTooShort(line.clone()))
            } else if len > cfg.max_length {
                Err(ProcessError::LineTooLong(line.clone()))
            } else {
                Ok(line.to_uppercase())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_text_basic() {
        let result = analyze_text("hello world");
        assert_eq!(result.0, 2);
        assert!((result.1 - 5.0).abs() < 0.01);
        assert_eq!(result.2, "hello");
    }

    #[test]
    fn test_analyze_text_empty() {
        assert_eq!(analyze_text(""), (0, 0.0, String::new()));
    }

    #[test]
    fn test_process_numbers() {
        assert_eq!(process_numbers(&[1, 2, 3, 4, 5, 6]), 56);
        assert_eq!(process_numbers(&[1, 3, 5]), 0);
        assert_eq!(process_numbers(&[]), 0);
    }

    #[test]
    fn test_counter_closure() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_binary_tree_creation() {
        let tree = BinaryTree::node(5, BinaryTree::leaf(3), BinaryTree::leaf(7));
        match tree {
            BinaryTree::Node { value, .. } => assert_eq!(value, 5),
            _ => panic!("Expected Node"),
        }
    }

    #[test]
    fn test_rc_reference_counting() {
        let data = Rc::new(SharedData { value: 42 });
        assert_eq!(Rc::strong_count(&data), 1);

        let _owner1 = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);

        let _owner2 = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 3);
    }

    #[test]
    fn test_refcell_mutation() {
        let counter = Counter::new();
        assert_eq!(Counter::get_value(&counter), 0);
        Counter::increment(&counter);
        assert_eq!(Counter::get_value(&counter), 1);
        Counter::increment(&counter);
        assert_eq!(Counter::get_value(&counter), 2);
    }

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(9.0, 3.0), Ok(3.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_parse_positive_number_valid() {
        assert_eq!(parse_positive_number("50"), Ok(50));
        assert_eq!(parse_positive_number("1"), Ok(1));
        assert_eq!(parse_positive_number("100"), Ok(100));
    }

    #[test]
    fn test_parse_positive_number_errors() {
        assert!(matches!(
            parse_positive_number(""),
            Err(ParseError::EmptyInput)
        ));
        assert!(matches!(
            parse_positive_number("abc"),
            Err(ParseError::InvalidNumber(_))
        ));
        assert!(matches!(
            parse_positive_number("150"),
            Err(ParseError::OutOfRange(150))
        ));
    }

    #[test]
    fn test_process_lines_success() {
        let config = Rc::new(RefCell::new(Config {
            min_length: 3,
            max_length: 10,
        }));
        let lines = vec!["hello".to_string(), "world".to_string()];
        let result = process_lines(&lines, config);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec!["HELLO".to_string(), "WORLD".to_string()]
        );
    }

    #[test]
    fn test_process_lines_too_short() {
        let config = Rc::new(RefCell::new(Config {
            min_length: 5,
            max_length: 10,
        }));
        let lines = vec!["hi".to_string()];
        let result = process_lines(&lines, config);
        assert!(result.is_err());
    }
}
