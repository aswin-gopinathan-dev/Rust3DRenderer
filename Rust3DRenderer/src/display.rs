use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::FullscreenType;
use sdl2::video::Window;

pub struct AppWindow {
    pub event_pump: sdl2::EventPump,
    pub canvas: Canvas<Window>,
    pub is_running: bool,
    pub color_buffer: Vec<u32>,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl AppWindow {
    pub fn new(sdl_context: &Sdl, video_subsystem: &sdl2::VideoSubsystem) -> Result<Self, String> {
        let display_mode = video_subsystem
            .current_display_mode(0)
            .map_err(|e| e.to_string())?;

        let screen_width = display_mode.w as u32;
        let screen_height = display_mode.h as u32;

        let mut window = video_subsystem
            .window("3D Renderer", screen_width, screen_height)
            .position_centered()
            .build()
            .map_err(|e| format!("error while initializing window. {e}"))?;

        window
            .set_fullscreen(FullscreenType::Desktop)
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| format!("error while initializing canvas. {e}"))?;

        Ok(Self {
            event_pump,
            canvas,
            is_running: true,
            color_buffer: vec![0; (screen_width * screen_height) as usize],
            screen_width,
            screen_height,
        })
    }

    pub fn render(&mut self) -> Result<(), String> {
        self.render_color_buffer()?;
        self.clear_color_buffer(0xFF000000);        

        self.canvas.present();

        Ok(())
    }

    pub fn render_color_buffer(&mut self) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::ARGB8888,
                self.screen_width,
                self.screen_height,
            )
            .map_err(|e| e.to_string())?;

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..self.screen_height as usize {
                    for x in 0..self.screen_width as usize {
                        let pixel = self.color_buffer[y * self.screen_width as usize + x];
                        let offset = y * pitch + x * 4;
                        buffer[offset..offset + 4].copy_from_slice(&pixel.to_ne_bytes());
                    }
                }
            })
            .map_err(|e| e.to_string())?;

        self.canvas
            .copy(&texture, None, None)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn clear_color_buffer(&mut self, color: u32) {
        self.color_buffer.fill(color);
    }

    pub fn draw_grid(&mut self, color: u32) {
        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                if y % 50 == 0 || x % 50 == 0 {
                    let index = (self.screen_width * y + x) as usize;
                    self.color_buffer[index] = color;
                }
            }
        }
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.screen_width as i32 && 
           y >= 0 && y < self.screen_height as i32 {
            let index = self.screen_width as usize * y as usize + x as usize;
            self.color_buffer[index] = color;
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        for i in 0..width {
            for j in 0..height {
                let cur_x = x + i;
                let cur_y = y + j;
                self.draw_pixel(cur_x, cur_y, color);
            }
        }
    }
}
