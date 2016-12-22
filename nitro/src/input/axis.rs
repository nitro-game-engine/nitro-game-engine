use app::App;
use input::Button;

pub struct Axis {
    pos: Button,
    neg: Button,
}

impl Axis {
    pub fn new(pos: Button, neg: Button) -> Axis {
        Axis {
            pos: pos,
            neg: neg,
        }
    }

    pub fn get_value(&self, app: &App) -> f64 {
        let pos_bool = app.is_button_down(self.pos);
        if pos_bool == app.is_button_down(self.neg) {
            return 0.0;
        }
        if pos_bool {
            return 1.0;
        }
        -1.0 // They aren't equal and positive is false so negative is true.
    }
}