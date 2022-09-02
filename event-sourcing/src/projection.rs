pub trait Projection: Sized + Send + Sync + Clone {
    type Event: Send + Sync + Clone;

    fn apply(event: Self::Event) -> Result<(), Box<dyn std::error::Error>>;
}
