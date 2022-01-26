
/// Trait used by [Sequence] operations to merge two entities
pub trait Merge<T> {
    /// Merge this [T] with another
    fn merge(&self, other: &T) -> T;
}
