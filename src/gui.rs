use crate::worker::do_work;

use eframe;
use eframe::egui; // cargo add eframe egui
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_gui() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "GUI Backup",
        options,
        Box::new(|_cc| Ok(Box::new(TheGui::new()))),
    )
    .unwrap();
}

struct TheGui {
    sync_running: Arc<Mutex<bool>>,
    got_error: Arc<Mutex<Option<String>>>, // TODO: allow for many errors ?
}

impl TheGui {
    fn new() -> Self {
        TheGui {
            sync_running: Arc::new(Mutex::new(false)),
            got_error: Arc::new(Mutex::new(None)),
        }
    }
}

impl eframe::App for TheGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let got_error = (*self.got_error.lock().unwrap()).clone();

            if let Some(error) = got_error {
                egui::Window::new("ГРЕШКА")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(error);
                        if ui.button("close").clicked() {
                            // self.show_popup = false;
                        }
                    });

                ui.label("Грешка");
                return;
            }

            //////////

            let running = *self.sync_running.lock().unwrap();

            if running {
                // TODO: add process stdout
                ui.label("Синхронизация...");
                ui.spinner();
                ctx.request_repaint(); // keep animating
                return;
            }

            if !ui.button("Синхронизирай").clicked() {
                return;
            }

            *self.sync_running.lock().unwrap() = true;

            let running_clone = Arc::clone(&self.sync_running);
            let got_error_clone = Arc::clone(&self.got_error);

            thread::spawn(move || {
                if let Err(err) = do_work() {
                    eprintln!("got error: {err}");
                    *got_error_clone.lock().unwrap() = Some(err);
                } else {
                    *running_clone.lock().unwrap() = false;
                }
            });
        });
    }
}
