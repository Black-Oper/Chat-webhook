pub fn decodificar_json_base64(base64_str: &str) -> Result<String, String> {
    let base64_clean = base64_str.trim_end_matches('=');
    
    let binario = converte_base64_bin(base64_clean);
    
    let binario_agrupado = separa_string_binaria(&binario, 8);
    
    let json_str = binario_para_texto(&binario_agrupado);
    
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_str) {
        Ok(parsed.to_string())
    } else {
        Err("String decodificada não é um JSON válido".to_string())
    }
}

fn converte_base64_bin(string: &str) -> String {
    let str_b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut str_bin = String::new();

    for caracter in string.chars() {
        if caracter == '=' {
            continue;
        }
        let mut i = 0;
        for cbin in str_b64.chars() {
            i += 1;
            if caracter == cbin {
                str_bin.push_str(&format!("{:06b}", i - 1));
                break;
            }
        }
    }
    str_bin
}

fn binario_para_texto(bin: &str) -> String {
    bin.split_whitespace()
       .filter_map(|byte| {
            if byte.len() == 8 {
                u8::from_str_radix(byte, 2).ok().map(|b| b as char)
            } else {
                None
            }
       })
       .collect()
}

pub fn separa_string_binaria(string: &str, num: i32) -> String {
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