use egui::plot::{Line, Value, Values};
use ndarray::{ArrayBase, Dim, OwnedRepr};
use ndarray_glm::{array, error::RegressionError, utility::standardize, Linear, ModelBuilder};

pub fn acceleration(
    point1_x: f64,
    point2_x: f64,
    point3_x: f64,
    point4_x: f64,
    point5_x: f64,
    point1_y: f64,
    point2_y: f64,
    point3_y: f64,
    point4_y: f64,
    point5_y: f64,
) -> Result<ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>, RegressionError> {
    // define some test data
    let data_y = array![point1_y, point2_y, point3_y, point4_y, point5_y];
    let data_x = array![
        [
            point1_x.powf(4.),
            point1_x.powf(3.),
            point1_x.powf(2.),
            point1_x
        ],
        [
            point2_x.powf(4.),
            point2_x.powf(3.),
            point2_x.powf(2.),
            point2_x
        ],
        [
            point3_x.powf(4.),
            point3_x.powf(3.),
            point3_x.powf(2.),
            point3_x
        ],
        [
            point4_x.powf(4.),
            point4_x.powf(3.),
            point4_x.powf(2.),
            point4_x
        ],
        [
            point5_x.powf(4.),
            point5_x.powf(3.),
            point5_x.powf(2.),
            point5_x
        ]
    ];
    // The design matrix can optionally be standardized, where the mean of each independent
    // variable is subtracted and each is then divided by the standard deviation of that variable.
    let data_x = standardize(data_x);
    let model = ModelBuilder::<Linear>::data(&data_y, &data_x)
        .build()
        .unwrap();
    // L2 (ridge) regularization can be applied with l2_reg().
    let fit = model.fit_options().l2_reg(1e-5).fit()?;
    // Currently the result is a simple array of the MLE estimators, including the intercept term.
    let result = fit.result;
    Ok(result)
    //return fit;
}
/*
pub fn velocity(
    x_cord: (f64, f64, f64, f64, f64),
    y_cord: (f64, f64, f64, f64, f64),
    acceleration: (f64, f64, f64, f64, f64),
    v0: f64,
) -> egui::plot::Line {
    let function = ((x_cord.0)..(x_cord.4)).map(|i| {
        let x = i as f64 * 0.01;
        Value::new(
            x,
            (((x_cord.0 * x.powf(5.)) / 5.)
                + ((x_cord.1 * x.powf(4.)) / 4.)
                + ((x_cord.2 * x.powf(3.)) / 3.)
                + ((x_cord.3 * x.powf(2.)) / 2.)
                + ((x_cord.4 * x)
                + v0),
        )
    });
    let line = Line::new(Values::from_values_iter(function));
    return line;
}
*/
/*
pub fn jerk(
    x_cord: (f64, f64, f64, f64, f64),
    y_cord: (f64, f64, f64, f64, f64),
    acceleration: (f64, f64, f64, f64, f64),
    v0: f64,
) -> egui::plot::Line {
    let function = ((x_cord.0)..(x_cord.4)).map(|i| {
        let x = i as f64 * 0.01;
        Value::new(
            x,
            (((x_cord.0 * x.powf(5.)) / 5.)
                + ((x_cord.1 * x.powf(4.)) / 4.)
                + ((x_cord.2 * x.powf(3.)) / 3.)
                + ((x_cord.3 * x.powf(2.)) / 2.)
                + ((x_cord.4 * x)
                + v0),
        )
    });
    let line = Line::new(Values::from_values_iter(function));
    return line;
}
*/