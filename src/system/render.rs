extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::ops::Deref;

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
    world_width: f32,
    world_height: f32
}

impl Render {
    pub fn new(sdl_ctx: &sdl2::Sdl, screen_width: u32, screen_height: u32, world_width: f32, world_height: f32) -> Self {
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
            screen_height,
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
                    println!("Camera: x: {}, y: {}, width: {}, height: {}.", t.x, t.y, c.view_width, c.view_height);
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

        let scene_width = (scene_right - scene_left);
        let scene_height = (scene_bottom - scene_top);
        let min = |a: f32, b: f32| if a < b { a } else { b };
        let max = |a: f32, b: f32| if a > b { a } else { b };
        let scene_scale: f32 = min(self.screen_width as f32 / scene_width, self.screen_height as f32 / scene_height);

        println!("Scene scale: {}; left: {}; right: {}; top: {}; bottom: {}", scene_scale, scene_left, scene_right, scene_top, scene_bottom);

        clear_canvas(&mut self.canvas);

        for arc_entity in sprites.keys() {
            let entity = arc_entity.deref().clone();
            match (sprites.get(&entity), transforms.get(&entity)) {
                (Some(s), Some(t)) => {
                    let left: i32 = round((t.x - scene_left) * scene_scale);
                    let top: i32 = round((t.y - scene_top) * scene_scale);
                    let width: i32 = round(max(scene_scale, 1.0));
                    let height: i32 = round(max(scene_scale, 1.0));
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

fn round(v: f32) -> i32 {
    if v % 1.0 < 0.5 {
        v.floor() as i32
    } else {
        v.ceil() as i32
    }
}
