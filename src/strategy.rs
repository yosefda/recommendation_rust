/// Trait that defines recommendation strategy.

pub trait Strategy {
    type Output;
    fn get_recommendations(&self) -> Option<Self::Output>;
}