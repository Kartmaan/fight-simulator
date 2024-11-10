mod tools;
mod player;
mod mobs;

use player::{Player, PlayerClass};
use mobs::{Mob, MoveCategory, BESTIARY, get_mob};
use tools::spatial::Pos;
use tools::math::{normalize, check_proba, exp_decay};
use tools::game_mechanics::attack;

fn main() {
    let mut dragon = get_mob("dragon").unwrap();
    dragon.move_to(51, 78);
    dragon.info();
    println!("{:?}", dragon.get_pos());

    let mut gobelin = get_mob("gobelin").unwrap();
    gobelin.info();

    let mut shark = get_mob("shark").unwrap();
    shark.info();

    let mut player = Player::new(
        "Lost".to_string(), 
        PlayerClass::Archer, 
        Pos::new(50, 50));
    
    player.info();

    attack(&shark, &mut dragon);
}