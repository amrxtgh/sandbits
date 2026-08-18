#[derive(Debug)]
pub struct Buffer<T> {
    data: Vec<T>,
}
impl<T> Buffer<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}
