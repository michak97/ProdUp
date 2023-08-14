use csv::ReaderBuilder;
use std::path::Path;
use serde::Serialize;
use eframe::egui;
use epaint::Shadow;

fn main() -> eframe::Result<()> {
    let tasks = read_csv("tasks.csv");
    println!("{:?}", tasks);
    let options = eframe::NativeOptions::default();
    eframe::run_native("ProdUp", options, Box::new(|_cc| Box::new(ProdUp::new(_cc))))
}
#[derive(Debug, Serialize, Clone, Default)]
struct Task {
    id: u64,
    name: String,
    description: String,
    difficulty: u8,
    done: bool,
}
fn read_csv(path: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let path = Path::new(path);
    if path.exists(){
        let reader_result = ReaderBuilder::new()
            .has_headers(false)
            .from_path(path);
        let reader = match reader_result {
            Ok(reader) => reader,
            Err(err) => return Err(Box::new(err)),
        };
        let mut task = Vec::new();
        for record in reader.into_records(){
            let record = match record {
                Ok(record) => record,
                Err(err) => return Err(Box::new(err)),
            };
            let row: Vec<String> = record.iter().map(|field|field.trim().to_string()).collect();
            task.push(Task {
                id: row[0].parse().unwrap(),
                name: row[1].clone(),
                description: row[2].clone(),
                difficulty: row[3].parse().unwrap(),
                done: row[4].parse().unwrap(),
            });
        }
        Ok(task)
    }else{
        println!("File does not exist");
        let mut task = Vec::new();
        task.push(Task {
            id: 0,
            name: "Task erstellen".to_string(),
            description: "Erstelle eine erste Aufgabe!".to_string(),
            difficulty: 0,
            done: false,
        });
        write_csv(path.to_str().unwrap(), &task).unwrap();
        Ok(task)
    }
}
fn write_csv(path: &str, tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path);
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    for task in tasks {
        writer.serialize(task)?;
    }
    writer.flush()?;
    Ok(())
}

fn add_task(path: &str, task: Task) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_csv(path)?;
    tasks.push(task);
    write_csv(path, &tasks)?;
    Ok(())
}

#[derive(Default)]
struct ProdUp {
    new_task: Task,
}

impl ProdUp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ProdUp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT)).show(ctx, |ui| {
            ui.heading("Aufgaben");
            ui.separator();
            ui.vertical(|ui|{
                egui::Frame::none().fill(egui::Color32::BLACK).show(ui, |ui|{
                    ui.heading("Neue Aufgabe");
                })
            });
        });
        egui::CentralPanel::default().frame(egui::Frame::none().fill(egui::Color32::BLACK)).show(ctx, |ui|{
            ui.heading("ProdUp");
            ui.separator();
            ui.heading("Aufgaben");
            read_csv("tasks.csv").unwrap().iter().for_each(|task|{
                egui::Frame::none
                ui.horizontal(|ui|{
                    ui.label(&task.description.to_string());
                    ui.label(&task.name);
                });
            });
            let name_label = ui.label("Name der Aufgabe:");
            ui.text_edit_singleline(&mut self.new_task.name)
                .labelled_by(name_label.id);
            let description_label = ui.label("Beschreibung der Aufgabe:");
            ui.text_edit_multiline(&mut self.new_task.description)
                .labelled_by(description_label.id);
            ui.add(egui::Slider::new(&mut self.new_task.difficulty, 0..=10)
                .text("Schwierigkeit")
                );
            });
        }
    }
