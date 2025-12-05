// mod ex; //testing examples

mod app;
mod camera;
mod glcontext;
mod input;
mod mesh;
mod scene_object;
mod shader;

fn main() {
    //ex::hello_triangle::run(); // ex1

    let mut app = app::App::new(1280, 720, "Rust OBJ Scene");
    app.run();
}
