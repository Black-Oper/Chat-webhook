pub fn converter_json_base64(json: &str) -> String {
    let binario = converter_binario(json);
    
    let binario_separado = separa_string_binaria(&binario, 6);
    
    converte_bin_base64(&binario_separado)
}

pub fn converter_binario(string: &str) -> String {
    string.bytes()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<String>>()
        .join("")
}

pub fn separa_string_binaria(string: &str, num: usize) -> String {
    let mut str_bin_separada = String::new();
    let mut i = 0;

    for caractere in string.chars() {
        str_bin_separada.push(caractere);
        i += 1;

        if i == num {
            i = 0;
            str_bin_separada.push(' ');
        }
    }

    str_bin_separada
}

pub fn converte_bin_base64(string: &str) -> String {
    let str_b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut contador = 0;
    let mut i = 0;
    let mut string_b64 = String::new();

    for caractere in string.chars() {
        if caractere == ' ' {
            string_b64.push(str_b64.chars().nth(contador).unwrap());
            contador = 0;
            i = 0;
        } else {
            i += 1;
            if caractere == '1' {
                match i {
                    1 => contador += 32,
                    2 => contador += 16,
                    3 => contador += 8,
                    4 => contador += 4,
                    5 => contador += 2,
                    6 => contador += 1,
                    _ => unreachable!("Erro inesperado no match"),
                }
            }
        }
    }

    if i > 0 {
        string_b64.push(str_b64.chars().nth(contador).unwrap());
    }

    let pad = (4 - (string_b64.len() % 4)) % 4;
    for _ in 0..pad {
        string_b64.push('=');
    }

    string_b64
}