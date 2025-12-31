#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Player {
  #[default] One,
  Two
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
  #[default] Empty,
  Occupied(Player)
}

#[derive(Debug, Default)]
pub struct Game {
  board: [[Cell; 7]; 6], // A 6x7 grid (rows x columns)
  current_player: Player,
  game_over: bool,
}

impl Game {
  /// Creates a new, empty Power Four game board.
  pub fn new() -> Self {
    Game {
      board: [[Cell::Empty; 7]; 6],
      current_player: Player::One,
      game_over: false,
    }
  }

  pub fn board(&self) -> &[[Cell; 7]]
  {
    &self.board
  }

  pub fn current_player(&self) -> Player
  {
    self.current_player
  }

  pub fn drop(&mut self, column_index: usize) -> bool
  {
    let mut row_index = self.board.len() - 1;
    loop
    {
      if self.board[row_index][column_index] == Cell::Empty
      {
        self.board[row_index][column_index] = Cell::Occupied(self.current_player);
        self.current_player = if self.current_player == Player::One { Player::Two } else { Player::One };
        return true;
      }
      if row_index == 0
      {
        return false;
      }
      row_index -= 1;
    }
  }

  // You would add methods here to:
  // * `drop_piece(&mut self, col: usize) -> Result<(), &str>`: Handle dropping a piece into a column.
  // * `check_for_win(&self) -> Option<Player>`: Check if the last move resulted in a win.
  // * `is_board_full(&self) -> bool`: Check for a draw.
  // * `display_board(&self)`: Print the current state of the board to the console.
}
