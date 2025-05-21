fn mod_pow(mut base: u128, mut exp: u64, modulus: u128) -> u128 {
    let mut result = 1u128;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

fn rsa_to_msg(cipher: &[u64], n: u64, d: u64) -> Vec<u8> {
    cipher
        .iter()
        .map(|&c| {
            let decrypted = mod_pow(c as u128, d, n as u128);
            decrypted as u8
        })
        .collect()
}


fn read_keys() -> (u64, u64) {
    let content = fs::read_to_string("src/key/keys.key")
        .expect("Erro ao ler o arquivo");

    let mut lines = content.lines();

    let n = lines.next().unwrap().parse::<u64>().unwrap();
    let _ = lines.next().unwrap();
    let d = lines.next().unwrap().parse::<u64>().unwrap();

    (n, d)
}

fn bytes_to_str(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec())
        .unwrap_or_else(|e| {
            eprintln!("Erro ao converter bytes para UTF‑8: {}", e);
            String::new()
        })
}

pub fn decrypt(input: &str, arq: bool) {
    let arr = decodificar_string_base64(input);

    let (n, d) = read_keys();

    let msg_bytes = rsa_to_msg(&arr, n, d);

    if arq {

        let mut nome = String::new();
        print!("Informe o nome do arquivo: ");
        stdout().flush().unwrap();
        std::io::stdin().read_line(&mut nome).unwrap();
        let nome = nome.trim();

        let file_path = format!(
            "/Users/pedromiguel/Documents/Estudos/5 semestre/Cripto/rsa/src/output/{}",
            nome
        );

        let mut file = File::create(&file_path)
            .expect("Não foi possível criar o arquivo");
        file.write_all(&msg_bytes)
            .expect("Falha ao escrever bytes no arquivo");
        file.flush()
            .expect("Falha ao finalizar a escrita no arquivo");
        
        println!("Arquivo '{}' gravado com sucesso ({} bytes).", nome, msg_bytes.len());
    } else {
        let texto = bytes_to_str(&msg_bytes);
        println!("Mensagem descriptografada: {}", texto);
    }
}