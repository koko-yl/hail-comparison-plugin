use hail_pdk::{ConfigInput, HailPlugin, PluginFFI, core::types::Comparison, sdl2::{pixels::Color, rect::Rect, render::{Texture, TextureQuery}}, utils::{ConfigFont, PdkUtilErr, TextRenderer}};
use serde::Deserialize;

#[derive(PluginFFI)]
struct SegmentPlugin {
    h: u32,
    textures: [Result<Texture, PdkUtilErr>; 4]
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
            renderer.render_text_line("Comparison: Personal Best", color, &creator),
            renderer.render_text_line("Comparison: Golds", color, &creator),
            renderer.render_text_line("Comparison: Average", color, &creator),
            renderer.render_text_line("Comparison: None", color, &creator)
        ];
        Ok(Self { textures, h: renderer.line_height })
    }
    
    fn update(&mut self, _: &hail_pdk::core::HailState, _: &hail_pdk::InputState) {}
    
    fn min_dims(&self) -> Option<(u32, u32)> {
        // println!("segment plugin min_dims");
        Some((0, 0))
    }
    
    fn max_height(&self) -> u32 {
        self.h
    }

    fn draw(&mut self, s: &hail_pdk::core::HailState, c: &mut hail_pdk::sdl2::render::WindowCanvas) {
        // println!("segment plugin draw");
        c.set_draw_color((0, 0, 0, 0));
        c.clear();
        let tex = match s.comparison {
            Comparison::PersonalBest => &self.textures[0],
            Comparison::Golds => &self.textures[1],
            Comparison::Average => &self.textures[2],
            Comparison::None => &self.textures[3],
        };
        // let tex = tex.unwrap();
        let vp = c.viewport();
        let TextureQuery { width, height, .. } = tex.as_ref().unwrap().query();
        let xpos = 8;//(vp.width() as i32 - width as i32) / 2;
        let ypos = (vp.height() as i32 - height as i32) / 2;
        c.copy(
            &tex.as_ref().unwrap(),
            None,
            Rect::new(xpos, ypos, width, height),
        )
        .unwrap();
        
        // hail_pdk::utils::TextRenderer
    }
    
    type Config = SConfig;
}