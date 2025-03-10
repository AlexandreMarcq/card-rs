#[derive(Clone, Debug)]
pub enum UnoColor {
    Blue,
    Green,
    Red,
    Yellow,
}

pub struct UnoCard {
    color: UnoColor,
    value: u8,
}

impl std::fmt::Debug for UnoCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}, {}>",
            match self.color {
                UnoColor::Blue => "B",
                UnoColor::Red => "R",
                UnoColor::Green => "G",
                UnoColor::Yellow => "Y",
            },
            self.value,
        )
    }
}

impl UnoCard {
    pub fn new(value: u8, color: UnoColor) -> Self {
        UnoCard { value, color }
    }
}
