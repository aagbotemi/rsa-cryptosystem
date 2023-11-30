use ethers::types::U256;
use modinverse::modinverse;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::FromPrimitive;

// Function to check if two numbers are coprime
fn euclidean_algorithm(a: &BigUint, b: &BigUint) -> bool {
    let mut x = a.clone();
    let mut y = b.clone();
    while y != BigUint::from(0u32) {
        let temp = y.clone();
        y = x % y;
        x = temp;
    }
    println!("This is it: {}", x);
    return x == BigUint::from(1u32);
    // return x == 1;
}

// USEFUL
pub fn multiplication(a: &BigUint, b: &BigUint) -> BigUint {
    // if a = &BigUint::from(0u32) || b = &BigUint::from(0u32) {
    //     println!("Do not accept zero value")
    // Err("Do not accept zero value")
    // }
    a * b
}

// USEFUL
/// Subtract two elements in the set:
///
/// `a - b = a + (-b) = a mod p`
///
pub fn subtract(a: &BigUint, b: &BigUint) -> BigUint {
    if a < b {
        println!("value a cannot be less than value b")
        // Err("value a cannot be less than value b")
    }
    a - b
}

// USEFUL
fn euler_totient(p: &BigUint, q: &BigUint) -> BigUint {
    let a = subtract(p, &BigUint::from(1u32));
    let b = subtract(q, &BigUint::from(1u32));
    let phi_n = multiplication(&a, &b);

    phi_n
}

// USEFUL
fn calculate_d(phi_n: &BigUint, e: &BigUint) -> Result<BigUint, String> {
    // Calculate d = e^-1 mod p
    let sub_ = subtract(&phi_n, &BigUint::from(2u32));
    // let sub_ = subtract(&phi_n, &BigUint::from(2u32), &phi_n);
    let d_ = e.modpow(&sub_, phi_n);
    // convert_i128_biguint(e).modpow(&convert_i128_biguint(&sub_), &convert_i128_biguint(phi_n));

    Ok(d_)
}

fn convert_biguint_i64(n: &BigUint) -> i64 {
    // BigUint::from_i128(*n).unwrap()
    // BigUint::to_u64_digits(&n)
    n.to_i64().unwrap()
}

fn encryption(m: &BigUint, e: &BigUint, n: &BigUint) -> BigUint {
    // let m_biguint = convert_i128_biguint(m);
    // let e_biguint = convert_i128_biguint(e);
    // let n_biguint = convert_i128_biguint(n);

    // m_biguint.modpow(&e_biguint, &n_biguint)

    m.modpow(&e, &n)
}

fn decryption(c: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    // let d_biguint = convert_i128_biguint(d);
    // let n_biguint = convert_i128_biguint(n);

    // c.modpow(&d_biguint, &n_biguint)
    c.modpow(&d, &n)
}

fn main() {
    // Given values
    let p = BigUint::from(1223u32);
    let q = BigUint::from(1987u32);
    let e = BigUint::from(948047u32);
    // let p = BigUint::from(3u32);
    // let q = BigUint::from(11u32);
    // let e = BigUint::from(7u32);
    // let p = BigUint::from(13u32);
    // let q = BigUint::from(17u32);
    // let e = BigUint::from(35u32);
    let m = BigUint::from(5u32);

    // 1. find n
    let n = multiplication(&p, &q);

    // 2. find phi
    let phi_n = euler_totient(&p, &q);
    println!("phi_n: {}, n: {}", phi_n, n);

    // 3. select e such that gcd(e, phi(N)) = 1, where 1 < e < phi(n)
    let e__ = euclidean_algorithm(&e, &phi_n.clone());
    println!("This is the solution {}", e__);

    println!("THis is our discussion: {:?}", convert_biguint_i64(&e));

    println!("THis is our discussion: {:?}", convert_biguint_i64(&phi_n));

    // 4. select d using modular inverse d*e=1(mod phi_n)
    let d = BigUint::from_i64(
        modinverse(convert_biguint_i64(&e), convert_biguint_i64(&phi_n)).unwrap(),
    )
    .unwrap();
    println!("d: {:?}", d);

    // // note: public key pu = [e,n]
    // // note: private key pr = [d,n]

    // 5. Encryption: c=m^e mod n, where m is the plain text
    let c = encryption(&m, &e, &n);
    println!("encryption: {}", c);

    // // 6. Encryption: m=c^d mod n, where m is the plain text
    let dec = decryption(&c, &d, &n);
    println!("decryption: {}", dec);
}
