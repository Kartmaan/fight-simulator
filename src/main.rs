const E: f32 = 2.71828;

/// Different categories of movement
#[derive(Debug)]
enum MoveCategory {
    Terrestrial,
    Aerian,
    Aquatic,
}

/// Class chosen by the player
enum PlayerClass {
    Brawler,
    Swordsman,
    Archer,
    Mage,
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

struct Player {
    name: String,
    pos: Pos,
}

struct Mob {
    category: MoveCategory,
    pos: Pos,
    speed: f32,
    hp: i32,
    damage: u32, // Base damage
    crit_proba: f32, // Critical hit probability
    crit_mult_dam: f32, // Critical multiplicative damage
    in_alert: bool,
    is_attacking: bool,
    alive: bool,
}

/// Normalize a value x between 0 and 1
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + E.powf(-x))
}

/// # speed_check
/// Checks the speed value which must be between 0 and 1 
/// (normalization)
/// ## Explication
/// If the value is within the range it's returned as is, 
/// if it's greater than this range then the value is passed 
/// through a sigmoid function, otherwise (presumably if the 
/// value is negative) the function returns an error
fn speed_check(speed: f32) -> Result<f32, String> {
    if speed >= 0.0 && speed <= 1.0 {
        Ok(speed)
    } else if speed > 1.0 {
        Ok(sigmoid(speed))
    } else {
        Err(String::from("Speed value must be between 0 and 1"))
    }
}

impl Pos {
    fn new(x: i32, y:i32) -> Pos {
        Pos {x:x, y:y}
    }

    /// Euclidian distance between two coordinates
    fn dist(&self, other:Pos) -> f32 {
        let res = (other.x - self.x).pow(2) + (other.y - self.y).pow(2);
        (res as f32).sqrt()
    }
}

impl Mob {
    fn new(cat: MoveCategory, speed: f32, pos: Pos) -> Mob {
        Mob {
            category: cat,
            pos: pos,
            speed: speed_check(speed).unwrap(),
            hp: 100,
            damage: 33,
            crit_proba: 0.05,
            crit_mult_dam: 1.5,
            in_alert: false,
            is_attacking: false,
            alive: true,
        }
    }

    fn info(&self) {
        println!("Category : {:?}", self.category);
        println!("Speed : {}", self.speed);
        println!("Pos x/y : {}/{}", self.pos.x, self.pos.y);
        println!("HP : {}", self.hp);
        println!("Alive : {}", self.alive);
    }

    fn hit(&mut self, damage: i32) {
        self.hp = self.hp - damage;

        // Damage reducing health below 0
        if self.hp < 0 {
            self.hp = 0;
        }

        if self.hp > 0 {
            self.alive = true;
        } else {
            self.alive = false;
        }
    }

    /// Euclidian distance between two Mobs
    fn dist(&self, other_mob:&Mob) -> f32 {
        let res: i32 = (other_mob.pos.x - self.pos.x).pow(2) + (other_mob.pos.y - self.pos.y).pow(2);
        (res as f32).sqrt()
    }
}

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
    )};

    dragon.info();
    gobelin.info();

    let distance: f32 = dragon.dist(&gobelin);
    println!("Distance between dragon and gobelin : {}", distance);
}