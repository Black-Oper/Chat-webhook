fn msg_to_rsa(msg: &[u8], n: &u64, e: &u64) -> Vec<u64> {
    let mut cypher_msg = vec![0u64; msg.len()];

    for i in 0..msg.len() {
        let m = msg[i] as u64;

        let mut aux = 1u128;
        for _j in 0..*e {
            aux = (aux * m as u128) % (*n as u128);
        }
        cypher_msg[i] = aux as u64;
    }

    cypher_msg
}

fn concatenate_numbers(numbers: &[u64]) -> String {
    numbers.iter()
        .map(|&num| num.to_string())
        .collect()
}

pub fn encrypt(input: &[u8], n: &u64, e: &u64) -> String {

    let cypher_numbers = msg_to_rsa(input, &n, &e);

    concatenate_numbers(&cypher_numbers)
}