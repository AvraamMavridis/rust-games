use core::fmt;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Players {
    #[default]
    X,
    O
}

impl fmt::Display for Players {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Players::X => write!(f, "X"),
            Players::O => write!(f, "O"),
        }
    }
}

impl Into<char> for Players {
  fn into(self) -> char {
    match self {
      Players::O => 'O',
      Players::X => 'X'
    }
  }
}
