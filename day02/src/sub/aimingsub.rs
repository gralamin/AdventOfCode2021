use crate::sub::movement::SubPosition;

#[derive(Debug)]
pub struct AimingSub {
    pub horizontal_pos: i32,
    pub depth: i32,
    pub aim: i32,
}

impl SubPosition for AimingSub {
    fn forward(&mut self, by: i32) {
        self.horizontal_pos += by;
        self.depth += self.aim * by;
    }

    fn down(&mut self, by: i32) {
        self.aim += by;
    }

    fn up(&mut self, by: i32) {
        self.aim -= by;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aiming_sub_movement() {
        let mut sub = AimingSub {
            horizontal_pos: 0,
            depth: 0,
            aim: 0,
        };
        sub.forward(5);
        assert_eq!(sub.horizontal_pos, 5);
        assert_eq!(sub.depth, 0);
        sub.down(5);
        assert_eq!(sub.aim, 5);
        sub.forward(8);
        assert_eq!(sub.horizontal_pos, 13);
        assert_eq!(sub.depth, 40);
        sub.up(3);
        assert_eq!(sub.aim, 2);
        sub.down(8);
        assert_eq!(sub.aim, 10);
        sub.forward(2);
        assert_eq!(sub.horizontal_pos, 15);
        assert_eq!(sub.depth, 60);
    }
}
