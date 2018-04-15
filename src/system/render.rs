extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::ops::Deref;
use std::iter::FromIterator;
use std::sync::Arc;
use std::cmp::Ordering;

use entity::Entity;
use component::{
    Sprite,
    Transform,
    Camera,
    ComponentManager
};

pub struct Render {
    canvas: Canvas<Window>,
    screen_width: u32,
    screen_height: u32,
    world_width: f64,
    world_height: f64
}

impl Render {
    pub fn new(sdl_ctx: &sdl2::Sdl, screen_width: u32, screen_height: u32, world_width: f64, world_height: f64) -> Self {
        let video_subsystem = sdl_ctx.video().unwrap();
        let window = video_subsystem
            .window("game window", screen_width, screen_height)
            .fullscreen_desktop()
            .build()
            .unwrap();

        let display_mode = window.display_mode().unwrap();

        let mut canvas = window.into_canvas().accelerated().build().unwrap();

        clear_canvas(&mut canvas);

        Render{
            canvas,
            screen_width: display_mode.w as u32,
            screen_height: display_mode.h as u32,
            world_width,
            world_height
        }
    }

    pub fn render(
        &mut self, _ticks: u32,
        cameras: &ComponentManager<Camera>,
        sprites: &ComponentManager<Sprite>,
        transforms: &ComponentManager<Transform>) {

        // For now, only one scene is rendered regarless of the number of cameras.
        // The scene is zoomed out to include all regions covered by any camera.
        // This means some regions may be visible even if they are not covered by any cameras.
        let mut scene_left = 0.0;
        let mut scene_right = 0.0;
        let mut scene_top = 0.0;
        let mut scene_bottom = 0.0;

        for arc_entity in cameras.keys() {
            let entity = arc_entity.deref().clone();
            match (cameras.get(&entity), transforms.get(&entity)) {
                (Some(c), Some(t)) => {
                    let camera_left = t.x - (c.view_width / 2.0);
                    let camera_right = t.x + (c.view_width / 2.0) + 1.0;
                    let camera_top = t.y - (c.view_height / 2.0);
                    let camera_bottom = t.y + (c.view_height / 2.0) + 1.0;

                    if scene_left == 0.0 || camera_left < scene_left {
                        scene_left = camera_left;
                    }
                    if scene_right == 0.0 || camera_right > scene_right {
                        scene_right = camera_right;
                    }
                    if scene_top == 0.0 || camera_top < scene_top {
                        scene_top = camera_top;
                    }
                    if scene_bottom == 0.0 || camera_bottom > scene_bottom {
                        scene_bottom = camera_bottom;
                    }
                },
                (_, _) => {}
            };
        }

        // Adjust scene to avoid rendering out of bounds
        if scene_left < 0.0 {
            let adj = 0.0 - scene_left;
            scene_left += adj;
            scene_right += adj;
        } else if scene_right > self.world_width {
            let adj = self.world_width - scene_right;
            scene_left += adj;
            scene_right += adj;
        }
        if scene_top < 0.0 {
            let adj = 0.0 - scene_top;
            scene_top += adj;
            scene_bottom += adj;
        } else if scene_bottom > self.world_height {
            let adj = self.world_height - scene_bottom;
            scene_top += adj;
            scene_bottom += adj;
        }

        let scene_width = scene_right - scene_left;
        let scene_height = scene_bottom - scene_top;
        let min = |a: f64, b: f64| if a < b { a } else { b };
        let max = |a: f64, b: f64| if a > b { a } else { b };
        let scene_scale: f64 = min(self.screen_width as f64 / scene_width, self.screen_height as f64 / scene_height);

        clear_canvas(&mut self.canvas);

        let mut draw_order: Vec<Arc<Entity>> = Vec::from_iter(sprites.keys());
        draw_order.sort_by(|a, b| {
            match (sprites.get(a), sprites.get(b)) {
                (Some(sa), Some(sb)) =>
                    if sa.z_index < sb.z_index { Ordering::Less }
                    else if sa.z_index > sb.z_index { Ordering::Greater }
                    else { Ordering::Equal },
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal
            }
        });
        for arc_entity in draw_order.iter() {
            let entity = arc_entity.deref().clone();
            match (sprites.get(&entity), transforms.get(&entity)) {
                (Some(s), Some(t)) => {
                    let left: i32 = ((t.x - scene_left) * scene_scale).floor() as i32;
                    let top: i32 = ((t.y - scene_top) * scene_scale).floor() as i32;
                    let width: i32 = max(scene_scale, 1.0).ceil() as i32;
                    let height: i32 = max(scene_scale, 1.0).ceil() as i32;
                    let rect = Rect::new(left, top, width as u32, height as u32);
                    self.canvas.set_draw_color(s.color);
                    if s.fill {
                        self.canvas.fill_rect(rect).unwrap();
                    } else {
                        self.canvas.draw_rect(rect).unwrap();
                    }
                },
                (_, _) => ()
            };

        }

        // Debug camera outline
        let rect = Rect::new(
            (0.0 * scene_scale) as i32,
            (0.0 * scene_scale) as i32,
            (scene_width * scene_scale) as u32,
            (scene_height * scene_scale) as u32);
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.draw_rect(rect).unwrap();

        self.canvas.present();
    }
}

fn clear_canvas(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}
