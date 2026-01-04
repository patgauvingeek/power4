use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Player
{
  #[default] One,
  Two
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Cell
{
  #[default] Empty,
  Occupied(Player)
}

#[derive(Debug)]
pub struct Game
{
  validation_board: [[Cell; 7]; 6], // A 6x7 grid (rows x columns)
  animated_board: [[Cell; 7]; 6], // A 6x7 grid (rows x columns)
  current_player: Player,
  winner: Option<Player>,
  player_one_available_token_count : usize,
  player_two_available_token_count : usize,
  is_reseting : bool
}

impl Default for Game
{
  fn default() -> Self {
    Self {
      validation_board: Default::default(),
      animated_board: Default::default(),
      current_player: Default::default(),
      winner: Default::default(),
      player_one_available_token_count: 21,
      player_two_available_token_count: 21,
      is_reseting: Default::default()
    }
  }
}

impl Game
{
  pub fn board(&self) -> &[[Cell; 7]]
  {
    &self.animated_board
  }

  pub fn current_player(&self) -> Player
  {
    self.current_player
  }

  pub fn winner(&self) -> Option<Player>
  {
    self.winner
  }

  pub fn player_one_available_token_count(&self) -> usize
  {
    self.player_one_available_token_count
  }

  pub fn player_two_available_token_count(&self) -> usize
  {
    self.player_two_available_token_count
  }

  pub fn animate(&mut self)
  {
    if self.is_reseting
    {
      self.player_one_available_token_count += self.animated_board[self.animated_board.len()-1].iter().filter(|&c| *c == Cell::Occupied(Player::One)).count();
      self.player_two_available_token_count += self.animated_board[self.animated_board.len()-1].iter().filter(|&c| *c == Cell::Occupied(Player::Two)).count();
      for column_index in 0..self.animated_board[0].len()
      {
        for row_index in (1..self.animated_board.len()).rev()
        {
          self.animated_board[row_index][column_index] = self.animated_board[row_index-1][column_index];
          self.animated_board[row_index-1][column_index] = Cell::Empty;
        }
      }
      // row 0
      for column_index in 0..self.animated_board[0].len()
      {
        self.animated_board[0][column_index] = Cell::Empty;
      }
      // reset animation is complete
      if self.animated_board.iter().all(|&r| r.iter().all(|&c| c == Cell::Empty))
      {
        self.is_reseting = false;
        self.current_player = Player::One;
        self.winner = None;
        for column_index in 0..self.validation_board[0].len()
        {
          for row_index in 0..self.validation_board.len()
          {
            self.validation_board[row_index][column_index] = Cell::Empty;
          }
        }
      }
      return;
    }
    for column_index in 0..self.animated_board[0].len()
    {
      for row_index in (1..self.animated_board.len()).rev()
      {
        if self.animated_board[row_index][column_index] == Cell::Empty
        {
          self.animated_board[row_index][column_index] = self.animated_board[row_index-1][column_index];
          self.animated_board[row_index-1][column_index] = Cell::Empty;
        }
      }
    }
  }

  fn is_four_in_a_row(&self, row_index: usize, column_index: usize, row_delta: i32, column_delta: i32, player: Player) -> bool {
    for i in 1..4
    {
      let v_row_index = row_index as i32 + row_delta * i;
      let v_column_index = column_index as i32 + column_delta * i;

      let row_count = self.validation_board.len() as i32;
      let column_count = self.validation_board[0].len() as i32;
      // Boundary check
      if v_row_index < 0 || v_row_index >= row_count || v_column_index < 0 || v_column_index >= column_count
      {
        return false;
      }

      // Cell match check
      if self.validation_board[v_row_index as usize][v_column_index as usize] == Cell::Empty ||
         self.validation_board[v_row_index as usize][v_column_index as usize] != Cell::Occupied(player)
      {
        return false;
      }
    }
    true
  }

  fn compute_winner(&self) -> Option<Player>
  {
    for column_index in 0..self.validation_board[0].len()
    {
      for row_index in 1..self.validation_board.len()
      {
        // Skip empty cells
        let v_player = match self.validation_board[row_index][column_index] {
          Cell::Occupied(p) => p,
          Cell::Empty => continue,
        };
        // Directions to check
        let directions = [
          (0, 1),  // Horizontal (right)
          (1, 0),  // Vertical (down)
          (1, 1),  // Diagonal (down-right)
          (1, -1), // Diagonal (down-left)
        ];

        for (row_delta, column_delta) in directions
        {
          if self.is_four_in_a_row(row_index, column_index, row_delta, column_delta, v_player)
          {
            return Some(v_player);
          }
        }
      }
    }
    None
  }
  
  pub fn is_full(&self) -> bool
  {
    for column_index in 0..self.validation_board[0].len()
    {
      if self.validation_board[0][column_index] == Cell::Empty
      {
        return false;
      }
    }
    return true;
  }

  pub fn drop(&mut self, column_index: usize)
  {
    if self.winner != None
    {
      return;
    }
    if self.animated_board[0][column_index] != Cell::Empty
    {
      return;
    }
    // start the animation
    self.animated_board[0][column_index] = Cell::Occupied(self.current_player);
    match self.current_player
    {
      Player::One => self.player_one_available_token_count -= 1,
      Player::Two => self.player_two_available_token_count -= 1,
    }
    // update the validation_board
    let mut row_index = self.validation_board.len() - 1;
    loop
    {
      if self.validation_board[row_index][column_index] == Cell::Empty
      {
        self.validation_board[row_index][column_index] = Cell::Occupied(self.current_player);
        self.current_player = if self.current_player == Player::One { Player::Two } else { Player::One };
        break;
      }
      if row_index == 0
      {
        break;
      }
      row_index -= 1;
    }
    self.winner = self.compute_winner();
  }

  pub fn reset(&mut self)
  {
    self.is_reseting = true;
  }
}

#[cfg(debug_assertions)]
impl fmt::Display for Game
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  {
    let mut board_string = String::new();

    // Iterate through rows (0 to 5)
    for row in &self.validation_board {
      board_string.push('|'); // Side border
      for cell in row {
        let symbol = match cell {
          Cell::Empty => ".",
          Cell::Occupied(Player::One) => "X", // Player 1 is X
          Cell::Occupied(Player::Two) => "O", // Player 2 is O
        };
        board_string.push_str(&format!(" {} ", symbol));
      }
      board_string.push_str("|\n"); // End of row
    }

    // Add a bottom border and column numbers
    board_string.push_str("+---------------------+ \n");
    board_string.push_str("  0  1  2  3  4  5  6  ");

    write!(f, "{}", board_string)
  }
}
