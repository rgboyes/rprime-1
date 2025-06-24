/// Factorizes a number into its prime components using 
/// wheel factorization. See: <https://en.wikipedia.org/wiki/Wheel_factorization>
///
/// # Arguments
/// * `number` - A positive integer to factorize
///
/// # Returns
/// * `Ok(Vec<u128>)` - A vector of prime factors
/// * `Err(String)` - If the input is zero
///
/// # Examples
/// ```
/// let factors = rprime::wheel_factorize(60).unwrap();
/// assert_eq!(factors, vec![2, 2, 3, 5]);
/// ```
pub fn wheel_factorize(mut number: u128) -> Result<Vec<u128>, String> {
    if number == 0 {
        return Err(String::from("Input must be greater than zero."));
    }

    let mut factors = Vec::with_capacity(24);
    let small_primes = [2, 3, 5, 7, 11];

    for prime in small_primes {
        while number % prime == 0 {
            number /= prime;
            factors.push(prime);
        }

        if number == 1 {
            return Ok(factors);
        }
    }
    let increments = [
        4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2,
        6, 4, 6, 8, 4, 2, 4, 2, 4, 14, 4, 6, 2, 10, 2, 6,
        6, 4, 2, 4, 6, 2, 10, 2, 4, 2, 12, 10, 2, 4, 2, 4,
        6, 2, 6, 4, 6, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4,
        2, 4, 6, 8, 6, 10, 2, 4, 6, 2, 6, 6, 4, 2, 4, 6,
        2, 6, 4, 2, 6, 10, 2, 10, 2, 4, 2, 4, 6, 8, 4, 2,
        4, 12, 2, 6, 4, 2, 6, 4, 6, 12, 2, 4, 2, 4, 8, 6,
        4, 6, 2, 4, 6, 2, 6, 10, 2, 4, 6, 2, 6, 4, 2, 4,
        2, 10, 2, 10, 2, 4, 6, 6, 2, 6, 6, 4, 6, 6, 2, 6,
        4, 2, 6, 4, 6, 8, 4, 2, 6, 4, 8, 6, 4, 6, 2, 4,
        6, 8, 6, 4, 2, 10, 2, 6, 4, 2, 4, 2, 10, 2, 10, 2,
        4, 2, 4, 8, 6, 4, 2, 4, 6, 6, 2, 6, 4, 8, 4, 6,
        8, 4, 2, 4, 2, 4, 8, 6, 4, 6, 6, 6, 2, 6, 6, 4,
        2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10, 2, 6, 4, 6,
        2, 6, 4, 2, 4, 6, 6, 8, 4, 2, 6, 10, 8, 4, 2, 4,
        2, 4, 8, 10, 6, 2, 4, 8, 6, 6, 4, 2, 4, 6, 2, 6,
        4, 6, 2, 10, 2, 10, 2, 4, 2, 4, 6, 2, 6, 4, 2, 4,
        6, 6, 2, 6, 6, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6,
        4, 8, 4, 6, 2, 6, 6, 4, 2, 4, 6, 8, 4, 2, 4, 2,
        10, 2, 10, 2, 4, 2, 4, 6, 2, 10, 2, 4, 6, 8, 6, 4,
        2, 6, 4, 6, 8, 4, 6, 2, 4, 8, 6, 4, 6, 2, 4, 6,
        2, 6, 6, 4, 6, 6, 2, 6, 6, 4, 2, 10, 2, 10, 2, 4,
        2, 4, 6, 2, 6, 4, 2, 10, 6, 2, 6, 4, 2, 6, 4, 6,
        8, 4, 2, 4, 2, 12, 6, 4, 6, 2, 4, 6, 2, 12, 4, 2,
        4, 8, 6, 4, 2, 4, 2, 10, 2, 10, 6, 2, 4, 6, 2, 6,
        4, 2, 4, 6, 6, 2, 6, 4, 2, 10, 6, 8, 6, 4, 2, 4,
        8, 6, 4, 6, 2, 4, 6, 2, 6, 6, 6, 4, 6, 2, 6, 4,
        2, 4, 2, 10, 12, 2, 4, 2, 10, 2, 6, 4, 2, 4, 6, 6,
        2, 10, 2, 6, 4, 14, 4, 2, 4, 2, 4, 8, 6, 4, 6, 2,
        4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 12, 2
    ];
    
    let mut potential_factor: u128 = 13;
    let mut i = 0;
    let sqrt_number = ((number + 1) as f64).sqrt().ceil() as u128;
    while potential_factor < sqrt_number {
        if number % potential_factor == 0 {
            factors.push(potential_factor);
            number /= potential_factor;
        } else {
            potential_factor += increments[i];
            i = (i + 1) % increments.len();
        }
    }

    if number > 1 {
        factors.push(number);
    }
    Ok(factors)
}

/// Calculate the greatest common divisor of two numbers using the Euclidean algorithm
fn gcd(mut a: u128, mut b: u128) -> u128 {
    // Handle edge cases
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    
    // Euclidean algorithm
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    
    a
}

pub fn pollard_rho(mut number: u128) -> Result<Vec<u128>, String> {
    if number == 0 {
        return Err(String::from("Input must be greater than zero."));
    }

    let mut factors = Vec::with_capacity(24);
    
    // Handle trivial cases
    if number == 1 {
        return Ok(factors);
    }
    
    // Handle even numbers
    while number % 2 == 0 {
        factors.push(2);
        number /= 2;
    }
    
    // Start Pollard's rho algorithm for odd numbers
    let mut x = 2_u128;
    let mut y = 2_u128;
    let mut d = 1_u128;
    
    while d == 1 {
        // Apply the function f(x) = (x² + 1) mod number
        x = (x.wrapping_mul(x) + 1) % number;
        y = (y.wrapping_mul(y) + 1) % number;
        y = (y.wrapping_mul(y) + 1) % number; // Apply twice for y
        
        // Calculate GCD
        d = gcd(if x > y { x - y } else { y - x }, number);
        
        if d == number {
            // Failed to find factor this way
            break;
        }
        
        if d > 1 {
            // Found a factor
            factors.push(d);
            number /= d;
            
            // Recursively factor the remaining part
            if number > 1 {
                // You could recursively call pollard_rho here
                // or just set x and y back to 2 and continue
                x = 2;
                y = 2;
                d = 1;
            }
        }
    }

    if number > 1 {
        factors.push(number);
    }
    
    Ok(factors)
}