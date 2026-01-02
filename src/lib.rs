use std::io;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::{Color, Style, Stylize},
  symbols::border,
  text::{Line, Span},
  widgets::{Block, Paragraph, Widget},
  DefaultTerminal, Frame,
};

pub mod game;

#[derive(Debug, Default)]
pub struct App
{
  iteration: usize,
  game: game::Game,
  selected_column: usize,
  exit: bool
}

impl App
{
  /// runs the application's main loop until the user quits
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>
  {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    while !self.exit
    {
      terminal.draw(|frame| self.draw(frame))?;
      let timeout = tick_rate
        .saturating_sub(last_tick.elapsed());
      if event::poll(timeout)?
      {
        self.handle_events()?;
      }
      if last_tick.elapsed() >= tick_rate
      {
        self.on_tick();
        last_tick = Instant::now();
      }
    }
    Ok(())
  }

  fn exit(&mut self)
  {
    self.exit = true;
  }

  fn draw(&self, frame: &mut Frame)
  {
    frame.render_widget(self, frame.area());
  }

  fn on_tick(&mut self)
  {
    self.iteration += 1;
    self.game.animate();
  }

  fn handle_events(&mut self) -> io::Result<()>
  {
    match event::read()? {
      // it's important to check that the event is a key press event as
      // crossterm also emits key release and repeat events on Windows.
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
          self.handle_key_event(key_event)
      }
      _ => {}
    };
    Ok(())
  }

  fn handle_left(&mut self)
  {
    if self.selected_column > 0
    {
      self.selected_column -= 1;
    }
  }

  fn handle_right(&mut self)
  {
    if self.selected_column < self.game.board()[0].len() - 1
    {
      self.selected_column += 1;
    }
  }

  fn handle_space(&mut self)
  {
    if self.game.winner() == None && !self.game.is_full()
    {
      self.game.drop(self.selected_column);
    }
    else
    {
      self.game.reset();
    }
  }

  pub fn handle_key_event(&mut self, key_event: KeyEvent)
  {
    match key_event.code
    {
      KeyCode::Char('q') => self.exit(),
      KeyCode::Left => { self.handle_left() },
      KeyCode::Right => { self.handle_right() },
      KeyCode::Char(' ') => { self.handle_space() },
      _ => {}
    }
  }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
      let title = Line::from(" Power-4 ".bold());
      let instructions = Line::from(vec![
          " Select ".into(),
          "<Left>".blue().bold(),
          "<Right>".blue().bold(),
          " Drop ".into(),
          "<Space>".blue().bold(),
          " Quit ".into(),
          "<Q> ".blue().bold(),
      ]);
      let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);
      
      let mut power_4: Vec<_> = self.game.board()
        .iter()
        .map(|row: &[game::Cell; 7]|
        {
          let spans: Vec<Span> = row
            .iter()
            .map(|cell| match cell
            {
              game::Cell::Empty => Span::raw(" . "), // Visual for empty cell
              game::Cell::Occupied(player) =>
              {
                // Customize colors based on player
                let color = if player == &game::Player::One { Color::Red } else { Color::Yellow };
                Span::styled(" ● ", Style::default().fg(color))
              }
            })
            .collect();
            Line::from(spans) // Combine row spans into a single Line
          })
          .collect();
      
      if self.game.winner() != None
      {
        power_4.insert(0, Line::from(""));
        power_4.insert(0, Line::from("GAME OVER"));
        let winner_text = format!("Player {} Wins !!!", if self.game.winner() == Some(game::Player::One) { "One" } else { "Two" });
        power_4.push(Line::from(""));
        power_4.push(Line::from(winner_text));
      }
      else if self.game.is_full()
      {
        power_4.insert(0, Line::from(""));
        power_4.insert(0, Line::from("GAME OVER"));
        power_4.push(Line::from(""));
        power_4.push(Line::from("DRAW !!!"));
      }
      else 
      {
        let column_indicator = format!("{}{}{}", "   ".repeat(self.selected_column), " ↓ ", "   ".repeat(6 - self.selected_column));
        power_4.insert(0,  Line::from(column_indicator));
        let player_indicator = format!("{} {}", "Player", if self.game.current_player() == game::Player::One { "One" } else { "Two" });
        power_4.insert(0,  Line::from(player_indicator));
      }

      Paragraph::new(power_4)
        .centered()
        .block(block)
        .render(area, buf);
    }
}
