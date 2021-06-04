mod app;
mod game;
mod gfx;
mod shaders;
mod world;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
