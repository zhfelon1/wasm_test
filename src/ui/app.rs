/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state

use std::collections::HashMap;
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,

    textures : HashMap<String, egui::TextureHandle>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            textures : HashMap::new(),
        }
    }
}

impl epi::App for TemplateApp {

    fn name(&self) -> &str {
        "Main-UI"
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::new(4096.0, 4096.0)
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
        _gl: &std::rc::Rc<glow::Context>
    ) {
       
        let mut cache_texture = |name:&str| {

            let key = name.to_string();
            let texture = crate::ui::load_png(_ctx, &key).unwrap();
            self.textures.insert(key, texture);
        };

        //cache texture
        cache_texture("voxygen.background.bg_main");
        cache_texture("voxygen.element.ui.generic.buttons.button");
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {

      
        //background layer
        egui::Area::new("Background")
            .order(egui::Order::Background)
            .movable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
            let texture = self.textures.get("voxygen.background.bg_main").unwrap();

            //背景自适应
            let tex_size = super::auto_texture_size(&ui, &texture);
            let background = egui::Image::new(texture.id(), tex_size);
            ui.add( background);
           
        });


        //UI Layer M
        egui::Area::new("Middle")
            .order(egui::Order::Middle)
            .fixed_pos(egui::pos2(0.0, 0.0))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {



            let texture = self.textures.get("voxygen.element.ui.generic.buttons.button").unwrap();

            if ui.add(egui::ButtonPng::new(texture.id(), texture.size_vec2(),"Bgm Test")).clicked() {
                log::info!("ButtonPng Test");
            }

            if ui.add(egui::ButtonPng::new(texture.id(), texture.size_vec2(),"Gpu Test")).clicked() {
                log::info!("Gpu Test");
            }
        });

    }
}
