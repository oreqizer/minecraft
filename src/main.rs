mod app;
mod game;
mod gfx;

use app::App;

fn main() {
    let app = App::new();
    app.run();
}
