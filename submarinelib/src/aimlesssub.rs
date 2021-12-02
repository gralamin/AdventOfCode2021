use crate::movement::SubPosition;

#[derive(Debug)]
pub struct AimlessSub {
    pub horizontal_pos: i32,
    pub depth: i32,
}

impl SubPosition for AimlessSub {
    fn forward(&mut self, by: i32) {
        self.horizontal_pos += by;
    }

    fn down(&mut self, by: i32) {
        self.depth += by;
    }

    fn up(&mut self, by: i32) {
        self.depth -= by;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aimless_sub_movement() {
        let mut sub = AimlessSub {
            horizontal_pos: 0,
            depth: 0,
        };
        sub.forward(5);
        assert_eq!(sub.horizontal_pos, 5);
        sub.down(5);
        assert_eq!(sub.depth, 5);
        sub.forward(8);
        assert_eq!(sub.horizontal_pos, 13);
        sub.up(3);
        assert_eq!(sub.depth, 2);
        sub.down(8);
        assert_eq!(sub.depth, 10);
        sub.forward(2);
        assert_eq!(sub.horizontal_pos, 15);
    }
}
