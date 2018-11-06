use std::rc::Rc;

use widget::WidgetContainer;

use {Event, EventBox, EventHandler};

#[derive(Copy, Clone)]
pub enum Key {
    Unknown,
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Space,
    Enter,
    A(bool),
    B(bool),
    C(bool),
    D(bool),
    E(bool),
    F(bool),
    G(bool),
    H(bool),
    I(bool),
    J(bool),
    K(bool),
    L(bool),
    M(bool),
    N(bool),
    O(bool),
    P(bool),
    Q(bool),
    S(bool),
    R(bool),
    T(bool),
    U(bool),
    V(bool),
    W(bool),
    X(bool),
    Y(bool),
    Z(bool),
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Dot,
    QuestionMark,
    ExclamationMark,
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match *self {
            Key::A(false) => String::from("a"),
            Key::B(false) => String::from("b"),
            Key::C(false) => String::from("c"),
            Key::D(false) => String::from("d"),
            Key::E(false) => String::from("e"),
            Key::F(false) => String::from("f"),
            Key::G(false) => String::from("g"),
            Key::H(false) => String::from("h"),
            Key::I(false) => String::from("i"),
            Key::J(false) => String::from("j"),
            Key::K(false) => String::from("k"),
            Key::L(false) => String::from("l"),
            Key::M(false) => String::from("m"),
            Key::N(false) => String::from("n"),
            Key::O(false) => String::from("o"),
            Key::P(false) => String::from("p"),
            Key::Q(false) => String::from("q"),
            Key::R(false) => String::from("r"),
            Key::S(false) => String::from("s"),
            Key::T(false) => String::from("t"),
            Key::U(false) => String::from("u"),
            Key::V(false) => String::from("v"),
            Key::W(false) => String::from("w"),
            Key::X(false) => String::from("x"),
            Key::Y(false) => String::from("y"),
            Key::Z(false) => String::from("z"),
            Key::A(true) => String::from("A"),
            Key::B(true) => String::from("B"),
            Key::C(true) => String::from("C"),
            Key::D(true) => String::from("D"),
            Key::E(true) => String::from("E"),
            Key::F(true) => String::from("F"),
            Key::G(true) => String::from("G"),
            Key::H(true) => String::from("H"),
            Key::I(true) => String::from("I"),
            Key::J(true) => String::from("J"),
            Key::K(true) => String::from("K"),
            Key::L(true) => String::from("L"),
            Key::M(true) => String::from("M"),
            Key::N(true) => String::from("N"),
            Key::O(true) => String::from("O"),
            Key::P(true) => String::from("P"),
            Key::Q(true) => String::from("Q"),
            Key::R(true) => String::from("R"),
            Key::S(true) => String::from("S"),
            Key::T(true) => String::from("T"),
            Key::U(true) => String::from("U"),
            Key::V(true) => String::from("V"),
            Key::W(true) => String::from("W"),
            Key::X(true) => String::from("X"),
            Key::Y(true) => String::from("Y"),
            Key::Z(true) => String::from("Z"),
            Key::Zero => String::from("0"),
            Key::One => String::from("1"),
            Key::Two => String::from("2"),
            Key::Three => String::from("3"),
            Key::Four => String::from("4"),
            Key::Five => String::from("5"),
            Key::Six => String::from("6"),
            Key::Seven => String::from("7"),
            Key::Eight => String::from("8"),
            Key::Nine => String::from("9"),
            Key::Space => String::from(" "),
            Key::Dot => String::from("."),
            Key::QuestionMark => String::from("?"),
            Key::ExclamationMark => String::from("!"),
            _ => String::from(""),
        }
    }
}

impl From<char> for Key {
    fn from(sight: char) -> Self {
        match sight {
            'a' => Key::A(false),
            'b' => Key::B(false),
            'c' => Key::C(false),
            'd' => Key::D(false),
            'e' => Key::E(false),
            'f' => Key::F(false),
            'g' => Key::G(false),
            'h' => Key::H(false),
            'i' => Key::I(false),
            'j' => Key::J(false),
            'k' => Key::K(false),
            'l' => Key::L(false),
            'm' => Key::M(false),
            'n' => Key::N(false),
            'o' => Key::O(false),
            'p' => Key::P(false),
            'q' => Key::Q(false),
            'r' => Key::R(false),
            's' => Key::S(false),
            't' => Key::T(false),
            'u' => Key::U(false),
            'v' => Key::V(false),
            'w' => Key::W(false),
            'x' => Key::X(false),
            'y' => Key::Y(false),
            'z' => Key::Z(false),
            'A' => Key::A(true),
            'B' => Key::B(true),
            'C' => Key::C(true),
            'D' => Key::D(true),
            'E' => Key::E(true),
            'F' => Key::F(true),
            'G' => Key::G(true),
            'H' => Key::H(true),
            'I' => Key::I(true),
            'J' => Key::J(true),
            'K' => Key::K(true),
            'L' => Key::L(true),
            'M' => Key::M(true),
            'N' => Key::N(true),
            'O' => Key::O(true),
            'P' => Key::P(true),
            'Q' => Key::Q(true),
            'R' => Key::R(true),
            'S' => Key::S(true),
            'T' => Key::T(true),
            'U' => Key::U(true),
            'V' => Key::V(true),
            'W' => Key::W(true),
            'X' => Key::X(true),
            'Y' => Key::Y(true),
            'Z' => Key::Z(true),
            '0' => Key::Zero,
            '1' => Key::One,
            '2' => Key::Two,
            '3' => Key::Three,
            '4' => Key::Four,
            '5' => Key::Five,
            '6' => Key::Six,
            '7' => Key::Seven,
            '8' => Key::Eight,
            '9' => Key::Nine,
            ' ' => Key::Space,
            '.' => Key::Dot,
            '?' => Key::QuestionMark,
            '!' => Key::ExclamationMark,
            _ => Key::Unknown,
        }
    }
}

pub struct KeyDownEvent {
    pub key: Key,
}

impl Event for KeyDownEvent {}

pub struct KeyUpEvent {
    pub key: Key,
}

impl Event for KeyUpEvent {}

pub type KeyHandler = Rc<Fn(&Key, &mut WidgetContainer) -> bool + 'static>;

#[derive(Default)]
pub struct KeyEventHandler {
    pub on_key_up: Option<KeyHandler>,
    pub on_key_down: Option<KeyHandler>,
}

impl EventHandler for KeyEventHandler {
    fn handle_event(&self, event: &EventBox, widget: &mut WidgetContainer) -> bool {
        if let Ok(event) = event.downcast_ref::<KeyDownEvent>() {
            if let Some(handler) = &self.on_key_down {
                return (handler)(&event.key, widget);
            }
        }

        if let Ok(event) = event.downcast_ref::<KeyUpEvent>() {
            if let Some(handler) = &self.on_key_up {
                return (handler)(&event.key, widget);
            }
        }

        false
    }
}
