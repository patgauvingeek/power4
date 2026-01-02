use std::io;
use power_4::App;

fn main() -> io::Result<()>
{
  ratatui::run(|terminal| App::default().run(terminal))
}
