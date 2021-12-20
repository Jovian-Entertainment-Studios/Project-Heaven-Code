use eframe::{egui, epi};
use egui::plot::{HLine, Line, Plot, Points, VLine, Value, Values};

mod point_manipulation;
use point_manipulation::acceleration;
//-----------------------------------------------------------------------------------------------------------------------------------------------
pub struct UniverseStarchart {
    //-----------------------------------------------------------------------------------------------------------------------------------------------
    pub active: bool,
    //-----------------------------------------------------------------------------------------------------------------------------------------------
}
//-----------------------------------------------------------------------------------------------------------------------------------------------
impl Default for UniverseStarchart {
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
impl UniverseStarchart {
    //-----------------------------------------------------------------------------------------------------------------------------------------------
    pub fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut epi::Frame<'_>) {
        //-----------------------------------------------------------------------------------------------------------------------------------------------
        egui::trace!(ui);
        //-----------------------------------------------------------------------------------------------------------------------------------------------
        ui.vertical_centered(|ui| {
            //-----------------------------------------------------------------------------------------------------------------------------------------------
            let system = String::from("Epsilon Eridani");
            let star_type = String::from("K2");
            let star_sequence = String::from("V");
            let mass = String::from("0.82");
            let luminosity = String::from("0.34");
            ui.add(egui::Label::new(format!("{}", system)).heading());
            ui.separator();
            ui.group(|ui| {
                //Some description
                if star_sequence == String::from("V") {
                    ui.add(egui::Label::new(format!(
                        "{} is a {} type 
                    main-sequence star 
                    with a mass of {} 
                    and a luminosity of {}",
                        system, star_type, mass, luminosity
                    )));
                }
            });
            ui.separator();
            ui.group(|ui| {
                //Velocity graph
                let vel_point1 = Points::new(Values::from_values(vec![Value::new(0., 0.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let vel_point2 = Points::new(Values::from_values(vec![Value::new(1., 2.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let vel_point3 = Points::new(Values::from_values(vec![Value::new(2., 0.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let vel_point4 = Points::new(Values::from_values(vec![Value::new(3., -2.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let vel_point5 = Points::new(Values::from_values(vec![Value::new(4., 0.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                /*
                let function = velocity(
                    vel_point1.values().1,
                    vel_point2.values().1,
                    vel_point3.values().1,
                    vel_point4.values().1,
                    vel_point5.values().1,
                    vel_point1.values().2,
                    vel_point2.values().2,
                    vel_point3.values().2,
                    vel_point4.values().2,
                    vel_point5.values().2,
                );

                let function_egui = (0..4).map(|i| {
                    let x = i as f64 * 0.01;
                    Value::new(x, function)
                });
                let line = Line::new(Values::from_values_iter(function_egui));
                */
                ui.add(
                    Plot::new("my_plot")
                        .points(vel_point1)
                        .points(vel_point2)
                        .points(vel_point3)
                        .points(vel_point4)
                        .points(vel_point5)
                        .hline(HLine::new(0.))
                        .vline(VLine::new(0.))
                        .allow_drag(false)
                        .allow_zoom(true)
                        .show_background(true)
                        .height(200.)
                        .width(600.),
                );
                // .line(line)
            });
            ui.group(|ui| {
                //Acceleration graph
                let acc_point1 = Points::new(Values::from_values(vec![Value::new(0., 0.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let acc_point2 = Points::new(Values::from_values(vec![Value::new(1., 2.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let acc_point3 = Points::new(Values::from_values(vec![Value::new(2., 0.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let acc_point4 = Points::new(Values::from_values(vec![Value::new(3., -2.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let acc_point5 = Points::new(Values::from_values(vec![Value::new(4., 0.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                /*
                let function = acceleration(
                    acc_point1.values().1,
                    acc_point2.values().1,
                    acc_point3.values().1,
                    acc_point4.values().1,
                    acc_point5.values().1,
                    acc_point1.values().2,
                    acc_point2.values().2,
                    acc_point3.values().2,
                    acc_point4.values().2,
                    acc_point5.values().2,
                );

                let function_egui = (0..4).map(|i| {
                    let x = i as f64 * 0.01;
                    Value::new(x, function)
                });
                let line = Line::new(Values::from_values_iter(function_egui));
                */
                ui.add(
                    Plot::new("my_plot")
                        .points(acc_point1)
                        .points(acc_point2)
                        .points(acc_point3)
                        .points(acc_point4)
                        .points(acc_point5)
                        .hline(HLine::new(0.))
                        .vline(VLine::new(0.))
                        .allow_drag(false)
                        .allow_zoom(true)
                        .show_background(true)
                        .height(200.)
                        .width(600.),
                );
                // .line(line)
            });
            ui.group(|ui| {
                //Jerk graph
                let jerk_point1 = Points::new(Values::from_values(vec![Value::new(0., 0.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let jerk_point2 = Points::new(Values::from_values(vec![Value::new(1., 2.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let jerk_point3 = Points::new(Values::from_values(vec![Value::new(2., 0.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let jerk_point4 = Points::new(Values::from_values(vec![Value::new(3., -2.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                let jerk_point5 = Points::new(Values::from_values(vec![Value::new(4., 0.)]))
                    .filled(true)
                    .stems(0f32)
                    .radius(5.);
                /*
                let function = jerk(
                    jerk_point1.values().1,
                    jerk_point2.values().1,
                    jerk_point3.values().1,
                    jerk_point4.values().1,
                    jerk_point5.values().1,
                    jerk_point1.values().2,
                    jerk_point2.values().2,
                    jerk_point3.values().2,
                    jerk_point4.values().2,
                    jerk_point5.values().2,
                );

                let function_egui = (0..4).map(|i| {
                    let x = i as f64 * 0.01;
                    Value::new(x, function)
                });
                let line = Line::new(Values::from_values_iter(function_egui));
                */
                ui.add(
                    Plot::new("my_plot")
                        .points(jerk_point1)
                        .points(jerk_point2)
                        .points(jerk_point3)
                        .points(jerk_point4)
                        .points(jerk_point5)
                        .hline(HLine::new(0.))
                        .vline(VLine::new(0.))
                        .allow_drag(false)
                        .allow_zoom(true)
                        .show_background(true)
                        .height(200.)
                        .width(600.),
                );
                // .line(line)
            });
            //-----------------------------------------------------------------------------------------------------------------------------------------------
        });
        //-----------------------------------------------------------------------------------------------------------------------------------------------
    }
}
