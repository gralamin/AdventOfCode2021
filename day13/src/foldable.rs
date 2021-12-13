use boardlib::{Board, BoardCoordinate, BoardTraversable};

#[derive(Debug)]
pub struct FoldablePaper {
    board: Board<bool>,
}

impl FoldablePaper {
    pub fn new(base_values: Vec<bool>, base_width: usize, base_height: usize) -> FoldablePaper {
        return FoldablePaper {
            board: Board::new(base_width, base_height, base_values),
        };
    }

    pub fn fold_horizontal(&self, y_to_fold: usize) -> FoldablePaper {
        let new_width = self.get_width();
        // folding takes out a row its on
        // On an equal fold, its just one half
        // If there isn't an equal fold, we should panic later for now.
        let new_height = self.get_height() / 2;
        let mut new_values: Vec<bool> = Vec::new();

        // When we fold, the distance between the y_to_fold and the y of each point remains constant
        // and X remains constant.
        for coord in self.board.coord_iter() {
            if coord.y >= y_to_fold {
                // skip over these
                continue;
            } else {
                let distance = y_to_fold - coord.y;
                let matching_coord = BoardCoordinate::new(coord.x, y_to_fold + distance);
                if let Some(unfolded_v) = self.board.get_value(matching_coord) {
                    if let Some(folded_v) = self.board.get_value(coord) {
                        new_values.push(unfolded_v || folded_v);
                    } else {
                        panic!("Couldn't find folded value I'm on");
                    }
                } else {
                    panic!("Couldn't find unfolded value - might be unequal fold");
                }
            }
        }

        return FoldablePaper::new(new_values, new_width, new_height);
    }

    pub fn fold_vertical(&self, x_to_fold: usize) -> FoldablePaper {
        // folding takes out a col its on
        // On an equal fold, its just one half
        // If there isn't an equal fold, we should panic later for now.
        let new_width = self.get_width() / 2;
        let new_height = self.get_height();
        let mut new_values: Vec<bool> = Vec::new();

        // When we fold, the distance between the x_to_fold and the x of each point remains constant
        // and y remains constant.
        for coord in self.board.coord_iter() {
            if coord.x >= x_to_fold {
                // skip over these, already handled.
                continue;
            } else {
                let distance = x_to_fold - coord.x;
                let matching_coord = BoardCoordinate::new(x_to_fold + distance, coord.y);
                if let Some(unfolded_v) = self.board.get_value(matching_coord) {
                    if let Some(folded_v) = self.board.get_value(coord) {
                        new_values.push(unfolded_v || folded_v);
                    } else {
                        panic!("Couldn't find folded value I'm on");
                    }
                } else {
                    panic!("Couldn't find unfolded value - might be unequal fold");
                }
            }
        }

        return FoldablePaper::new(new_values, new_width, new_height);
    }

    pub fn get_num_dots(&self) -> usize {
        let mut count: usize = 0;
        for coord in self.board.coord_iter() {
            if let Some(value) = self.board.get_value(coord) {
                if value {
                    count += 1;
                }
            }
        }
        return count;
    }

    pub fn get_width(&self) -> usize {
        return self.board.get_width();
    }

    pub fn get_height(&self) -> usize {
        return self.board.get_height();
    }

    pub fn fancy_print(&self) -> String {
        let mut string_parts: Vec<&str> = Vec::new();
        let mut last_y = 0;
        for coord in self.board.coord_iter() {
            if coord.y > last_y {
                string_parts.push("\n");
            }
            last_y = coord.y;
            if self.board.get_value(coord).unwrap() {
                string_parts.push("█");
            } else {
                string_parts.push(" ");
            }
        }
        return string_parts.iter().map(|x| *x).collect::<String>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coords_to_bool;
    use rustc_hash::FxHashSet;

    fn make_paper() -> FoldablePaper {
        let height = 15;
        let width = 11;
        let mut dot_coords: FxHashSet<BoardCoordinate> = FxHashSet::default();
        dot_coords.insert(BoardCoordinate::new(6, 10));
        dot_coords.insert(BoardCoordinate::new(0, 14));
        dot_coords.insert(BoardCoordinate::new(9, 10));
        dot_coords.insert(BoardCoordinate::new(0, 3));
        dot_coords.insert(BoardCoordinate::new(10, 4));
        dot_coords.insert(BoardCoordinate::new(4, 11));
        dot_coords.insert(BoardCoordinate::new(6, 0));
        dot_coords.insert(BoardCoordinate::new(6, 12));
        dot_coords.insert(BoardCoordinate::new(4, 1));
        dot_coords.insert(BoardCoordinate::new(0, 13));
        dot_coords.insert(BoardCoordinate::new(10, 12));
        dot_coords.insert(BoardCoordinate::new(3, 4));
        dot_coords.insert(BoardCoordinate::new(3, 0));
        dot_coords.insert(BoardCoordinate::new(8, 4));
        dot_coords.insert(BoardCoordinate::new(1, 10));
        dot_coords.insert(BoardCoordinate::new(2, 14));
        dot_coords.insert(BoardCoordinate::new(8, 10));
        dot_coords.insert(BoardCoordinate::new(9, 0));
        let values = coords_to_bool(&dot_coords, width, height);
        return FoldablePaper::new(values, width, height);
    }

    #[test]
    fn test_get_num_dots() {
        let paper = make_paper();
        assert_eq!(paper.get_num_dots(), 18);
    }

    #[test]
    fn test_fold_vertically() {
        let paper = make_paper();
        let folded_paper = paper.fold_horizontal(7);
        assert_eq!(folded_paper.get_num_dots(), 17);
        assert_eq!(folded_paper.get_width(), 11);
        assert_eq!(folded_paper.get_height(), 7);
    }

    #[test]
    fn test_fold_vertically_then_horizontally() {
        let paper = make_paper();
        let folded_paper = paper.fold_horizontal(7);
        let twice_folded = folded_paper.fold_vertical(5);
        assert_eq!(twice_folded.get_num_dots(), 16);
        assert_eq!(twice_folded.get_width(), 5);
        assert_eq!(twice_folded.get_height(), 7);
    }
}
