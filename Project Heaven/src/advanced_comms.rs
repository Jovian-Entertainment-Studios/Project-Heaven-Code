use eframe::{egui, epi};
//-----------------------------------------------------------------------------------------------------------------------------------------------
pub struct AdvancedCommsTerminal {
    //-----------------------------------------------------------------------------------------------------------------------------------------------
    pub active: bool,
    //-----------------------------------------------------------------------------------------------------------------------------------------------
}
//-----------------------------------------------------------------------------------------------------------------------------------------------
impl Default for AdvancedCommsTerminal {
    //-----------------------------------------------------------------------------------------------------------------------------------------------
    fn default() -> Self {
        //-----------------------------------------------------------------------------------------------------------------------------------------------
        Self {
            //-----------------------------------------------------------------------------------------------------------------------------------------------
            active: false,
        }
        //-----------------------------------------------------------------------------------------------------------------------------------------------
    }
}
//-----------------------------------------------------------------------------------------------------------------------------------------------
impl AdvancedCommsTerminal {
    //-----------------------------------------------------------------------------------------------------------------------------------------------
    pub fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut epi::Frame<'_>) {
        //-----------------------------------------------------------------------------------------------------------------------------------------------
        egui::trace!(ui);
        //-----------------------------------------------------------------------------------------------------------------------------------------------
        ui.vertical_centered(|ui| {
            //-----------------------------------------------------------------------------------------------------------------------------------------------
            ui.label("this will be a todo list");
            //-----------------------------------------------------------------------------------------------------------------------------------------------
        });
        //-----------------------------------------------------------------------------------------------------------------------------------------------
    }
}
