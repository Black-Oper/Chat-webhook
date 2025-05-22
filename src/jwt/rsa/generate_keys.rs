use std::fs::File;
use std::io::Write;

use rand::Rng;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0{
        return false;
    }

    let lim = (n as f64).sqrt() as u64;
    let mut i = 3;
    while i <= lim {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn random_prime(min: u64, max: u64) -> u64 {
    
    let mut rng = rand::rng();
    loop {
        let cand = rng.random_range(min..=max);
        if is_prime(cand) {
            return cand;
        }
    }

}

fn mdc(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    mdc(b, a % b)
}

fn generate_e(phin: u64) -> u64{
    let e: u64 = 65537;

    if mdc(e, phin) == 1 {
        return e;
    }

    for e in 1..phin {
        if mdc(e, phin) == 1 {
            return e;
        }
    }

    panic!("Não encontrou o e...");
}

fn generate_d(e: u64, phin: u64) -> u64 {
    for d in 1..phin {
        if (e * d) % phin == 1 {
            return d;
        }
    }
    panic!("Não encontrou o d...")
}

pub fn generate_keys() {
    
    let p = random_prime(2, 10000);
    let mut q: u64;
    
    loop {
        q = random_prime(2, 10000);
        if p != q {
            break;
        }
    }
    
    let n = p * q;
    let phin = (p - 1) * (q - 1);
    
    let e = generate_e(phin);
    let d = generate_d(e, phin);
    
    let keys: [u64; 3] = [n, e, d];
    let mut file = File::create("src/key.txt")
        .expect("Não foi possível criar o arquivo de chaves");

    for &value in &keys {
        writeln!(file, "{}", value)
            .expect("Falha ao escrever no arquivo de chaves");
    }

    file.flush()
        .expect("Falha ao finalizar a escrita no arquivo de chaves");
}