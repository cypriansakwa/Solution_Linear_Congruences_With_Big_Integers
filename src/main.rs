use std::vec::Vec;
use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive};

fn main() {
    let a: BigInt = "5074444444443393839383923433277777777444442".parse().unwrap();
    let b: BigInt = "1838383833838383837899999999999999999998".parse().unwrap();
    let n: BigInt = "71338888888888828282828287777777777777774".parse().unwrap();

    match solve_linear_congruence(&a, &b, &n) {
        Some(solutions) => {
            println!("Solutions to {}x ≡  {} (mod {}): {:?}", a, b, n, solutions);
        }
        None => {
            println!("The congruence {}x ≡  {} (mod {}) has no solutions", a, b, n);
        }
    }
}

fn gcd_extended(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        return (b.clone(), BigInt::zero(), BigInt::one());
    }
    let (g, x1, y1) = gcd_extended(&(b % a), a);
    let x = y1 - (b / a) * &x1;
    let y = x1;
    (g, x, y)
}

fn mod_inverse(a: &BigInt, n: &BigInt) -> Option<BigInt> {
    let (g, x, _) = gcd_extended(a, n);
    if !g.is_one() {
        None
    } else {
        Some((x % n + n) % n)
    }
}

fn solve_linear_congruence(a: &BigInt, b: &BigInt, n: &BigInt) -> Option<Vec<BigInt>> {
    let (g, _x, _) = gcd_extended(a, n); // Prefix `_x` to suppress warning

    if b % &g != BigInt::zero() {
        return None; // No solutions
    }

    let a_prime = a / &g;
    let b_prime = b / &g;
    let n_prime = n / &g;

    let x0 = (b_prime * mod_inverse(&a_prime, &n_prime)?) % &n_prime;
    let mut solutions = Vec::new();

    for i in 0..g.to_usize().unwrap() {
        solutions.push((x0.clone() + i * &n_prime) % n);
    }

    Some(solutions)
}

