use wasm_bindgen::prelude::*;
mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


use winit::{
    event::{Event, WindowEvent},
    window::{WindowBuilder, Window},
    platform::web::WindowExtWebSys
};

//canvas_id 来自html的canvas
#[wasm_bindgen]
pub fn start(){
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    log::info!("start web");
    //试验结果，winit可以用，无报错
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    };

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Wasm_test")
        .build(&event_loop)
        .unwrap();


    //创建画布
    create_render_area(&window, "canvas_main");

    //event loop
    event_loop.run(move |win_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        //各种event情况下干什么
        match win_event {

            Event::WindowEvent {
                event,
                window_id,

            } => match event {

                WindowEvent::CloseRequested => {
                    log::info!("indowEvent::CloseRequested");

                    if window_id == window.id() {
                        *control_flow = ControlFlow::Exit;
                    } 
                },

                // WindowEvent::ScaleFactorChanged{scale_factor, new_inner_size} => {
                //     log::info!("window ScaleFactorChanged: {:?}, {:?}", scale_factor, new_inner_size);
                // },

                //default
                _ => {
                    //log::info!("window event:{:?}", event);
                }
            }
        

            Event::MainEventsCleared => {
                window.request_redraw();

                log::info!("Event::MainEventsCleared");
            },
            _ => (),
        }
    });
}

pub fn create_render_area(window: &Window, canvas_id: &str)  {

    //设置画布大小,样式等
    let canvas = window.canvas();
    canvas.style().set_css_text("background-color: gray; ");
    canvas.set_id(canvas_id);

    let web_window = web_sys::window().unwrap();
    let document = web_window.document().unwrap();
    let body = document.body().unwrap();

    //html放入画布
    body.append_child(&canvas).unwrap();

    //初始化ui
    let app = app::TemplateApp::default();
    init_egui(canvas_id, Box::new(app));
}

pub fn init_egui(canvas_id: &str, app: Box<dyn epi::App>) {
    egui_web::start(canvas_id, app);
}

