use crate::networking;
use crate::networking::connect;
use eframe::egui::{CentralPanel, Context, Slider};
use eframe::epi::{App, Frame};
use log::debug;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

enum AppState {
    Connecting(String, bool),
    Connected(Option<JoinHandle<()>>, Sender<u8>, Receiver<f32>, u8, f32),
}

pub struct MandosClient {
    state: AppState,
    new_state: Option<AppState>,
}

impl App for MandosClient {
    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        if let Some(state) = self.new_state.take() {
            self.state = state;
        }

        match &mut self.state {
            AppState::Connecting(input_string, connection_failed_before) => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.label("Server Address");
                    ui.text_edit_singleline(input_string);
                    if ui.button("Connect").clicked() {
                        match connect(input_string) {
                            Ok(stream) => {
                                debug!("Ok");
                                let (tx1, rx1) = mpsc::channel();
                                let (tx2, rx2) = mpsc::channel();
                                let networking_thread = networking::worker(stream, rx1, tx2);
                                self.new_state = Some(AppState::Connected(
                                    Some(networking_thread),
                                    tx1,
                                    rx2,
                                    2,
                                    -1.0,
                                ))
                            }
                            Err(_) => {
                                debug!("Err");
                                *connection_failed_before = true
                            }
                        }
                    }
                    if *connection_failed_before {
                        ui.label("Connection Failed");
                    }
                });
            }
            AppState::Connected(_, tx, rx, secs, moisture) => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.label("Connected");
                    if ui.button("Water").clicked() {
                        tx.send(*secs).unwrap();
                    }
                    ui.add(Slider::new(secs, 1 as u8..=100).text("length"));
                    if let Ok(v) = rx.try_recv() {
                        *moisture = v;
                    }
                    ui.label(format!("{}", moisture));
                });
            }
        }
    }

    fn name(&self) -> &str {
        "Mandos-Client"
    }
}

impl Default for MandosClient {
    fn default() -> Self {
        Self {
            state: AppState::Connecting(String::from("localhost"), false),
            new_state: None,
        }
    }
}
