use rand::Rng;

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
    armor: i32, // 
    precision: f32, // Chance of hitting the target
    damage: u32, // Base damage
    crit_proba: f32, // Critical hit probability
    crit_mult_dam: f32, // Critical multiplicative damage
    dodge_proba: f32,
    in_alert: bool,
    is_attacking: bool,
    alive: bool,
}

/// Normalize a value x between 0 and 1
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// Normalizes a value
/// ## Details
/// The function will attempt by several means to normalize the 
/// value according to its order of magnitude
/// - If the value is within the range [0,1] it's returned as is. 
/// - If the value is within the range ]1,100] then the it's 
/// divided by 100 
/// - If the value is greater than 100 then it's passed through 
/// a sigmoid function. 
/// - Otherwise the function returns an error (we assume that 
/// the value is negative)
fn normalize(value: f32) -> Result<f32, String> {
    if value >= 0.0 && value <= 1.0 {
        Ok(value)
    } else if value > 1.0 && value <= 100.0 {
        Ok(value / 100.0)
    } else if value > 100.0 {
        Ok(sigmoid(value))
    } else {
        Err(String::from("Speed value must be between 0 and 1"))
    }
}

/// Calculates the final damage received based on the armor value.
/// 
/// ## Details
/// A decreasing exponential function is used to give more value 
/// to the first armor points, making the damage reduction much 
/// greater at the beginning and more negligible at last armor
/// values.
/// 
/// ### Formula
/// final_dam = init_dam * exp(-k * armor)
/// 
/// k is the parameter controlling the decay rate
fn armor(damage: i32, armor_val: i32) -> i32 {
    let k: f32 = 0.0314; // Seems ok...
    let final_dam: f32 = damage as f32 * (-k * armor_val as f32).exp();
    return  final_dam as i32;
}

/// The function tests a probability based on a value 
/// between 0 and 1
fn proba_test(proba: f32) -> Result<bool, String> {
    let coef: f32;
    
    // A probability of 0 could create infinite loops. 
    // Values ​​less than or equal to 0 are prohibited.
    if proba <= 0.0 {
        return Err(String::from("Value can't be less than zero"));
    }
    
    // If a percentage is inserted, the value is divided by 100 
    // so that it fits in the range ]0,1]
    if proba > 1.0 && proba <= 100.0 {
        coef = proba / 100.0;
    } else {
        return Err(String::from("Value can't be greater than 100"));
    }

    let rng_num: f32 = rand::thread_rng().gen();

    if rng_num < coef {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

impl Pos {
    /// Create a new Pos struct
    fn new(x: i32, y:i32) -> Pos {
        Pos {x:x, y:y}
    }

    /// Change the coordinates of a Pos struct
    fn move_to(&mut self, x:i32, y:i32) {
        self.x = x;
        self.y = y;
    }

    /// Euclidian distance between two Pos coordinates
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
            speed: normalize(speed).unwrap(),
            hp: 100,
            armor: 40,
            precision: 0.9,
            damage: 33,
            crit_proba: 0.05,
            crit_mult_dam: 1.5,
            dodge_proba: 0.03,
            in_alert: false,
            is_attacking: false,
            alive: true,
        }
    }

    fn move_to(&mut self, x:i32, y:i32) {
        self.pos.move_to(x, y);
    }

    fn info(&self) {
        println!("\nCategory : {:?}", self.category);
        println!("Speed : {}", self.speed);
        println!("Pos x,y : ({},{})", self.pos.x, self.pos.y);
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

    gobelin.move_to(80, 160);
    gobelin.hit(50);
    gobelin.info();

    let test_dam = armor(50, 0);
    println!("{}", test_dam);

    println!("Sigmoid 1278 : {}", sigmoid(1234.0));
}