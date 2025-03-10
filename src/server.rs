pub trait GameState {
    type Event;
    type Error;

    fn validate(&self, event: &Self::Event) -> Result<(), Self::Error>;
    fn consume(&mut self, event: &Self::Event);

    fn next(&mut self, event: &Self::Event) -> Result<(), Self::Error> {
        self.validate(event)?;
        self.consume(event);
        Ok(())
    }
}
