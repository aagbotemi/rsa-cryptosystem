# RSA-Crptosystem
RSA (Rivest–Shamir–Adleman) is a public-key cryptosystem. It works through the following steps: 

- 1. find public modulus, n = p * q
- 2. find euler totient function, Φ(n) = (p-1)(q-1)
- 3. select e, public verification exponent such that gcd(e, Φ(n)) = 1, where 1 < e < Φ(n)
- 4. find d, private signing key using modular inverse d*e=1(mod Φ(n))
- 5. Encryption, c=m^e mod n, where m is the plain text, e is the public key, n
- 6. Decryption, m=c^d mod n, where m is the plain text

## Clone this repository:
```
git clone https://github.com/aagbotemi/prime-number-generator.git
```

Get into the project directory

## Build the project
```
cargo build
```

## Usage 
To test the implementation, run:
```
cargo test
```

## Contributing
Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License
This project is licensed under the MIT License.