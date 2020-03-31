use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

#[rustfmt::skip]
const WORLD_MAP: [[i32; MAP_WIDTH]; MAP_HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video().map_err(|e| format!("{:?}", e))?;
    let mut timer_subsystem = sdl_context.timer().map_err(|e| format!("{:?}", e))?;
    let window = video_subsystem
        .window("Raycaster", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .allow_highdpi()
        .build()
        .map_err(|e| format!("{:?}", e))?;
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| format!("{:?}", e))?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT)
        .map_err(|e| format!("{:?}", e))?;
    let mut texture_buffer;

    let mut pos_x = 22.0;
    let mut pos_y = 12.0;
    let mut dir_x = -1.0;
    let mut dir_y = 0.0;
    let mut plane_x = 0.0;
    let mut plane_y = 0.66;

    let mut time = 0.0;
    let mut old_time;

    let mut done = false;
    let mut event_pump = sdl_context.event_pump()?;
    while !done {
        texture_buffer = vec![0; (SCREEN_WIDTH * SCREEN_HEIGHT * 3) as usize];
        for x in 0..SCREEN_WIDTH {
            let camera_x = 2.0 * x as f64 / SCREEN_WIDTH as f64 - 1.0;
            let ray_dir_x = dir_x + plane_x * camera_x;
            let ray_dir_y = dir_y + plane_y * camera_x;

            let mut map_x = pos_x as usize;
            let mut map_y = pos_y as usize;

            let delta_dist_x = if ray_dir_y == 0.0 {
                0.0
            } else if ray_dir_x == 0.0 {
                1.0
            } else {
                (1.0 / ray_dir_x).abs()
            };
            let delta_dist_y = if ray_dir_x == 0.0 {
                0.0
            } else if ray_dir_y == 0.0 {
                1.0
            } else {
                (1.0 / ray_dir_y).abs()
            };

            let step_x;
            let mut side_dist_x = if ray_dir_x < 0.0 {
                step_x = -1;
                (pos_x - map_x as f64) * delta_dist_x
            } else {
                step_x = 1;
                (map_x as f64 + 1.0 - pos_x) * delta_dist_x
            };

            let step_y;
            let mut side_dist_y = if ray_dir_y < 0.0 {
                step_y = -1;
                (pos_y - map_y as f64) * delta_dist_y
            } else {
                step_y = 1;
                (map_y as f64 + 1.0 - pos_y) * delta_dist_y
            };

            let mut side = 0;
            let mut hit = 0;
            while hit == 0 {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x = (step_x + map_x as i32) as usize;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y = (step_y + map_y as i32) as usize;
                    side = 1;
                }

                if WORLD_MAP[map_x][map_y] > 0 {
                    hit = 1;
                }
            }

            let perp_wall_dist = if side == 0 {
                (map_x as f64 - pos_x + ((1 - step_x) / 2) as f64) / ray_dir_x
            } else {
                (map_y as f64 - pos_y + ((1 - step_y) / 2) as f64) / ray_dir_y
            };
            let line_height = (SCREEN_HEIGHT as f64 / perp_wall_dist) as i32;
            let mut draw_start = -line_height / 2 + SCREEN_HEIGHT as i32 / 2;
            if draw_start < 0 {
                draw_start = 0;
            }
            let mut draw_end = line_height / 2 + SCREEN_HEIGHT as i32 / 2;
            if draw_end >= SCREEN_HEIGHT as i32 {
                draw_end = SCREEN_HEIGHT as i32 - 1;
            }

            let mut color = match WORLD_MAP[map_x][map_y] {
                1 => [0xFF, 0, 0],
                2 => [0, 0xFF, 0],
                3 => [0, 0, 0xFF],
                4 => [0xFF, 0xFF, 0xFF],
                _ => [0xFF, 0xFF, 0],
            };

            if side == 1 {
                for i in color.iter_mut() {
                    *i /= 2;
                }
            }

            vert_line(
                x as usize,
                draw_start as usize,
                draw_end as usize,
                &mut texture_buffer,
                color,
            );
        }
        old_time = time;
        time = timer_subsystem.ticks() as f64;
        let frame_time = (time - old_time) / 1000.0;
        texture.with_lock(None, |buffer, _| buffer.clone_from_slice(&texture_buffer))?;
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        let move_speed = frame_time * 5.0;
        let rot_speed = frame_time * 3.0;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => done = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => done = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if WORLD_MAP[(pos_x + dir_x * move_speed) as usize][pos_y as usize] == 0 {
                        pos_x += dir_x * move_speed;
                    }
                    if WORLD_MAP[pos_x as usize][(pos_y + dir_y * move_speed) as usize] == 0 {
                        pos_y += dir_y * move_speed;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if WORLD_MAP[(pos_x - dir_x * move_speed) as usize][pos_y as usize] == 0 {
                        pos_x -= dir_x * move_speed;
                    }
                    if WORLD_MAP[pos_x as usize][(pos_y - dir_y * move_speed) as usize] == 0 {
                        pos_y -= dir_y * move_speed;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let old_dir_x = dir_x;
                    dir_x = dir_x * (-rot_speed).cos() - dir_y * (-rot_speed).sin();
                    dir_y = old_dir_x * (-rot_speed).sin() + dir_y * (-rot_speed).cos();
                    let old_plane_x = plane_x;
                    plane_x = plane_x * (-rot_speed).cos() - plane_y * (-rot_speed).sin();
                    plane_y = old_plane_x * (-rot_speed).sin() + plane_y * (-rot_speed).cos();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let old_dir_x = dir_x;
                    dir_x = dir_x * rot_speed.cos() - dir_y * rot_speed.sin();
                    dir_y = old_dir_x * rot_speed.sin() + dir_y * rot_speed.cos();
                    let old_plane_x = plane_x;
                    plane_x = plane_x * rot_speed.cos() - plane_y * rot_speed.sin();
                    plane_y = old_plane_x * rot_speed.sin() + plane_y * rot_speed.cos();
                }
                _ => (),
            }
        }
        timer_subsystem.delay(5);
    }

    Ok(())
}

fn vert_line(
    x: usize,
    line_start: usize,
    line_end: usize,
    texture_buffer: &mut [u8],
    color: [u8; 3],
) {
    for i in line_start..line_end {
        for (j, component) in color.iter().enumerate() {
            texture_buffer[(x + i * SCREEN_WIDTH as usize) * 3 + j] = *component;
        }
    }
}
