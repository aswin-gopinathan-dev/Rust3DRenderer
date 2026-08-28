use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod display;
mod vector;

use display::AppWindow;
use vector::{Vec2, Vec3, Axis};

const N_POINTS: usize = 9 * 9 * 9;

struct _3dModel {
    cube_points: [Vec3; N_POINTS],
    projected_points: [Vec2; N_POINTS],
}

impl Default for _3dModel {
    fn default() -> Self {
        Self {
            cube_points: [Vec3::default(); N_POINTS],
            projected_points: [Vec2::default(); N_POINTS],
        }
    }
}

struct RenderParams {
    fov_factor: f32,
    camera_position: Vec3,
    cube_rotation: Vec3,
}

fn setup(model: &mut _3dModel) {
    let mut x: f32 = -1.0;
    let mut y: f32;
    let mut z: f32;

    let mut point_count: usize = 0;
    while x <= 1.0 {
        y = -1.0;
        while y <= 1.0 {
            z = -1.0;
            while z <= 1.0 {
                let point = Vec3 { x, y, z };
                model.cube_points[point_count] = point;
                point_count += 1;
                z += 0.25;
            }
            y += 0.25;
        }
        x += 0.25;
    }
}

fn project(point: Vec3, render_params: &mut RenderParams) -> Vec2 {
    let projected_point = Vec2{ x: point.x *  render_params.fov_factor / point.z,
                                y: point.y *  render_params.fov_factor / point.z};

    projected_point
}

fn update(model: &mut _3dModel, render_params: &mut RenderParams) {
    render_params.cube_rotation.y += 0.01;
    render_params.cube_rotation.z += 0.01;
    render_params.cube_rotation.x += 0.01;
    for i in 0..N_POINTS {
        let point = model.cube_points[i];
        let mut transformed_point = point;
        transformed_point.rotate(Axis::XAxis, render_params.cube_rotation.x);
        transformed_point.rotate(Axis::YAxis, render_params.cube_rotation.y);
        transformed_point.rotate(Axis::ZAxis, render_params.cube_rotation.z);
        transformed_point.z -= render_params.camera_position.z;
        let projected_point = project(transformed_point, render_params);
        model.projected_points[i] = projected_point;
    }
}

fn render(wnd: &mut AppWindow, model: &_3dModel) -> Result<(), String> {
    for i in 0..N_POINTS {
        let projected_point: Vec2 = model.projected_points[i];
        let screen_x = (projected_point.x + wnd.screen_width as f32 / 2.0) as i32;
        let screen_y = (projected_point.y + wnd.screen_height as f32 / 2.0) as i32;
        wnd.draw_rect(
                screen_x,
                screen_y,
                2, 2, 0xFF4444FF);
    }
    wnd.render()?;

    Ok(())
}

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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut app_window = AppWindow::new(&sdl_context, &video_subsystem)?;

    let mut model = _3dModel::default();
    setup(&mut model);

    let mut render_params = RenderParams { fov_factor: 840.0, 
                                          camera_position : Vec3 { x: 0.0, y: 0.0, z: -5.0 },
                                          cube_rotation : Vec3 { x: 0.0, y:0.0, z:0.0 } };


    while app_window.is_running {
        process_input(&mut app_window);
        update(&mut model, &mut render_params);
        render(&mut app_window, &model)?;
    }

    Ok(())
}
