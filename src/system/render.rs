extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::ops::Deref;

use component::Sprite;
use component::Transform;
use component::ComponentManager;

pub struct Render {
    canvas: Canvas<Window>,
    screen_width: u32,
    screen_height: u32
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
            canvas,
            screen_width,
            screen_height
        }
    }

    pub fn render(&mut self, ticks: u32, sprites: &ComponentManager<Sprite>, transforms: &ComponentManager<Transform>) {
        let mut scene_left = 0.0;
        let mut scene_right = 0.0;
        let mut scene_top = 0.0;
        let mut scene_bottom = 0.0;

        for arc_entity in sprites.keys() {
            let entity = arc_entity.deref().clone();
            match transforms.get(&entity) {
                Some(t) => {
                    if t.x < scene_left {
                        scene_left = t.x;
                    } else if t.x > scene_right {
                        scene_right = t.x;
                    }
                    if t.y < scene_top {
                        scene_top = t.y;
                    } else if t.y > scene_bottom {
                        scene_bottom = t.y;
                    }
                },
                None => ()
            };
        }

        let scene_width = (scene_right - scene_left) + 1.0;
        let scene_height = (scene_bottom - scene_top) + 1.0;
        let min = |a: f32, b: f32| if a < b { a } else { b };
        let max = |a: f32, b: f32| if a > b { a } else { b };
        let scene_scale: f32 = min(self.screen_width as f32 / scene_width, self.screen_height as f32 / scene_height);

        println!("Scene scale: {}; left: {}; right: {}; top: {}; bottom: {}", scene_scale, scene_left, scene_right, scene_top, scene_bottom);

        clear_canvas(&mut self.canvas);

        for arc_entity in sprites.keys() {
            let entity = arc_entity.deref().clone();
            match (sprites.get(&entity), transforms.get(&entity)) {
                (Some(s), Some(t)) => {
                    let left: i32 = (t.x * scene_scale) as i32;
                    let width: i32 = max(scene_scale, 1.0) as i32;
                    let top: i32 = (t.y * scene_scale) as i32;
                    let height: i32 = max(scene_scale, 1.0) as i32;
                    println!("Rendering: {} at {}x{}", entity.id, left, top);
                    let rect = Rect::new(left, top, width as u32, height as u32);
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
