// mod ex; //testing examples

mod camera;
mod engine;
mod glcontext;
mod gui;
mod input;
mod mesh;
mod scene_object;
mod shader;
mod textures;

fn main() {
    //ex::hello_triangle::run(); // ex1

    let mut engine = engine::Engine::new(1280, 720, "Rust OBJ Scene");
    engine.run();
}
