mod utils;
mod player;
mod mobs;

use player::{Player, PlayerClass};
use mobs::{Mob, get_mob};
use utils::spatial::Pos;
use utils::math::round;
use utils::game_mechanics::{attack, defense};
use utils::traits::Mortal;

fn main() {
    // let mut dragon: Mob = get_mob("dragon").unwrap();

    // let mut gobelin: Mob = get_mob("gobelin").unwrap();

    let mut shark: Mob = get_mob("shark").unwrap();

    let mut player = Player::new(
        "Lost".to_string(), 
        PlayerClass::Warrior, 
        Pos::new(50, 50));

    //let dam = attack(&shark);
    //println!("{}", dam);

    for _ in 1..40 {
        print!("{}|", round(attack(&shark), 2));
    }

    for hit in 1..50 {
        defense(&mut player, 50);
        if player.get_hp() <= 0 {
            println!("Hits: {}", hit);
            break;
        }
    }
}