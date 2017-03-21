extern crate sdl2;
extern crate serde;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
#[macro_use]
extern crate enum_primitive;
extern crate num;

pub extern crate ncollide;
pub extern crate nphysics2d;
pub extern crate nalgebra;

mod app;
pub use app::App;

mod audio;
pub use audio::Audio;

mod game_object;
pub use game_object::GameObject;

#[macro_use]
pub mod component;

mod graphics_private;
pub mod graphics {
    pub use graphics_private::texture::Texture;
    pub use graphics_private::Sprite;
    pub use graphics_private::sprite_sheet::SpriteSheet;
    pub use graphics_private::canvas::Canvas;
}

mod math_private;
pub mod math {
    pub use math_private::vector::Vector;
    pub use math_private::transform::Transform;
    pub use math_private::polar_coords::PolarCoords;
    pub use math_private::Rect;
    pub use math_private::IntRect;
    pub use math_private::IntVector;
}

mod input_private;
pub mod input {
    pub use input_private::Axis;
    pub use input_private::input::Input;
    pub use input_private::keyboard;
    pub use input_private::mouse;
    pub use input_private::Button;
}

mod camera;
pub use camera::Camera;

mod option_loaned;
pub use option_loaned::OptionLoaned;