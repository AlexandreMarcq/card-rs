#[derive(Clone, Debug, PartialEq)]
pub enum UnoColor {
    Blue,
    Green,
    Red,
    Yellow,
}

#[derive(Clone, PartialEq)]
pub struct UnoCard {
    pub color: UnoColor,
    pub value: u8,
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
