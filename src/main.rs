mod utils;
mod player;
mod mobs;

use player::{Player, PlayerClass};
use mobs::{Mob, MoveCategory, BESTIARY, get_mob};
use utils::spatial::Pos;
use utils::math::{normalize, check_proba, exp_decay};
use utils::game_mechanics::{attack, defense};

fn main() {
    let mut dragon = get_mob("dragon").unwrap();
    dragon.move_to(51, 78);
    //dragon.info();
    //println!("{:?}", dragon.get_pos());

    let mut gobelin = get_mob("gobelin").unwrap();
    //gobelin.info();

    let mut shark = get_mob("shark").unwrap();
    //shark.info();

    let mut player = Player::new(
        "Lost".to_string(), 
        PlayerClass::Warrior, 
        Pos::new(50, 50));
    
    //player.info();

    //let dam = attack(&shark);
    //println!("{}", dam);

    /* for _ in 1..40 {
        print!("{}|", attack(&shark));
    } */

    for _ in 1..50 {
        defense(&mut player, 25);
    }
}