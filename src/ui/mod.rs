use egui::TextureHandle;
use epaint::image::{ColorImage, ImageData};
use image::{DynamicImage, ImageError};
use std::sync::Arc;
use super::res::{self, AssetExt};

mod app;

pub fn init_egui(canvas_id: &str) {

    let app = app::TemplateApp::default();
    let _result = egui_web::start(canvas_id, Box::new(app));
}

//读取png
pub fn load_png(ctx:&egui::Context, name: &str) -> Result<TextureHandle, ImageError> {
   
    let bg_img = res::Image::load_expect(name).read().to_image();
    let result = load_image_from_memory(&bg_img);

    let color_image = match result {
        Ok(image) => image,
        Err(err)=>{
            log::error!("load png error:{:?}", err);
            return Err(err);
        }
    };

    let data = ImageData::Color(color_image);
    let texture_handle = ctx.load_texture(name, data);
    Ok(texture_handle)
}

//根据ui容器大小自动计算texture size
pub fn auto_texture_size(ui:&egui::Ui, texture:&egui::TextureHandle) -> egui::Vec2 {
    let ui_size = ui.available_size();
    let tex_size = texture.size_vec2();
    let texture_radio = tex_size.x / tex_size.y;
    let canvas_radio = ui_size.x / ui_size.y;

     //画布比tex宽,
    if canvas_radio > texture_radio {
                
        egui::vec2(ui_size.x, ui_size.x / texture_radio)
    //画布比tex窄, 以高做标准
    } else{
        egui::vec2(ui_size.y * texture_radio , ui_size.y)
    }
}


fn load_image_from_memory(image: &Arc<DynamicImage>) -> Result<ColorImage, ImageError> {
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
