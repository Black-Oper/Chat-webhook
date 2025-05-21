use axum::{Router, routing::post, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Deserialize};
use chrono::Local;
use std::{env, io::BufRead, net::SocketAddr, thread};
use tokio::net::TcpListener;
use reqwest::blocking::Client;


// define a estrutura do JSON que será enviado
#[derive(Serialize, Deserialize)]
struct ChatMessage {
    username: String,
    text: String,
    timestamp: String,
}

// Função assíncrona que processa a mensagem recebida
// Tenta parsear o corpo da requisição como JSON para ChatMessage
// Se bem-sucedido, imprime a mensagem formatada no console
// Retorna códigos de status HTTP apropriados
async fn handle_message(body: String) -> impl IntoResponse {
    match serde_json::from_str::<ChatMessage>(&body) {
        Ok(msg) => {
            println!("{} [{}]: {}", msg.username, msg.timestamp, msg.text);
            StatusCode::OK
        }
        Err(err) => {
            eprintln!("JSON inválido: {}", err);
            StatusCode::BAD_REQUEST
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Uso: {} <porta> <url_do_outro> <seu_nome>", args[0]);
        std::process::exit(1);
    }
    let port: u16 = args[1].parse().expect("Porta inválida");
    let peer_url  = args[2].clone();
    let username  = args[3].clone();

    let app = Router::new().route("/message", post(handle_message));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Servidor na porta {} — enviando para {}", port, peer_url);

    // Cria uma thread separada para não bloquear o servidor
    // Lê continuamente da entrada padrão
    // Para cada linha não vazia, cria um ChatMessage
    thread::spawn(move || {
        let stdin = std::io::stdin();
        let http  = Client::new();

        for line in stdin.lock().lines() {

            let text = match line { Ok(t) => t, Err(_) => break };
            if text.trim().is_empty() { continue; }

            let chat_msg = ChatMessage {
                username: username.clone(),
                text,
                timestamp: Local::now().to_rfc3339(),
            };

            if let Err(e) = http.post(&peer_url).json(&chat_msg).send() {
                eprintln!("Falha no POST: {}", e);
            }
        }
    });

    // Inicia o servidor HTTP na porta especificada
    // Usa TcpListener para escutar na porta especificada
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
