use modinverse::modinverse;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::FromPrimitive;

pub struct RSA {
    p: BigUint,
    q: BigUint,
    pub_key: BigUint,
}

#[derive(Debug)]
pub enum RSAError {
    BadArgument(String),
    OperationFailure(String),
}

impl RSA {
    /// covert BigUint to i64
    pub fn convert_biguint_i64(n: &BigUint) -> i64 {
        n.to_i64().unwrap()
    }

    pub fn multiplication(&self, a: &BigUint, b: &BigUint) -> Result<BigUint, RSAError> {
        if a == &BigUint::from(0u32) || b == &BigUint::from(0u32) {
            return Err(RSAError::BadArgument("Do not accept zero value".into()));
        }
        Ok(a * b)
    }

    pub fn subtract(&self, a: &BigUint, b: &BigUint) -> Result<BigUint, RSAError> {
        if a < b {
            return Err(RSAError::BadArgument(
                "value a cannot be less than value b".into(),
            ));
        }
        Ok(a - b)
    }

    /// euler totient function, Φ(n) = (p-1)(q-1)
    pub fn euler_totient(&self, p: &BigUint, q: &BigUint) -> BigUint {
        let one_val = BigUint::from(1u32);
        let a = self.subtract(p, &one_val).unwrap();
        let b = self.subtract(q, &one_val).unwrap();
        let phi_n = self.multiplication(&a, &b).unwrap();

        phi_n
    }

    /// euclidean_algorithm, gcd(e, phi(N)) = 1, where 1 < e < phi(n)
    pub fn euclidean_algorithm(&self, a: &BigUint, b: &BigUint) -> Result<bool, RSAError> {
        let mut x = a.clone();
        let mut y = b.clone();
        while y != BigUint::from(0u32) {
            let temp = y.clone();
            y = x % y;
            x = temp;
        }

        if x != BigUint::from(1u32) {
            return Err(RSAError::OperationFailure(
                "public verification exponent e doesn't satisfy gcd(e, (p-1)(q-1)) = 1".into(),
            ));
        }

        Ok(true)
    }

    fn encryption(&self, m: &BigUint) -> BigUint {
        let n = self
            .multiplication(&self.p, &self.q)
            .expect("Do not accept zero value");

        let phi_n = self.euler_totient(&self.p, &self.q);

        let pub_ver_exp = self
            .euclidean_algorithm(&self.pub_key, &phi_n)
            .expect("public verification exponent e doesn't satisfy gcd(e, (p-1)(q-1)) = 1");

        // assert that gcd(e, phi(N)) = 1
        assert_eq!(pub_ver_exp, true);

        m.modpow(&self.pub_key, &n)
    }

    fn decryption(&self, cipher_text: &BigUint) -> BigUint {
        let n = self
            .multiplication(&self.p, &self.q)
            .expect("Do not accept zero value");

        let phi_n = self.euler_totient(&self.p, &self.q);

        // modular inverse of d*e=1(mod Φ(n))
        let d = BigUint::from_i64(
            modinverse(
                Self::convert_biguint_i64(&self.pub_key),
                Self::convert_biguint_i64(&phi_n),
            )
            .unwrap(),
        )
        .unwrap();

        cipher_text.modpow(&d, &n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let rsa = RSA {
            p: BigUint::from(13u32),
            q: BigUint::from(17u32),
            pub_key: BigUint::from(35u32),
        };

        // Encryption: c=m^e mod n, where m is the plain text, e is the public key, n is the phi_n
        let message = BigUint::from(5u32);
        let cipher_text = rsa.encryption(&message);
        assert_eq!(cipher_text, BigUint::from(125u32));

        // Encryption: m=c^d mod n, where m is the plain text
        let decrypt = rsa.decryption(&cipher_text);
        assert_eq!(decrypt, BigUint::from(5u32));
    }

    #[test]
    fn test_large_prime() {
        let rsa = RSA {
            p: BigUint::from(1223u32),
            q: BigUint::from(1987u32),
            pub_key: BigUint::from(948047u32),
        };

        let message = BigUint::from(5u32);
        let cipher_text = rsa.encryption(&message);
        assert_eq!(cipher_text, BigUint::from(915542u32));

        let decrypt = rsa.decryption(&cipher_text);
        assert_eq!(decrypt, BigUint::from(5u32));
    }

    #[test]
    fn test_other_fn_in_the_code() {
        let rsa = RSA {
            p: BigUint::from(13u32),
            q: BigUint::from(17u32),
            pub_key: BigUint::from(35u32),
        };

        // 1. FIND public modulus
        let pub_mod = rsa
            .multiplication(&rsa.p, &rsa.q)
            .expect("Do not accept zero value");
        assert_eq!(pub_mod, BigUint::from(221u32));

        // 2. find phi
        let phi_n = rsa.euler_totient(&rsa.p, &rsa.q);
        assert_eq!(phi_n, BigUint::from(192u32));

        // 3. select e, public verification exponent such that gcd(e, phi(N)) = 1, where 1 < e < phi(n)
        let pub_ver_exp = rsa
            .euclidean_algorithm(&rsa.pub_key, &phi_n)
            .expect("public verification exponent e doesn't satisfy gcd(e, (p-1)(q-1)) = 1");
        assert_eq!(pub_ver_exp, true);

        // 4. find d, private signing key using modular inverse d*e=1(mod phi_n)
        let priv_sign_key = BigUint::from_i64(
            modinverse(
                RSA::convert_biguint_i64(&rsa.pub_key),
                RSA::convert_biguint_i64(&phi_n),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(priv_sign_key, BigUint::from(11u32));
    }
}
