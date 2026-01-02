#[cfg(test)]
mod tests
{
  use power_4::game::*;

  #[test]
  fn player_one_wins()
  {
    let mut game: Game = Default::default();
    game.drop(0);
    game.animate();
    game.drop(0);

    game.drop(1);
    game.animate();
    game.drop(1);

    game.drop(2);
    game.animate();
    game.drop(2);

    game.drop(3);
    game.animate();
    game.drop(3);

    game.drop(4);
    game.animate();

    // help to debug if one of the asserts fails...
    println!("{}", game);

    assert_eq!(game.winner(), Some(Player::One));
    assert_eq!(false, game.is_full());
  }

  #[test]
  fn player_two_wins()
  {
    let mut game: Game = Default::default();
    game.drop(0);
    game.animate();
    game.drop(1);
    game.animate();

    game.drop(0);
    game.animate();
    game.drop(1);
    game.animate();

    game.drop(0);
    game.animate();
    game.drop(1);
    game.animate();

    game.drop(2);
    game.animate();
    game.drop(1);
    game.animate();

    // help to debug if one of the asserts fails...
    println!("{}", game);

    assert_eq!(game.winner(), Some(Player::Two));
    assert_eq!(false, game.is_full());
  }

  fn fill_draw(game: &mut Game, index: usize)
  {
    let half = game.board().len() / 2;
    for _ in 0..half
    {
      game.drop(index);
      game.drop(index+1);
      game.animate();
    }
    for _ in half..game.board().len()
    {
      game.drop(index+1);
      game.drop(index);
      game.animate();
    }
  }
  
  #[test]
  fn draw()
  {
    let mut game: Game = Default::default();
    fill_draw(&mut game, 0);
    fill_draw(&mut game, 2);
    fill_draw(&mut game, 5);
    for _ in 0..game.board().len()
    {
      game.drop(4);
      game.animate();
    }

    // help to debug if one of the asserts fails...
    println!("{}", game);

    assert_eq!(game.winner(), None);
    assert_eq!(true, game.is_full());
  }

}
