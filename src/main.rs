mod app;
use app::*;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
