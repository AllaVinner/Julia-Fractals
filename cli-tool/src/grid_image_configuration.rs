
pub struct GridImageConfiguration {
    height: Pixels,
    width: Pixels,
    x_min: Number,
    x_max: Number,
    y_min: Number,
    y_max: Number,
}

impl Default for GridImageConfiguration {
    fn default() -> Self {
        Self { height: 800, width: 800, x_min: -3., x_max: 3., y_min: -3., y_max: 3.}
    }
}
