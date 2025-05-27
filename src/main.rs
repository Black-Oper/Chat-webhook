use std::{
    env,
    net::SocketAddr,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};
use chrono::Local;
use eframe::egui;
use reqwest::blocking::Client;
use tokio::runtime::Runtime;

mod jwt;
use jwt::read_jwt::read_jwt;
use jwt::{generate_jwt::generate_jwt, structs::ChatMessage};

// Função que o servidor Axum usa para lidar com mensagens recebidas
async fn handle_message(
    body: String,
    tx: Sender<ChatMessage>, // Canal para enviar mensagens à GUI
) -> impl IntoResponse {
    let token_str: String = match serde_json::from_str(&body) {
        Ok(s) => s,
        Err(_) => return (StatusCode::BAD_REQUEST, "JSON inválido").into_response(),
    };

    match read_jwt(&token_str) {
        Ok(payload_str) => match serde_json::from_str::<ChatMessage>(&payload_str) {
            Ok(msg) => {
                // Envia a mensagem para a thread da GUI para ser exibida
                if tx.send(msg).is_err() {
                    eprintln!("Falha ao enviar mensagem para a GUI.");
                }
                StatusCode::OK.into_response()
            }
            Err(_) => (StatusCode::BAD_REQUEST, "Payload JWT inválido").into_response(),
        },
        Err(_) => (StatusCode::BAD_REQUEST, "Erro na validação do JWT").into_response(),
    }
}

// Estrutura principal da nossa aplicação GUI
struct ChatApp {
    username: String,
    peer_url: String,
    http_client: Client,
    messages: Vec<String>,
    input_text: String,
    message_receiver: Receiver<ChatMessage>,
}

impl ChatApp {
    // Cria uma nova instância da aplicação de chat
    fn new(
        username: String,
        peer_url: String,
        message_receiver: Receiver<ChatMessage>,
    ) -> Self {
        Self {
            username,
            peer_url,
            http_client: Client::new(),
            messages: Vec::new(),
            input_text: String::new(),
            message_receiver,
        }
    }
}

// Implementa a lógica da interface gráfica para a nossa aplicação
impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Verifica se há novas mensagens recebidas do servidor
        if let Ok(msg) = self.message_receiver.try_recv() {
            self.messages
                .push(format!("[{}]: {}", msg.username, msg.text));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chat em Rust com Webhook");

            // Área de rolagem para exibir as mensagens
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        for msg in &self.messages {
                            ui.label(msg);
                        }
                    });
                });

            // Separador
            ui.separator();

            // Área para entrada de texto e botão de enviar
            ui.horizontal(|ui| {
                let text_edit = ui.text_edit_singleline(&mut self.input_text);
                if text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    // Lógica para enviar ao pressionar Enter
                    if !self.input_text.trim().is_empty() {
                        let chat_msg = ChatMessage {
                            username: self.username.clone(),
                            text: self.input_text.clone(),
                            timestamp: Local::now().to_rfc3339(),
                        };

                        let token = generate_jwt(&chat_msg).unwrap_or_default();
                        if let Err(e) = self.http_client.post(&self.peer_url).json(&token).send() {
                            eprintln!("Falha no POST: {}", e);
                        }
                        self.messages.push(format!("[{}]: {}", self.username, self.input_text));
                        self.input_text.clear();
                        text_edit.request_focus(); // Mantém o foco no input
                    }
                }
                
                if ui.button("Enviar").clicked() {
                    if !self.input_text.trim().is_empty() {
                        let chat_msg = ChatMessage {
                            username: self.username.clone(),
                            text: self.input_text.clone(),
                            timestamp: Local::now().to_rfc3339(),
                        };

                        let token = generate_jwt(&chat_msg).unwrap_or_default();
                        if let Err(e) = self.http_client.post(&self.peer_url).json(&token).send() {
                            eprintln!("Falha no POST: {}", e);
                        }
                        self.messages.push(format!("[{}]: {}", self.username, self.input_text));
                        self.input_text.clear();
                    }
                }
            });
        });

        // Solicita uma nova atualização para manter a interface responsiva
        ctx.request_repaint();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Uso: {} <porta> <url_do_outro> <seu_nome>", args[0]);
        std::process::exit(1);
    }

    let port: u16 = args[1].parse().expect("Porta inválida");
    let peer_url = args[2].clone();
    let username = args[3].clone();

    // Cria um canal para comunicação entre o servidor e a GUI
    let (tx, rx): (Sender<ChatMessage>, Receiver<ChatMessage>) = mpsc::channel();

    // Inicia o servidor Axum em uma thread separada
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let app = Router::new().route(
                "/message",
                post(move |body: String| handle_message(body, tx.clone())),
            );
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            println!("Servidor ouvindo na porta {}", port);

            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    });

    // Configurações da janela da GUI
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chat Rust",
        native_options,
        Box::new(|_cc| Box::new(ChatApp::new(username, peer_url, rx))),
    )
    .expect("Falha ao iniciar a GUI");
}