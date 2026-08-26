use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::Sdl;



const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct AppWindow {
    event_pump: sdl2::EventPump,
    canvas: Canvas<Window>,
    is_running: bool
}

impl AppWindow {
    fn new(sdl_context: &Sdl, video_subsystem: &sdl2::VideoSubsystem) -> Result<Self, String> {
        let window = video_subsystem.window("3D Renderer", SCREEN_WIDTH, SCREEN_HEIGHT)
                                     .position_centered()
                                     .build()
                                     .map_err(|e| format!("error while initializing window. {e}"))?;

        let event_pump = sdl_context.event_pump()?;

        let canvas = window.into_canvas()
                            .build()
                            .map_err(|e| format!("error while initializing canvas. {e}"))?;

        Ok( Self {
                event_pump,
                canvas,
                is_running: true,
        })
    }

    fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                self.is_running = false;
            }
        }
    }

    fn setup() {
    }

    fn update() {
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 0));
        self.canvas.clear();
        self.canvas.present();
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut app_window = AppWindow::new(&sdl_context, &video_subsystem)?;

    AppWindow::setup();
    while app_window.is_running {
        app_window.process_input();
        AppWindow::update();
        app_window.render();
    }

    Ok(())
}
