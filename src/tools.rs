//! Set of tools useful for manipulation of numeric and 
//! alphabetic values

/// Mathematical tools
pub mod math {
    use rand::Rng;

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
    pub fn check_proba(proba: f32) -> Result<bool, String> {
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
    pub fn exp_decay(input_value: f32, factor: f32, k: f32) -> f32 {
        let final_dam: f32 = input_value * (-k * factor).exp();
        return  final_dam;
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
    pub fn normalize(value: f32) -> Result<f32, String> {
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
}