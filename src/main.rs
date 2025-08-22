mod ui;
use crate::ui::GomokuApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Gomoku",
        options,
        Box::new(|_cc| Ok(Box::new(GomokuApp::default())))
    )
}
