extern crate filelib;

pub use filelib::load;
pub use filelib::parse_csv_i32_lines;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy() {
        assert_eq(1 + 1, 2)
    }
}