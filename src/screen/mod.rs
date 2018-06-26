use std::fmt;
use std::process::Command;

pub struct Screen {
  screen: [[bool; 64]; 32],
}

fn wrap_x(x: usize) -> usize {
  x % 64
}

fn wrap_y(y: usize) -> usize {
  y % 32
}

impl Screen {
  pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
    let mut collision = false;
    let mut i = 0;
    for byte in sprite {
      for j in 0..8 {
        let screen_value = self.screen[wrap_y(i + y)][wrap_x(j + x)];
        let write_value = (byte << j) & 0b1000_0000 != 0;
        let new_screen_value = write_value ^ screen_value;
        collision = screen_value & !new_screen_value;
        self.screen[wrap_y(i + y)][wrap_x(j + x)] = new_screen_value;
      }
      i += 1;
    }
    collision
  }

  pub fn clear_screen(& self) {
    Command::new("clear").output().unwrap_or_else(|e| {
      panic!("Failed to clear screen: {}", e)
    });
  }

  pub fn print_screen(& self) {
    print!("{:#?}", self);
  }
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
        let cell_char = if cell { "X" } else { " " };
        write!(f, "{}", cell_char)?;
      }
      write!(f, "{}", separator)?;
    }

    write!(f, "{}", separator)
  }
}
