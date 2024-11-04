use rand::Rng;

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

/// 2D coordinates
#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

/// The character controlled by the player
struct Player {
    name: String,
    pos: Pos,
}

/// The enemy to be defeated
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

/// Normalizes a value to be between 0 and 1.
/// 
/// # Details
/// The function will attempt by several means to normalize the 
/// value according to its order of magnitude.
/// - If the value is within the range [0,1] it's returned as is. 
/// - If the value is within the range ]1,100] then the it's 
/// divided by 100.
/// - All values ​​greater than 100 become 1.0.
/// - Otherwise the function returns an error (we assume that 
/// the value is negative).
/// 
/// # Arguments
/// * `value` - The f32 value to be normalized.
/// 
/// # Return value
/// * `Ok(f32)` - The normalized value if valid.
/// * `Err(String)` - Error message if the value is invalid.
/// 
/// # Exemples
/// ```
/// assert_eq!(normalize(0.5).unwrap(), 0.5);
/// assert_eq!(normalize(50.0).unwrap(), 0.5);
/// assert_eq!(normalize(150.0).unwrap(), 1.0);
/// assert!(normalize(-1.0).is_err());
/// ```
fn normalize(value: f32) -> Result<f32, String> {
    if value >= 0.0 && value <= 1.0 {
        Ok(value)
    } else if value > 1.0 && value <= 100.0 {
        Ok(value / 100.0)
    } else if value > 100.0 {
        Ok(1.0)
    } else {
        Err(String::from("Speed value must be between 0 and 1"))
    }
}

/// Calculates an exponential reduction of an initial value based 
/// on a given factor.
/// 
/// # Arguments
/// 
/// * `init_value` - The initial value to reduce (f32).
/// * `factor` - The decline factor that influences the intensity 
/// of the reduction (f32).
/// * `k` - Parameter controlling the decay rate.
/// 
/// # Returns
/// 
/// * The reduced value after applying the exponential 
/// reduction (f32).
/// 
/// # Example
/// Let's imagine a damage reduction function: `input_value` 
/// would be the initial damage, while `factor` would be our 
/// armor points, the higher these are, the greater the reduction 
/// of `input_value` will be. If the damage received is 50 and 
/// our armor is 100 with a parameter k set to 0.0217 then the 
/// function will be called as follows: 
/// ```
/// let final_dam = exp_decay(50.0, 100.0, 0.0217);
/// println!("{}", final_dam) // 5.708
/// ```
/// The 50 initial damage is reduced to around 5.7, the higher 
/// the armor value the more effective this defense will be 
/// and vice versa.
/// 
/// # Note
/// *This function uses the exponential function `exp()` from the 
/// Rust standard library whose precision is not deterministic*.
fn exp_decay(input_value: f32, factor: f32, k: f32) -> f32 {
    let final_dam: f32 = input_value * (-k * factor).exp();
    return  final_dam;
}

/// Tests a probability based on a percentage value : if the 
/// probability is realized then the function returns `true`, 
/// otherwise `false`.
/// 
/// # Arguments
/// 
/// * `proba` : The probability value in percent (f32)
/// 
/// # Returns
/// 
/// * `Ok(true)` : The probability has been realized
/// * `Ok(false)` : The probability was not realized
/// * `Err(String)` : An error has been encountered
/// 
/// # Example
/// 
/// The function acts like a dice roll. For example, if we want 
/// an event to occur only once out of three:
/// ```
/// if check_proba(33.33).unwrap() {
///     println!("OK");
/// } else {
///     println!("NOPE");
/// }
/// ```
fn check_proba(proba: f32) -> Result<bool, String> {
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

    // Generation of a float between 0 and 1
    let rng_num: f32 = rand::thread_rng().gen();

    // Probability check
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
    /// Create a new Mob
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

    /// Moves a Mob to a x, y coordinates
    fn move_to(&mut self, x:i32, y:i32) {
        self.pos.move_to(x, y);
    }

    /// Print some infos about the Mob
    fn info(&self) {
        println!("\nCategory : {:?}", self.category);
        println!("Speed : {}", self.speed);
        println!("Pos x,y : ({},{})", self.pos.x, self.pos.y);
        println!("HP : {}", self.hp);
        println!("Alive : {}", self.alive);
    }


    /// The Mob recieved damage
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

    let test_dam = exp_decay(50.0, 100.0, 0.0217);
    println!("Exp dacay : {}", test_dam);

    if check_proba(33.33).unwrap() {
        println!("OK")
    }
}