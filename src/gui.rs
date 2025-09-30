use crate::worker::do_work;

use eframe;
use eframe::egui; // cargo add eframe egui
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_gui() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Синхронизация",
        options,
        Box::new(|_cc| Ok(Box::new(TheGui::new()))),
    )
    .unwrap();
}

struct TheGui {
    sync_running: Arc<Mutex<bool>>,
    got_error: Arc<Mutex<Option<String>>>, // TODO: allow for many errors ?
    sync_done: Arc<Mutex<bool>>, // TODO: I think it would be better if we had a state machine instead of a billion variables
}

impl TheGui {
    fn new() -> Self {
        TheGui {
            sync_running: Arc::new(Mutex::new(false)),
            got_error: Arc::new(Mutex::new(None)),
            sync_done: Arc::new(Mutex::new(false)),
        }
    }
}

impl eframe::App for TheGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            {
                let got_error = (*self.got_error.lock().unwrap()).clone();

                if let Some(error) = got_error {
                    egui::Window::new("ГРЕШКА")
                        .collapsible(false)
                        .resizable(false)
                        .show(ctx, |ui| {
                            // ui.label(error);

                            ui.label(egui::RichText::new(error).monospace());
                            // .size(16.0)
                            // .color(egui::Color32::WHITE)

                            // if ui.button("close").clicked() {
                            //     self.show_popup = false;
                            // }
                        });

                    ui.label("Грешка");
                    return;
                }
            }

            //////////
            {
                let done = *self.sync_done.lock().unwrap();

                if done {
                    ui.label("Готово");
                    return;
                }
            }

            //////////

            {
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
                let done_clone = Arc::clone(&self.sync_done);

                thread::spawn(move || {
                    if let Err(err) = do_work() {
                        eprintln!("got error:\n{err}");
                        *got_error_clone.lock().unwrap() = Some(err);
                    } else {
                        *done_clone.lock().unwrap() = true;
                        *running_clone.lock().unwrap() = false;
                    }
                });
            }
        });
    }
}
