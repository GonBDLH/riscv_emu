use std::{
    sync::{
        Arc, RwLock,
        mpsc::{self, Receiver, Sender, TryRecvError},
    },
    time::Duration,
};

use eframe::egui::{self, Button};
use egui_extras::{Column, TableBuilder};
use emulator::interpreter::Interpreter;

enum Command {
    Reset,

    SetRun(bool),
    Step,
    Run,

    Quit,

    LoadElf(String),
}

pub struct GuiState {
    interpreter: Arc<RwLock<Interpreter>>,

    cmd_sender: Sender<Command>,
    response_receiver: Receiver<u32>,
    running: bool,
}

impl GuiState {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let (cmd_sender, cmd_receiver) = mpsc::channel();
        let (response_sender, respose_receiver) = mpsc::channel();

        let app = Self {
            interpreter: Arc::new(RwLock::new(Interpreter::default())),
            cmd_sender: cmd_sender,
            response_receiver: respose_receiver,
            running: false,
        };

        let interpreter_clone = app.interpreter.clone();

        std::thread::spawn(move || {
            let mut running = false;

            loop {
                match cmd_receiver.try_recv() {
                    Ok(Command::Reset) => {
                        if !running {
                            interpreter_clone.write().unwrap().reset()
                        }
                    }

                    Ok(Command::SetRun(val)) => running = val,
                    Ok(Command::Step) => {
                        if !running {
                            let mut interpreter = interpreter_clone.write().unwrap();

                            interpreter.emulator_step(Duration::from_nanos(1));
                        }
                    }
                    Ok(Command::Quit) => {
                        break;
                    }
                    Ok(Command::Run) => {
                        if running {
                            let mut interpreter = interpreter_clone.write().unwrap();

                            if let Some(response) = interpreter.run_time(Duration::from_millis(50))
                            {
                                response_sender.send(response).unwrap();
                                running = false;
                            }
                        }
                    }
                    Ok(Command::LoadElf(path)) => {
                        let mut interpreter = interpreter_clone.write().unwrap();

                        interpreter.load_elf(&path);
                    }
                    Err(TryRecvError::Empty) => {}
                    _ => panic!("MPSC Disconected"),
                }
            }
        });

        app
    }
}

impl eframe::App for GuiState {
    fn on_exit(&mut self) {
        self.cmd_sender.send(Command::Quit).unwrap();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.cmd_sender.send(Command::Run).unwrap();
        if let Ok(resp) = self.response_receiver.try_recv() {
            println!("{resp}");
            self.running = false;
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Ok(interp) = self.interpreter.read() {
                egui::Grid::new("layout").show(ui, |ui| {
                    ui.vertical(|ui| {
                        if ui.button("Load ELF").clicked()
                            && let Some(path) = rfd::FileDialog::new().pick_file()
                        {
                            let picked_path = path.display().to_string();
                            self.cmd_sender.send(Command::LoadElf(picked_path)).unwrap();
                        }

                        let step_button = Button::new("Step");
                        if ui.add_enabled(!self.running, step_button).clicked() {
                            self.cmd_sender.send(Command::Step).unwrap();
                        }
                        let run_button = Button::new("Run");
                        if ui.add_enabled(!self.running, run_button).clicked() {
                            self.running = true;
                            self.cmd_sender.send(Command::SetRun(self.running)).unwrap();
                        }

                        let stop_button = Button::new("Stop");
                        if ui.add_enabled(self.running, stop_button).clicked() {
                            self.running = false;
                            self.cmd_sender.send(Command::SetRun(self.running)).unwrap();
                        }

                        let reset_button = Button::new("Reset");
                        if ui.add_enabled(!self.running, reset_button).clicked() {
                            self.cmd_sender.send(Command::Reset).unwrap();
                        }
                        ui.label(format!("{:08X}", interp.core.pc));
                    });

                    ui.vertical(|ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .column(Column::auto().resizable(true))
                            .column(Column::auto().resizable(true))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("Registro");
                                });
                                header.col(|ui| {
                                    ui.heading("Valor");
                                });
                            })
                            .body(|body| {
                                body.rows(20.0, interp.core.registers.len(), |mut row| {
                                    let index = row.index();
                                    row.col(|ui| {
                                        ui.label(format!("x{index}"));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:08X}", interp.core.registers[index]));
                                    });
                                });
                            });
                    });

                    ui.end_row();
                });
            }
        });

        ui.request_repaint();
    }
}
