#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use std::{process::{Command, Stdio}, sync::mpsc::{Receiver, channel}, thread, time::Duration};


use eframe::egui;
use egui::RichText;

use crate::models::service::Service;

mod models;
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}", format!($($arg)*));
    };
}
fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Skip until we find a letter (the command character)
                while let Some(&next_char) = chars.peek() {
                    chars.next();
                    if next_char.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

fn main() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1250.0, 850.0]).with_maximized(true),        
        ..Default::default()
    };
    eframe::run_native(
        "PM2 Manager",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    receiver: Receiver<Vec<Service>>,
    current_value: Vec<Service>,
    selected_service_name: Option<String>,
    logs_content: String,
    logs_error_content:String
}



impl Default for MyApp {
    fn default() -> Self {
        let (sender, receiver) = channel();
        
        // Thread solo envía
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(300));
                let res = Command::new("pm2").arg("ls").arg("--no-color").output().expect("Error ejecutando comando pm2");
                let parsed = String::from_utf8(res.stdout).expect("Error parseando comando ");
                let mut splitted = parsed.as_str().split("\n").collect::<Vec<&str>>();
                // log!("{:?}", splitted);
                
                // splitted.drain(0..6);
                while !splitted[0].starts_with("├─"){
                    splitted.remove(0);
                }
                splitted.remove(0);
                splitted.drain(splitted.len()-2..);
                // splitted.clone().iter().for_each(|s|{

                //         log!("{:?}", s);
                //     });
                if splitted.len() > 0{

                
                    let resplitted = splitted.iter().map(|s|s.split("│").collect::<Vec<&str>>());
                    let trimmed = resplitted.map(|s|s.iter().map(|ss|ss.trim().to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
                    
                    
// trimmed.clone().iter().for_each(|s|{

//                         log!("{:?}", s);
//                     });
                    let prog_arr:Vec<Service> = trimmed.clone().iter().map(|s|{
                         log!("{:?}", s);
                        Service::new(s[1].clone(), s[2].clone(), s[9].clone(), s[8].clone(), s[7].clone())
                    }).collect();               
                    

                    sender.send(prog_arr).ok();
                }
            }
        });
        
        Self {
            receiver,
            current_value: Vec::new(),
            selected_service_name: None,
            logs_content: String::new(),
            logs_error_content: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // ctx.request_repaint_after(Duration::from_millis(500));
        ctx.request_repaint();
        egui::SidePanel::left("list").show(ctx, |ui| {
            if let Ok(new_value) = self.receiver.try_recv() {
                // log!("Received");
                ctx.request_repaint();
                self.current_value = new_value;
            }
            ui.horizontal(|ui|{

                ui.heading("PM2 Manager");
                if ui.button("Restart").clicked(){
                    log!("Restart Clicked");
                    Command::new("pm2").args(["restart", "all"]).stdout(Stdio::null())  // Ignorar salida
        .stderr(Stdio::null()).spawn().unwrap();
                }
                if ui.button("Stop all").clicked(){
                    log!("Stop Clicked");
                    Command::new("pm2").args(["stop", "all"]).stdout(Stdio::null())  // Ignorar salida
        .stderr(Stdio::null()).spawn().unwrap();
                }
                if ui.button("Reset Stats").clicked(){
                        
                        Command::new("pm2").args(["reset", "all"]).stdout(Stdio::null())  // Ignorar salida
        .stderr(Stdio::null()).spawn().unwrap();
                    }
            });
  
           egui::Grid::new("my_grid")
            .striped(true)
            .min_col_width(15.0)
            .max_col_width(200.0)
            .show(ui, |ui| {
                ui.label("id");
                ui.label("name");
                ui.label("is online");
                ui.label("time up");
                ui.label("restarts");
                ui.end_row();
                for row in 0..self.current_value.len() {
                    ui.label(self.current_value[row].id.clone());
                    ui.label(self.current_value[row].name.clone());
                    
                    ui.label(RichText::new(self.current_value[row].online.clone()).color(
                        if self.current_value[row].online == "online" {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        }
                    ));
                    ui.label(self.current_value[row].time.clone());
                    ui.label(self.current_value[row].errors.clone());
                    if self.current_value[row].online == "stopped"{
                        if ui.button("Start").clicked(){
                            log!("Starting {}", self.current_value[row].name);
                            Command::new("pm2").args(["start", self.current_value[row].id.as_str()]).output().unwrap();
                        }
                    }else{                        
                        if ui.button("Stop").clicked(){
                            log!("Stopping {}", self.current_value[row].name);
                            Command::new("pm2").args(["stop", self.current_value[row].id.as_str()]).output().unwrap();
                        }
                    }
                    if ui.button("Logs").clicked(){
                        self.selected_service_name = Some(self.current_value[row].name.clone());
                        
                        let mut servicename = self.current_value[row].name.clone();
                        log!("Fetching logs for {}", servicename);
                        servicename = servicename.replace("[", "-");
                        servicename = servicename.replace("] ", "--");
                        // let home = env::var("HOME").expect("HOME no está definida");
                        // let log_pathout = format!("{}/.pm2/logs/{}-{}.log", home, servicename, "out");
                        // let log_patherr = format!("{}/.pm2/logs/{}-{}.log", home, servicename, "err");
                        
                        // // Usar tail -n 200 en lugar de tail -f para evitar bloqueo
                        // let logs_output = Command::new("tail")
                        //     .args(["-n", "200", log_pathout.as_str(), log_patherr.as_str()])
                        //     .output();

                        let logs_output = Command::new("pm2")
                            .args(["logs", self.current_value[row].id.as_str(), "--lines=100", "--nostream", "--out" ,"--no-color"])
                            .output();
                        let logs_error_output = Command::new("pm2")
                            .args(["logs", self.current_value[row].id.as_str(), "--lines=100", "--nostream", "--err", "--no-color"])
                            .output();
                        
                        if let Ok(output) = logs_output {
                            let raw_content = String::from_utf8(output.stdout).unwrap_or_else(|_| "Error al parsear logs".to_string());
                            self.logs_content = strip_ansi_codes(&raw_content);
                            
                        } else {
                            self.logs_content = format!("Error al obtener logs de: {} ",  servicename);
                        }
                        if let Ok(output) = logs_error_output {
                            let raw_content = String::from_utf8(output.stdout).unwrap_or_else(|_| "Error al parsear logs de error".to_string());
                            self.logs_error_content = strip_ansi_codes(&raw_content);
                            
                        } else {
                            self.logs_error_content = format!("Error al obtener logs de error de: {} ",  servicename);
                        }
                        ctx.request_repaint();
                    }
                    if ui.button("Restart").clicked(){
                        log!("Restarting {}", self.current_value[row].name);
                        Command::new("pm2").args(["restart", self.current_value[row].id.as_str()]).output().unwrap();
                    }
                    

                    ui.end_row();
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(service_name) = &self.selected_service_name {
                ui.heading(format!("Logs de: {}", service_name));
                ui.separator();
                
                let available_height = ui.available_height();
                
                // Usar columnas horizontales
                ui.columns(2, |columns| {
                    // Columna izquierda - Output (stdout)
                    columns[0].group(|ui| {
                        ui.label("📄 Output (stdout)");
                        egui::ScrollArea::vertical()
                            .id_salt("logs_out")
                            .max_height(available_height - 60.0)
                            .auto_shrink([false; 2])
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.y = 6.0;
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.logs_content.as_str())
                                        .code_editor()
                                        .desired_width(f32::INFINITY)
                                );
                            });
                    });
                    
                    // Columna derecha - Error (stderr)
                    columns[1].group(|ui| {
                        ui.label("⚠️ Error (stderr)");
                        egui::ScrollArea::vertical()
                            .id_salt("logs_err")
                            .max_height(available_height - 60.0)
                            .auto_shrink([false; 2])
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.logs_error_content.as_str())
                                        .code_editor()
                                        .desired_width(f32::INFINITY)
                                );
                            });
                    });
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(200.0);
                    ui.heading("Selecciona un servicio para ver sus logs");
                });
            }
        });
        
    }
}