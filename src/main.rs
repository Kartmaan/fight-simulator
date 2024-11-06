mod tools;
mod player;
mod mobs;
mod spatial;

use tools::math::{normalize, check_proba, exp_decay};
use player::{Player, PlayerClass};
use mobs::{Mob, MoveCategory};
use spatial::Pos;

fn main() {
    let mut dragon: Mob = {
    Mob::new( 
    MoveCategory::Aerian, 
    0.7,
    Pos::new(12, 50),
    )};

    let mut gobelin: Mob = {
        Mob::new( 
        MoveCategory::Terrestrial, 
        0.5,
        Pos::new(72, 150),
        )
    };


    dragon.info();
    gobelin.info();

    gobelin.move_to(80, 160);
    gobelin.hit(50);
    gobelin.info();

    let test_dam = exp_decay(50.0, 100.0, 0.0217);
    println!("Exp dacay : {}", test_dam);

    if check_proba(33.33).unwrap() {
        println!("OK")
    }
}