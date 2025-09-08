use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use midir::{MidiInput, MidiInputConnection};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiMessage {
    pub message_type: String,
    pub note: Option<u8>,
    pub velocity: Option<u8>,
    pub control: Option<u8>,
    pub value: Option<u8>,
}

impl MidiMessage {
    fn from_raw_message(message: &[u8]) -> Option<Self> {
        if message.is_empty() {
            return None;
        }

        let status = message[0];
        let message_type = status & 0xF0;

        match message_type {
            0x90 => {
                // Note On
                if message.len() >= 3 {
                    let velocity = message[2];
                    // Velocity 0 is actually Note Off
                    if velocity == 0 {
                        Some(MidiMessage {
                            message_type: "NoteOff".to_string(),
                            note: Some(message[1]),
                            velocity: Some(velocity),
                            control: None,
                            value: None,
                        })
                    } else {
                        Some(MidiMessage {
                            message_type: "NoteOn".to_string(),
                            note: Some(message[1]),
                            velocity: Some(velocity),
                            control: None,
                            value: None,
                        })
                    }
                } else {
                    None
                }
            }
            0x80 => {
                // Note Off
                if message.len() >= 3 {
                    Some(MidiMessage {
                        message_type: "NoteOff".to_string(),
                        note: Some(message[1]),
                        velocity: Some(message[2]),
                        control: None,
                        value: None,
                    })
                } else {
                    None
                }
            }
            0xB0 => {
                // Control Change
                if message.len() >= 3 {
                    Some(MidiMessage {
                        message_type: "ControlChange".to_string(),
                        note: None,
                        velocity: None,
                        control: Some(message[1]),
                        value: Some(message[2]),
                    })
                } else {
                    None
                }
            }
            _ => {
                // Other message types
                Some(MidiMessage {
                    message_type: format!("Unknown({})", message_type),
                    note: None,
                    velocity: None,
                    control: None,
                    value: None,
                })
            }
        }
    }
}

type SharedState = Arc<Mutex<AppState>>;

#[derive(Clone)]
struct AppState {
    midi_sender: broadcast::Sender<MidiMessage>,
}

impl AppState {
    fn new() -> Self {
        let (midi_sender, _) = broadcast::channel(100);
        Self { midi_sender }
    }
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = socket.split();
    let mut midi_receiver = {
        let state_guard = state.lock().unwrap();
        state_guard.midi_sender.subscribe()
    };

    // Task to forward MIDI messages to WebSocket
    let send_task = tokio::spawn(async move {
        while let Ok(midi_message) = midi_receiver.recv().await {
            if let Ok(json) = serde_json::to_string(&midi_message) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Task to handle incoming WebSocket messages
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(_)) => {
                    // Echo or handle client messages if needed
                }
                Ok(Message::Close(_)) => break,
                Err(_) => break,
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "MIDI Backend is running!")
}

fn setup_midi_input(state: SharedState) -> anyhow::Result<Option<MidiInputConnection<()>>> {
    let midi_in = MidiInput::new("midir reading input")?;
    let in_ports = midi_in.ports();

    if in_ports.is_empty() {
        info!("No MIDI input devices found, will use simulation mode");
        return Ok(None);
    }

    let in_port = &in_ports[0];
    info!("Connecting to MIDI device: {}", midi_in.port_name(in_port)?);

    let state_clone = state.clone();
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            if let Some(midi_message) = MidiMessage::from_raw_message(message) {
                let state_guard = state_clone.lock().unwrap();
                if let Err(e) = state_guard.midi_sender.send(midi_message) {
                    error!("Failed to send MIDI message: {}", e);
                }
            }
        },
        (),
    )?;

    Ok(Some(_conn_in))
}

async fn simulate_midi_events(state: SharedState) {
    let c_major_scale = [60, 62, 64, 65, 67, 69, 71, 72]; // C4 to C5
    let mut current_note = 0;

    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;

        let note = c_major_scale[current_note % c_major_scale.len()];

        // Send Note On
        let note_on = MidiMessage {
            message_type: "NoteOn".to_string(),
            note: Some(note),
            velocity: Some(64),
            control: None,
            value: None,
        };

        {
            let state_guard = state.lock().unwrap();
            if let Err(e) = state_guard.midi_sender.send(note_on) {
                error!("Failed to send simulated Note On: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_millis(400)).await;

        // Send Note Off
        let note_off = MidiMessage {
            message_type: "NoteOff".to_string(),
            note: Some(note),
            velocity: Some(0),
            control: None,
            value: None,
        };

        {
            let state_guard = state.lock().unwrap();
            if let Err(e) = state_guard.midi_sender.send(note_off) {
                error!("Failed to send simulated Note Off: {}", e);
            }
        }

        current_note += 1;
    }
}

pub async fn start_midi_server() -> anyhow::Result<()> {
    let state = Arc::new(Mutex::new(AppState::new()));

    // Try to set up real MIDI input
    let _midi_connection = setup_midi_input(state.clone())?;

    // If no MIDI device, start simulation
    if _midi_connection.is_none() {
        info!("Starting MIDI simulation");
        let sim_state = state.clone();
        tokio::spawn(async move {
            simulate_midi_events(sim_state).await;
        });
    }

    let app = Router::new()
        .route("/", get(health_check))
        .route("/ws", get(websocket_handler))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3001".parse::<axum::http::HeaderValue>().unwrap())
                .allow_methods([axum::http::Method::GET])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("MIDI Backend server running on http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
