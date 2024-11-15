mod utils;
mod player;
mod mobs;

use player::{Player, PlayerClass};
use mobs::{Mob, get_mob};
use utils::spatial::Pos;
use utils::game_mechanics::battle;

fn main() {
    let mut gobelin: Mob = get_mob("gobelin").unwrap();

    let mut player = Player::new(
        "Lost".to_string(), 
        PlayerClass::Warrior, 
        Pos::new(50, 50));
    
    let mut player_2 = Player::new(
        "Duriel".to_string(), 
        PlayerClass::Archer, 
        Pos::new(150, 70));

    battle(&mut player, &mut player_2);

    player.info();
    player_2.info();
}