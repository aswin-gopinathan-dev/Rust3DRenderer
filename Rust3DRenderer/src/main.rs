
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod display;

use display::AppWindow;



fn process_input(wnd: &mut AppWindow) {
    for event in wnd.event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                println!("Quit event");
                wnd.is_running = false;
            }

            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                wnd.is_running = false;
            }

            _ => {}
        } 
    }
}

fn setup() {
}

fn update() {
}


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut app_window = AppWindow::new(&sdl_context, &video_subsystem)?;

    setup();
    while app_window.is_running {
        process_input(&mut app_window);
        update();
        app_window.render()?;
    }

    Ok(())
}
