mod utils;
mod player;
mod mobs;

use player::{Player, PlayerClass};
use mobs::{Mob, get_mob};
use utils::spatial::Pos;
use utils::math::round;
use utils::game_mechanics::{attack, defense, battle};
use utils::traits::{Located, Mortal};

fn main() {
    let mut gobelin: Mob = get_mob("gobelin").unwrap();
    let mut dragon: Mob = get_mob("dragon").unwrap();
    let mut shark: Mob = get_mob("shark").unwrap();
    shark.set_pos(Pos::new(170, 45));

    let mut player = Player::new(
        "Lost".to_string(), 
        PlayerClass::Warrior, 
        Pos::new(50, 50));
    
    //let dist = player.get_distance(&shark);
    //println!("{}", dist);

    battle(&mut player, &mut gobelin);

    player.info();
    gobelin.info();
    /* for _ in 1..40 {
        print!("{}|", round(attack(&shark), 2));
    }

    for hit in 1..50 {
        defense(&mut player, 50);
        if player.get_hp() <= 0 {
            println!("Hits: {}", hit);
            break;
        }
    } */
}