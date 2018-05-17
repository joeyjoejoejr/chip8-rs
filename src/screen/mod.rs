use std::fmt;

pub struct Screen {
  screen: [[bool; 64]; 32],
}

impl Default for Screen {
  fn  default() -> Screen {
    Screen { screen: [[false; 64]; 32] }
  }
}

impl fmt::Debug for Screen {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let separator = if f.alternate() {"\n"} else { "|" };
    write!(f, "{}", separator)?;

    for row in self.screen.iter() {
      for &cell in row.iter() {
        let cell_char = if cell { 'X' } else { ' ' };
        write!(f, "{}", cell_char)?;
      }
      write!(f, "{}", separator)?;
    }

    write!(f, "{}", separator)
  }
}
