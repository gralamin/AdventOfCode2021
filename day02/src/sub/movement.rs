pub trait SubPosition {
    fn forward(&mut self, by: i32);
    fn down(&mut self, by: i32);
    fn up(&mut self, by: i32);
}
