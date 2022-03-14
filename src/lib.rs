use wasm_bindgen::prelude::*;
use winit::window;


//use rand::{thread_rng, Rng};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn start(){
    wasm_logger::init(wasm_logger::Config::default());

    // Logging
    log::info!("start");


    //试验结果，winit可以用，无报错
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::Window,
    };

    let win_builder = winit::window::WindowBuilder::new()
            .with_title("Veloren")
            .with_maximized(true);

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
}

