extern crate sdl2;

use im::HashMap;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::ops::Deref;

use entity::Entity;
use component::Sprite;
use component::Transform;
use component::ComponentManager;

pub struct Render {
    canvas: Canvas<Window>
}

impl Render {
    pub fn new(sdl_ctx: &sdl2::Sdl, screen_width: u32, screen_height: u32) -> Self {
        let video_subsystem = sdl_ctx.video().unwrap();
        let window = video_subsystem
            .window("game window", screen_width, screen_height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().accelerated().build().unwrap();

        clear_canvas(&mut canvas);

        Render{
            canvas: canvas
        }
    }

    pub fn render(&mut self, ticks: u32, sprites: &ComponentManager<Sprite>, transforms: &ComponentManager<Transform>) {
        clear_canvas(&mut self.canvas);

        for arc_entity in sprites.keys() {
            let entity = arc_entity.deref().clone();
            match (sprites.get(&entity), transforms.get(&entity)) {
                (Some(s), Some(t)) => {
                    println!("Rendering: {} at {}x{}", entity.id, t.x, t.y);
                    let rect = Rect::new(t.x as i32, t.y as i32, s.width, s.height);
                    self.canvas.set_draw_color(s.color);
                    self.canvas.draw_rect(rect).unwrap();
                },
                (_, _) => ()
            };

        }

        self.canvas.present();
    }
}

fn clear_canvas(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}
