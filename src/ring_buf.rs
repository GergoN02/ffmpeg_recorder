#[derive(Debug)]
pub struct RingBuffer<T> {
    start: usize,
    data: Vec<T>,
}
