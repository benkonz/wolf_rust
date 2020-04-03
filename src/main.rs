use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::collections::HashSet;

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const TEXTURE_WIDTH: usize = 64;
const TEXTURE_HEIGHT: usize = 64;

#[rustfmt::skip]
const WORLD_MAP: [[i32; MAP_WIDTH]; MAP_HEIGHT] = [
    [8,8,8,8,8,8,8,8,8,8,8,4,4,6,4,4,6,4,6,4,4,4,6,4],
    [8,0,0,0,0,0,0,0,0,0,8,4,0,0,0,0,0,0,0,0,0,0,0,4],
    [8,0,3,3,0,0,0,0,0,8,8,4,0,0,0,0,0,0,0,0,0,0,0,6],
    [8,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6],
    [8,0,3,3,0,0,0,0,0,8,8,4,0,0,0,0,0,0,0,0,0,0,0,4],
    [8,0,0,0,0,0,0,0,0,0,8,4,0,0,0,0,0,6,6,6,0,6,4,6],
    [8,8,8,8,0,8,8,8,8,8,8,4,4,4,4,4,4,6,0,0,0,0,0,6],
    [7,7,7,7,0,7,7,7,7,0,8,0,8,0,8,0,8,4,0,4,0,6,0,6],
    [7,7,0,0,0,0,0,0,7,8,0,8,0,8,0,8,8,6,0,0,0,0,0,6],
    [7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,6,0,0,0,0,0,4],
    [7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,6,0,6,0,6,0,6],
    [7,7,0,0,0,0,0,0,7,8,0,8,0,8,0,8,8,6,4,6,0,6,6,6],
    [7,7,7,7,0,7,7,7,7,8,8,4,0,6,8,4,8,3,3,3,0,3,3,3],
    [2,2,2,2,0,2,2,2,2,4,6,4,0,0,6,0,6,3,0,0,0,0,0,3],
    [2,2,0,0,0,0,0,2,2,4,0,0,0,0,0,0,4,3,0,0,0,0,0,3],
    [2,0,0,0,0,0,0,0,2,4,0,0,0,0,0,0,4,3,0,0,0,0,0,3],
    [1,0,0,0,0,0,0,0,1,4,4,4,4,4,6,0,6,3,3,0,0,0,3,3],
    [2,0,0,0,0,0,0,0,2,2,2,1,2,2,2,6,6,0,0,5,0,5,0,5],
    [2,2,0,0,0,0,0,2,2,2,0,0,0,2,2,0,5,0,5,0,0,0,5,5],
    [2,0,0,0,0,0,0,0,2,0,0,0,0,0,2,5,0,5,0,5,0,5,0,5],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5],
    [2,0,0,0,0,0,0,0,2,0,0,0,0,0,2,5,0,5,0,5,0,5,0,5],
    [2,2,0,0,0,0,0,2,2,2,0,0,0,2,2,0,5,0,5,0,0,0,5,5],
    [2,2,2,2,1,2,2,2,2,2,2,1,2,2,2,5,5,5,5,5,5,5,5,5],
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

    let mut pressed_keys = HashSet::new();

    let eagle_texture_buf = include_bytes!("textures/eagle.png");
    let redbrick_texture_buf = include_bytes!("textures/redbrick.png");
    let purple_stone_texture_buf = include_bytes!("textures/purplestone.png");
    let grey_stone_texture_buf = include_bytes!("textures/greystone.png");
    let blue_stone_texture_buf = include_bytes!("textures/bluestone.png");
    let mossy_texture_buf = include_bytes!("textures/mossy.png");
    let wood_texture_buf = include_bytes!("textures/wood.png");
    let color_stone_texture_buf = include_bytes!("textures/colorstone.png");

    let eagle_texture = load_png(eagle_texture_buf.as_ref())?;
    let redbrick_texture = load_png(redbrick_texture_buf.as_ref())?;
    let purple_stone_texture = load_png(purple_stone_texture_buf.as_ref())?;
    let grey_stone_texture = load_png(grey_stone_texture_buf.as_ref())?;
    let blue_stone_texture = load_png(blue_stone_texture_buf.as_ref())?;
    let mossy_texture = load_png(mossy_texture_buf.as_ref())?;
    let wood_texture = load_png(wood_texture_buf.as_ref())?;
    let color_stone_texture = load_png(color_stone_texture_buf.as_ref())?;

    let mut textures = vec![vec![0; TEXTURE_WIDTH * TEXTURE_HEIGHT * 3]; 8];
    textures[0].clone_from_slice(&eagle_texture);
    textures[1].clone_from_slice(&redbrick_texture);
    textures[2].clone_from_slice(&purple_stone_texture);
    textures[3].clone_from_slice(&grey_stone_texture);
    textures[4].clone_from_slice(&blue_stone_texture);
    textures[5].clone_from_slice(&mossy_texture);
    textures[6].clone_from_slice(&wood_texture);
    textures[7].clone_from_slice(&color_stone_texture);
    let mut pos_x = 22.0;
    let mut pos_y = 11.5;
    let mut dir_x = -1.0;
    let mut dir_y = 0.0;
    let mut plane_x = 0.0;
    let mut plane_y = 0.66;
    let mut time = 0.0;
    let mut old_time;
    let mut texture_buffer;
    let mut done = false;
    let mut event_pump = sdl_context.event_pump()?;
    while !done {
        texture_buffer = vec![0; (SCREEN_WIDTH * SCREEN_HEIGHT * 3) as usize];

        // floor casting
        for y in 0..SCREEN_HEIGHT {
            let ray_dir_x0 = dir_x - plane_x;
            let ray_dir_y0 = dir_y - plane_y;
            let ray_dir_x1 = dir_x + plane_x;
            let ray_dir_y1 = dir_y + plane_y;

            let p = y as i32 - SCREEN_HEIGHT as i32 / 2;

            let pos_z = 0.5 * SCREEN_HEIGHT as f64;

            let row_distance = pos_z / p as f64;

            let floor_step_x = row_distance * (ray_dir_x1 - ray_dir_x0) / SCREEN_WIDTH as f64;
            let floor_step_y = row_distance * (ray_dir_y1 - ray_dir_y0) / SCREEN_WIDTH as f64;

            let mut floor_x = pos_x + row_distance * ray_dir_x0;
            let mut floor_y = pos_y + row_distance * ray_dir_y0;

            for x in 0..SCREEN_WIDTH {
                let cell_x = floor_x as usize;
                let cell_y = floor_y as usize;

                let tx = (TEXTURE_WIDTH as f64 * (floor_x - cell_x as f64)) as usize
                    & (TEXTURE_WIDTH - 1);
                let ty = (TEXTURE_HEIGHT as f64 * (floor_y - cell_y as f64)) as usize
                    & (TEXTURE_HEIGHT - 1);

                floor_x += floor_step_x;
                floor_y += floor_step_y;

                let floor_texture = 3;
                let ceiling_texture = 6;

                for i in 0..3 {
                    let mut color = textures[floor_texture][(TEXTURE_WIDTH * ty + tx) * 3 + i];
                    color /= 2;
                    texture_buffer[(x as usize + y as usize * SCREEN_WIDTH as usize) * 3 + i] =
                        color;

                    color = textures[ceiling_texture][(TEXTURE_WIDTH * ty + tx) * 3 + i];
                    color /= 2;
                    texture_buffer[(x as usize
                        + (SCREEN_HEIGHT - 1 - y) as usize * SCREEN_WIDTH as usize)
                        * 3
                        + i] = color;
                }
            }
        }

        // wall casting
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

            let tex_num = (WORLD_MAP[map_x][map_y] - 1) as usize;

            let mut wall_x = if side == 0 {
                pos_y + perp_wall_dist * ray_dir_y
            } else {
                pos_x + perp_wall_dist * ray_dir_x
            };
            wall_x -= wall_x.floor();

            let mut tex_x = (wall_x * TEXTURE_WIDTH as f64) as usize;
            if side == 0 && ray_dir_x > 0.0 {
                tex_x = TEXTURE_WIDTH as usize - tex_x - 1;
            }
            if side == 1 && ray_dir_y < 0.0 {
                tex_x = TEXTURE_WIDTH as usize - tex_x - 1;
            }

            let step = 1.0 * TEXTURE_HEIGHT as f64 / line_height as f64;
            let mut tex_pos =
                (draw_start - SCREEN_HEIGHT as i32 / 2 + line_height / 2) as f64 * step;
            for y in draw_start..draw_end {
                let tex_y = tex_pos as usize & (TEXTURE_HEIGHT as usize - 1);
                tex_pos += step;

                for i in 0..3 {
                    let mut color =
                        textures[tex_num][(TEXTURE_HEIGHT as usize * tex_y + tex_x) * 3 + i];
                    if side == 1 {
                        color /= 2;
                    }

                    texture_buffer[(x as usize + y as usize * SCREEN_WIDTH as usize) * 3 + i] =
                        color;
                }
            }
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
                    keycode: Some(keycode),
                    ..
                } => {
                    let _ = pressed_keys.insert(keycode);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    let _ = pressed_keys.remove(&keycode);
                }
                _ => (),
            }
        }

        if pressed_keys.contains(&Keycode::Escape) {
            done = true;
        }

        if pressed_keys.contains(&Keycode::Up) {
            if WORLD_MAP[(pos_x + dir_x * move_speed) as usize][pos_y as usize] == 0 {
                pos_x += dir_x * move_speed;
            }
            if WORLD_MAP[pos_x as usize][(pos_y + dir_y * move_speed) as usize] == 0 {
                pos_y += dir_y * move_speed;
            }
        }

        if pressed_keys.contains(&Keycode::Down) {
            if WORLD_MAP[(pos_x - dir_x * move_speed) as usize][pos_y as usize] == 0 {
                pos_x -= dir_x * move_speed;
            }
            if WORLD_MAP[pos_x as usize][(pos_y - dir_y * move_speed) as usize] == 0 {
                pos_y -= dir_y * move_speed;
            }
        }

        if pressed_keys.contains(&Keycode::Right) {
            let old_dir_x = dir_x;
            dir_x = dir_x * (-rot_speed).cos() - dir_y * (-rot_speed).sin();
            dir_y = old_dir_x * (-rot_speed).sin() + dir_y * (-rot_speed).cos();
            let old_plane_x = plane_x;
            plane_x = plane_x * (-rot_speed).cos() - plane_y * (-rot_speed).sin();
            plane_y = old_plane_x * (-rot_speed).sin() + plane_y * (-rot_speed).cos();
        }

        if pressed_keys.contains(&Keycode::Left) {
            let old_dir_x = dir_x;
            dir_x = dir_x * rot_speed.cos() - dir_y * rot_speed.sin();
            dir_y = old_dir_x * rot_speed.sin() + dir_y * rot_speed.cos();
            let old_plane_x = plane_x;
            plane_x = plane_x * rot_speed.cos() - plane_y * rot_speed.sin();
            plane_y = old_plane_x * rot_speed.sin() + plane_y * rot_speed.cos();
        }
        timer_subsystem.delay(5);
    }

    Ok(())
}

fn load_png(texture: impl std::io::Read) -> Result<Vec<u8>, String> {
    let decoder = png::Decoder::new(texture);
    let (info, mut reader) = decoder.read_info().map_err(|e| format!("{:?}", e))?;
    let mut buf = vec![0; info.buffer_size()];
    reader
        .next_frame(&mut buf)
        .map_err(|e| format!("{:?}", e))?;

    Ok(buf)
}
