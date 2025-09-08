use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket, MessageEvent, ErrorEvent, CloseEvent};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MidiMessage {
    pub message_type: String,
    pub note: Option<u8>,
    pub velocity: Option<u8>,
    pub control: Option<u8>,
    pub value: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
struct MidiEvent {
    message: MidiMessage,
    timestamp: String,
}

impl MidiEvent {
    fn new(message: MidiMessage) -> Self {
        let now = js_sys::Date::new_0();
        let timestamp = format!("{:02}:{:02}:{:02}.{:03}",
            now.get_hours(),
            now.get_minutes(), 
            now.get_seconds(),
            now.get_milliseconds()
        );
        Self { message, timestamp }
    }
}

#[component]
fn Piano(active_notes: ReadSignal<HashMap<u8, bool>>) -> impl IntoView {
    // C4–B4 (12 keys) - includes all white and black keys
    let white_keys = [60, 62, 64, 65, 67, 69, 71]; // C4, D4, E4, F4, G4, A4, B4

    let note_to_name = |note: u8| -> String {
        let names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let octave = (note / 12) - 1;
        format!("{}{}", names[(note % 12) as usize], octave)
    };

    view! {
        <div class="relative inline-block">
            // White keys - horizontal layout
            <div class="flex">
                {white_keys.into_iter().map(|note| {
                    let is_active = move || active_notes.get().get(&note).copied().unwrap_or(false);
                    view! {
                        <div class={move || format!(
                            "bg-white border border-black w-12 h-40 inline-block flex items-end justify-center pb-2 text-xs font-mono piano-key piano-key-transition {}",
                            if is_active() { "!bg-green-500" } else { "" }
                        )}>
                            <span class="text-gray-600">{note_to_name(note)}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            
            // Black keys - overlaid with absolute positioning
            <div class="absolute top-0">
                // C# - positioned between C and D
                {
                    let note = 61u8; // C#
                    let is_active = move || active_notes.get().get(&note).copied().unwrap_or(false);
                    view! {
                        <div class={move || format!(
                            "bg-black w-8 h-24 absolute ml-[-12px] z-10 flex items-end justify-center pb-2 text-xs font-mono piano-key black-key piano-key-transition {}",
                            if is_active() { "!bg-green-700" } else { "" }
                        )} style="left: 32px;">
                            <span class="text-gray-300">{note_to_name(note)}</span>
                        </div>
                    }
                }
                // D# - positioned between D and E  
                {
                    let note = 63u8; // D#
                    let is_active = move || active_notes.get().get(&note).copied().unwrap_or(false);
                    view! {
                        <div class={move || format!(
                            "bg-black w-8 h-24 absolute ml-[-12px] z-10 flex items-end justify-center pb-2 text-xs font-mono piano-key black-key piano-key-transition {}",
                            if is_active() { "!bg-green-700" } else { "" }
                        )} style="left: 80px;">
                            <span class="text-gray-300">{note_to_name(note)}</span>
                        </div>
                    }
                }
                // F# - positioned between F and G
                {
                    let note = 66u8; // F#
                    let is_active = move || active_notes.get().get(&note).copied().unwrap_or(false);
                    view! {
                        <div class={move || format!(
                            "bg-black w-8 h-24 absolute ml-[-12px] z-10 flex items-end justify-center pb-2 text-xs font-mono piano-key black-key piano-key-transition {}",
                            if is_active() { "!bg-green-700" } else { "" }
                        )} style="left: 176px;">
                            <span class="text-gray-300">{note_to_name(note)}</span>
                        </div>
                    }
                }
                // G# - positioned between G and A
                {
                    let note = 68u8; // G#
                    let is_active = move || active_notes.get().get(&note).copied().unwrap_or(false);
                    view! {
                        <div class={move || format!(
                            "bg-black w-8 h-24 absolute ml-[-12px] z-10 flex items-end justify-center pb-2 text-xs font-mono piano-key black-key piano-key-transition {}",
                            if is_active() { "!bg-green-700" } else { "" }
                        )} style="left: 224px;">
                            <span class="text-gray-300">{note_to_name(note)}</span>
                        </div>
                    }
                }
                // A# - positioned between A and B
                {
                    let note = 70u8; // A#
                    let is_active = move || active_notes.get().get(&note).copied().unwrap_or(false);
                    view! {
                        <div class={move || format!(
                            "bg-black w-8 h-24 absolute ml-[-12px] z-10 flex items-end justify-center pb-2 text-xs font-mono piano-key black-key piano-key-transition {}",
                            if is_active() { "!bg-green-700" } else { "" }
                        )} style="left: 272px;">
                            <span class="text-gray-300">{note_to_name(note)}</span>
                        </div>
                    }
                }
            </div>
        </div>
    }
}

#[component]
fn MidiEventLog(events: ReadSignal<Vec<MidiEvent>>) -> impl IntoView {
    view! {
        <div class="bg-gray-100 border rounded-lg p-4 h-64 overflow-y-auto">
            <h3 class="text-lg font-semibold mb-2">"MIDI Event Log"</h3>
            <div class="space-y-1 font-mono text-sm">
                {move || events.get().into_iter().rev().take(50).map(|event| {
                    let color_class = match event.message.message_type.as_str() {
                        "NoteOn" => "text-green-600",
                        "NoteOff" => "text-red-600",
                        "ControlChange" => "text-blue-600",
                        _ => "text-gray-600",
                    };
                    view! {
                        <div class={format!("flex justify-between {}", color_class)}>
                            <span class="font-semibold">{event.timestamp}</span>
                            <span>{format_midi_message(&event.message)}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

fn format_midi_message(msg: &MidiMessage) -> String {
    match msg.message_type.as_str() {
        "NoteOn" => format!("Note On: {} (vel: {})", 
            msg.note.unwrap_or(0), msg.velocity.unwrap_or(0)),
        "NoteOff" => format!("Note Off: {} (vel: {})", 
            msg.note.unwrap_or(0), msg.velocity.unwrap_or(0)),
        "ControlChange" => format!("CC: {} = {}", 
            msg.control.unwrap_or(0), msg.value.unwrap_or(0)),
        _ => format!("{:?}", msg.message_type),
    }
}

#[component]
fn ConnectionStatus(connected: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div class={move || format!(
            "flex items-center space-x-2 px-3 py-2 rounded-lg {}",
            if connected.get() { "bg-green-100 text-green-800" } else { "bg-red-100 text-red-800" }
        )}>
            <div class={move || format!(
                "w-3 h-3 rounded-full {}",
                if connected.get() { "bg-green-500" } else { "bg-red-500" }
            )}></div>
            <span class="font-medium">
                {move || if connected.get() { "Connected" } else { "Disconnected" }}
            </span>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    let (events, set_events) = create_signal(Vec::<MidiEvent>::new());
    let (active_notes, set_active_notes) = create_signal(HashMap::<u8, bool>::new());
    let (connected, set_connected) = create_signal(false);
    let (_websocket, set_websocket) = create_signal(None::<WebSocket>);

    let connect_websocket = move || {
        let ws = WebSocket::new("ws://localhost:3000/ws");
        
        match ws {
            Ok(ws) => {
                let _ws_clone = ws.clone();
                
                // onopen handler
                let onopen_callback = Closure::wrap(Box::new(move |_| {
                    web_sys::console::log_1(&"WebSocket connected".into());
                    set_connected.set(true);
                }) as Box<dyn FnMut(JsValue)>);
                ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
                onopen_callback.forget();

                // onmessage handler
                let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                        let message_str = String::from(text);
                        if let Ok(midi_message) = serde_json::from_str::<MidiMessage>(&message_str) {
                            let event = MidiEvent::new(midi_message.clone());
                            
                            set_events.update(|events| {
                                events.push(event);
                                if events.len() > 100 {
                                    events.remove(0);
                                }
                            });

                            // Update active notes for piano display
                            if let Some(note) = midi_message.note {
                                match midi_message.message_type.as_str() {
                                    "NoteOn" => {
                                        set_active_notes.update(|notes| {
                                            notes.insert(note, true);
                                        });
                                    },
                                    "NoteOff" => {
                                        set_active_notes.update(|notes| {
                                            notes.insert(note, false);
                                        });
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(MessageEvent)>);
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();

                // onerror handler
                let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
                    web_sys::console::log_2(&"WebSocket error:".into(), &e.into());
                    set_connected.set(false);
                }) as Box<dyn FnMut(ErrorEvent)>);
                ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
                onerror_callback.forget();

                // onclose handler
                let onclose_callback = Closure::wrap(Box::new(move |_e: CloseEvent| {
                    web_sys::console::log_1(&"WebSocket closed".into());
                    set_connected.set(false);
                }) as Box<dyn FnMut(CloseEvent)>);
                ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
                onclose_callback.forget();

                set_websocket.set(Some(ws));
            }
            Err(e) => {
                web_sys::console::log_2(&"Failed to create WebSocket:".into(), &e.into());
            }
        }
    };

    // Auto-connect on component mount
    create_effect(move |_| {
        connect_websocket();
    });

    view! {
        <Html lang="en"/>
        <Title text="MIDI Monitor"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        
        <body class="bg-gray-50 min-h-screen">
            <div class="container mx-auto px-4 py-8">
                <header class="mb-8">
                    <div class="flex justify-between items-center">
                        <h1 class="text-3xl font-bold text-gray-800">"MIDI Monitor"</h1>
                        <ConnectionStatus connected/>
                    </div>
                    <p class="text-gray-600 mt-2">
                        "Real-time MIDI event monitoring with virtual piano display"
                    </p>
                </header>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <div class="space-y-6">
                        <div class="bg-white border rounded-lg p-6 shadow-sm">
                            <h2 class="text-xl font-semibold mb-4">"Virtual Piano"</h2>
                            <p class="text-sm text-gray-600 mb-4">
                                "Keys light up green when MIDI Note On events are received"
                            </p>
                            <div class="flex justify-center">
                                <Piano active_notes/>
                            </div>
                        </div>

                        <div class="bg-white border rounded-lg p-6 shadow-sm">
                            <h2 class="text-xl font-semibold mb-4">"Statistics"</h2>
                            <div class="grid grid-cols-2 gap-4 text-center">
                                <div class="bg-blue-50 p-4 rounded">
                                    <div class="text-2xl font-bold text-blue-600">
                                        {move || events.get().len()}
                                    </div>
                                    <div class="text-sm text-gray-600">"Total Events"</div>
                                </div>
                                <div class="bg-green-50 p-4 rounded">
                                    <div class="text-2xl font-bold text-green-600">
                                        {move || active_notes.get().values().filter(|&&v| v).count()}
                                    </div>
                                    <div class="text-sm text-gray-600">"Active Notes"</div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="space-y-6">
                        <div class="bg-white border rounded-lg p-6 shadow-sm">
                            <MidiEventLog events/>
                        </div>

                        <div class="bg-white border rounded-lg p-6 shadow-sm">
                            <h3 class="text-lg font-semibold mb-2">"Instructions"</h3>
                            <ul class="text-sm text-gray-600 space-y-1">
                                <li>"• Connect a MIDI device to see real events"</li>
                                <li>"• Without a device, simulated events will play"</li>
                                <li>"• Green keys indicate active notes"</li>
                                <li>"• Event log shows the latest 50 events"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </body>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
