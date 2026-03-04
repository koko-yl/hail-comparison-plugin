use hail_pdk::{ConfigInput, HailPlugin, PluginFFI, core::types::Comparison, sdl2::{image::LoadTexture, pixels::Color, rect::Rect, render::{ScaleMode, Texture, TextureQuery}}, utils::{ConfigFont, PdkUtilErr, TextRenderer}};
use serde::Deserialize;

#[derive(PluginFFI)]
struct SegmentPlugin {
    h: u32,
    textures: [Result<Texture, PdkUtilErr>; 5],
    images: [Option<Texture>; 5]
}


#[derive(Deserialize)]
struct SConfig {
    pub font: ConfigFont,
    pub text_color: (u8, u8, u8, u8)
}

impl HailPlugin for SegmentPlugin {
    fn init(
        _: &hail_pdk::core::HailState,
        cfg: ConfigInput<SConfig>,
        c: &hail_pdk::sdl2::render::WindowCanvas,
        ssrc: &hail_pdk::font_kit::source::SystemSource,
    ) -> Result<Self, String> {
        println!("segment plugin init");
        let cfg = match cfg {
            ConfigInput::Config(c) => c,
            ConfigInput::Error(e) => panic!("font error: {}", e),
        };
        let font = cfg.font;
        let color: Color = cfg.text_color.into();
        let creator = c.texture_creator();
        let mut renderer = TextRenderer::new(ssrc, font).unwrap();
        let textures = [
            renderer.render_text_line(" Comparison: ", color, &creator),
            renderer.render_text_line(" Personal Best", color, &creator),
            renderer.render_text_line(" Golds", color, &creator),
            renderer.render_text_line(" Average", color, &creator),
            renderer.render_text_line("None", color, &creator)
        ];
        let images = [
            None,
            creator.load_texture_bytes(include_bytes!("../assets/pb.png")).ok(),
            creator.load_texture_bytes(include_bytes!("../assets/gold.png")).ok(),
            creator.load_texture_bytes(include_bytes!("../assets/avg.png")).ok(),
            None
        ];
        
        Ok(Self { textures, h: renderer.line_height, images })
    }
    
    fn update(&mut self, _: &hail_pdk::core::HailState, _: &hail_pdk::InputState) {}
    
    fn min_dims(&self) -> Option<(u32, u32)> {
        Some((0, 0))
    }
    
    fn max_height(&self) -> u32 {
        self.h
    }

    fn draw(&mut self, s: &hail_pdk::core::HailState, c: &mut hail_pdk::sdl2::render::WindowCanvas) {
        c.set_draw_color((0, 0, 0, 0));
        c.clear();
        let n = match s.comparison {
            Comparison::PersonalBest => 1,
            Comparison::Golds => 2,
            Comparison::Average => 3,
            Comparison::None => 4,
        };
        let tex = &self.textures[n];
        let vp = c.viewport();
        let mut xpos = 8;
        
        let head_tex = self.textures[0].as_ref().unwrap();
        let TextureQuery { width, height, .. } = head_tex.query();
        let ypos = (vp.height() as i32 - height as i32) / 2;
        
        
        c.copy(
            &head_tex,
            None,
            Rect::new(xpos, ypos, width, height),
        )
        .unwrap();
        xpos += width as i32;
        if let Some(img) = self.images[n].as_mut() {
            img.set_scale_mode(ScaleMode::Best);
            let d = (height as f32 * 0.8) as u32;
            let pad = (height - d) / 2;
            c.copy(
                img,
                None,
                Rect::new(
                    xpos,
                    ypos, 
                    d, 
                    d
                )
            ).unwrap();
    
            xpos += (height - pad - pad) as i32;
        }

        let TextureQuery { width, height, .. } = tex.as_ref().unwrap().query();
        c.copy(
            tex.as_ref().unwrap(),
            None,
            Rect::new(xpos, ypos, width, height),
        )
        .unwrap();
    }
    
    type Config = SConfig;
}