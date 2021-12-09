use day09::extract_all_nums;
use day09::get_board_size;
use day09::load;
use day09::puzzle_a;
use day09::puzzle_b;

fn main() {
    let filename = "input";
    let input = load(filename);
    let (width, height) = get_board_size(&input);
    //assert_eq!(width, 100);
    //assert_eq!(height, 100);
    let numbers = extract_all_nums(&input);
    let numbers_b = numbers.clone();
    let len: i32 = numbers.len().try_into().unwrap();
    assert_eq!(len, width * height);

    let value_a = puzzle_a(numbers, width, height);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(numbers_b, width, height);
    println!("Solution to 2: {}", value_b);
}
