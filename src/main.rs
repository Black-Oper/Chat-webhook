use axum::{Router, routing::post, http::StatusCode, response::IntoResponse};
use chrono::Local;
use std::{env, io::BufRead, net::SocketAddr, thread};
use tokio::net::TcpListener;
use reqwest::blocking::Client;

mod jwt;
use jwt::generate_jwt::generate_jwt;
use jwt::read_jwt::read_jwt;
use jwt::structs::ChatMessage;

// Função assíncrona que processa a mensagem recebida
// Tenta parsear o corpo da requisição como JSON para ChatMessage
// Se bem-sucedido, imprime a mensagem formatada no console
// Retorna códigos de status HTTP apropriados
// main.rs
async fn handle_message(body: String) -> impl IntoResponse {
    // Primeiro, desserialize a string do token do corpo JSON
    let token_str: String = match serde_json::from_str(&body) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Falha ao desserializar o token do corpo JSON: {}", e);
            // Retorne um erro explícito para indicar que o corpo da requisição não era o esperado
            return (StatusCode::BAD_REQUEST, format!("Corpo da requisição JSON inválido: {}", e)).into_response();
        }
    };

    // Agora passe a string JWT real para read_jwt
    match read_jwt(&token_str) { // Alterado de &body para &token_str
        Ok(ref payload_str) => {
            match serde_json::from_str::<ChatMessage>(payload_str) {
                Ok(msg) => {
                    println!("{} [{}]: {}", msg.username, msg.timestamp, msg.text);
                    StatusCode::OK.into_response()
                }
                Err(err) => {
                    eprintln!("JSON inválido no payload do JWT: {}", err);
                    (StatusCode::BAD_REQUEST, format!("Payload JWT inválido: {}", err)).into_response()
                }
            }
        }
        Err(err) => {
            eprintln!("Erro ao ler JWT: {}", err);
            // Forneça o erro específico de read_jwt na resposta
            (StatusCode::BAD_REQUEST, format!("Erro na validação do JWT: {}", err)).into_response()
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
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

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

            let token = generate_jwt(&chat_msg).unwrap_or_else(|err| {
                eprintln!("Erro ao gerar JWT: {}", err);
                String::new()
            });

            if let Err(e) = http.post(&peer_url).json(&token).send() {
                eprintln!("Falha no POST: {}", e);
            }
        }
    });

    // Inicia o servidor HTTP na porta especificada
    // Usa TcpListener para escutar na porta especificada
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
