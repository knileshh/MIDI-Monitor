# MIDI Monitor

A cross-platform desktop application built with Rust, Leptos, and Tauri that monitors MIDI input in real-time and displays it on a virtual piano interface.

## Features

- **Real-time MIDI monitoring**: Connects to MIDI input devices and displays events live
- **Virtual Piano**: Visual piano interface with 1 octave (C4-B4) that highlights active notes
- **WebSocket Communication**: Backend WebSocket server streams MIDI events to the frontend
- **MIDI Simulation**: Automatically simulates MIDI events when no real device is connected
- **Event Logging**: Scrollable log displaying the latest MIDI events with timestamps
- **Cross-platform**: Runs on Windows, macOS, and Linux thanks to Tauri

## Architecture

```
midi-monitor/
├── backend/           # Rust WebSocket server + MIDI handler (midir, axum)
├── frontend/          # Leptos frontend with Tailwind CSS
├── src-tauri/         # Tauri desktop app integration  
└── README.md
```

## Dependencies

### Backend
- `midir` - MIDI input handling
- `axum` - WebSocket server
- `tokio` - Async runtime
- `serde/serde_json` - JSON serialization

### Frontend  
- `leptos` - Reactive web framework
- `wasm-bindgen` - WebAssembly bindings
- `web-sys` - WebSocket client
- Tailwind CSS - Styling

### Desktop App
- `tauri` - Cross-platform desktop app framework

## Prerequisites

Before running this project, make sure you have:

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Trunk** (for building the Leptos frontend)
   ```bash
   cargo install trunk
   ```

3. **Tauri CLI**
   ```bash
   cargo install tauri-cli
   ```

4. **WebAssembly target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Installation & Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd midi-monitor
   ```

2. **Install dependencies**
   All Rust dependencies will be installed automatically when building.

## Running the Application

### Development Mode

There are several ways to run the application:

#### Option 1: Run Full Desktop App (Recommended)
```bash
cargo tauri dev
```
This will:
- Build and serve the Leptos frontend
- Start the MIDI backend server
- Launch the Tauri desktop application
- Auto-reload on file changes

#### Option 2: Run Components Separately

**Terminal 1: Start the backend server**
```bash
cd backend
cargo run
```
Backend will be available at `http://localhost:3000`

**Terminal 2: Start the frontend**
```bash
cd frontend  
trunk serve --port 3001
```
Frontend will be available at `http://localhost:3001`

**Terminal 3: Start Tauri (optional)**
```bash
cargo tauri dev
```

### Production Build

```bash
cargo tauri build
```

This creates a production desktop application in `src-tauri/target/release/`.

## Usage

1. **Launch the application**
   - The app will automatically detect MIDI input devices
   - If no MIDI device is found, it will start simulation mode

2. **MIDI Simulation Mode**
   - Plays a C major scale (C4 to C5) every 500ms
   - Each note has a Note On followed by Note Off event
   - Perfect for testing without physical MIDI hardware

3. **Virtual Piano**
   - White keys: C, D, E, F, G, A, B (C4 to B4)
   - Black keys: C#, D#, F#, G#, A# 
   - Keys turn green when Note On events are received
   - Keys return to normal on Note Off events

4. **Event Log**
   - Shows the latest 50 MIDI events with timestamps
   - Color-coded by event type:
     - Green: Note On events
     - Red: Note Off events  
     - Blue: Control Change events
     - Gray: Other events

5. **Statistics Panel**
   - Total Events: Count of all received MIDI events
   - Active Notes: Number of currently pressed keys

## MIDI Message Structure

The application parses MIDI messages into this structure:

```rust
struct MidiMessage {
    message_type: String, // "NoteOn", "NoteOff", "ControlChange", etc.
    note: Option<u8>,     // MIDI note number (0-127)
    velocity: Option<u8>, // Note velocity (0-127)
    control: Option<u8>,  // Control number (for CC messages)
    value: Option<u8>,    // Control value (for CC messages)
}
```

## Testing

### Backend Tests
```bash
cd backend
cargo test
```

### Simulation Test Function
The backend includes a test function that simulates MIDI events:

```rust
#[tokio::test]
async fn test_simulation() {
    // Creates a test MIDI event and verifies it's received correctly
}
```

## Troubleshooting

### MIDI Device Not Detected
- Ensure your MIDI device is properly connected
- Try restarting the application
- Check that no other applications are using the MIDI device

### WebSocket Connection Issues
- Verify the backend is running on port 3000
- Check firewall settings
- Ensure frontend is configured to connect to `ws://localhost:3000/ws`

### Build Issues
- Make sure all prerequisites are installed
- Update Rust: `rustup update`
- Clear cargo cache: `cargo clean`

## Development

### Project Structure
```
midi-monitor/
├── backend/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs          # WebSocket server + MIDI handling
├── frontend/
│   ├── Cargo.toml
│   ├── Trunk.toml           # Trunk build configuration
│   ├── index.html           # Entry HTML file
│   └── src/
│       └── main.rs          # Leptos frontend app
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json      # Tauri configuration
│   └── src/
│       ├── main.rs          # Tauri main entry point
│       ├── lib.rs           # Tauri app setup
│       └── backend/
│           └── mod.rs       # Integrated backend module
├── Cargo.toml               # Workspace configuration
└── README.md
```

### Adding New Features

1. **Backend Changes**: Edit `backend/src/main.rs` or `src-tauri/src/backend/mod.rs`
2. **Frontend Changes**: Edit `frontend/src/main.rs`
3. **Tauri Integration**: Edit `src-tauri/src/lib.rs`

### Hot Reload

- Frontend: Trunk automatically reloads on changes
- Backend: Use `cargo watch` for auto-restart
- Tauri: `cargo tauri dev` provides hot reload for the full app

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

## Acknowledgments

- [midir](https://github.com/Boddlnagg/midir) - Cross-platform MIDI I/O library
- [Leptos](https://github.com/leptos-rs/leptos) - Reactive web framework for Rust
- [Tauri](https://github.com/tauri-apps/tauri) - Cross-platform desktop app framework
- [Axum](https://github.com/tokio-rs/axum) - Web framework for WebSocket server
