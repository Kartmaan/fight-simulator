mod tools;
mod player;
mod mobs;
mod spatial;

use player::{Player, PlayerClass};
use mobs::{Mob, MoveCategory, BESTIARY, get_mob};
use spatial::Pos;
use tools::math::{normalize, check_proba, exp_decay};

fn main() {
    let mut dragon = BESTIARY.get("dragon").cloned().unwrap();
    dragon.move_to(51, 78);
    dragon.info();
    println!("{:?}", dragon.get_pos());

    let mut gobelin = get_mob("gobelin").unwrap();
    gobelin.info();
}