use super::World;
use crate::error::Result;
use crate::event::Event;
use crate::player::Player;

impl World {
  pub fn start_round(&mut self) -> Result<()> {
    let ids = self
      .player_manager
      .players()
      .filter(|player| !player.is_inactive())
      .map(Player::id);

    self.round.start(ids)?;

    let round = Box::new(self.round.clone());
    self.emit(Event::RoundUpdated { round });
    Ok(())
  }
}
