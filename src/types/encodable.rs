pub trait Encodable<T> {
    fn encode(&self) -> T;
}