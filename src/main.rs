use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use rand::random;
use std::time::Duration;

mod test;

struct HSV {
    H: i32,
    S: f32,
    V: f32,
}

impl HSV {
    // this is totally broken
    fn to_rgb(&self) -> Color {
        let C = self.S * self.V;
        let H_ = self.H as f32 / 60.0;
        let X = C * (1 - (H_ as i32 % 2 - 1).abs()) as f32;
        let m = self.V - C;

        let rgb = |r: f32, g: f32, b: f32| -> Color {
            Color::RGB(
                ((r + m) * 255.0) as u8,
                ((g + m) * 255.0) as u8,
                ((b + m) * 255.0) as u8,
            )
        };

        return match self.H {
            0..=59 => rgb(C, X, 0.0),
            60..=119 => rgb(X, C, 0.0),
            120..=179 => rgb(0.0, C, X),
            180..=239 => rgb(0.0, X, C),
            240..=299 => rgb(C, 0.0, X),
            300..=359 => rgb(X, 0.0, C),
            _ => Color::RGB(0, 0, 0),
        };
    }
}

// is it possible to pass width and height as compile-time constants?
#[derive(Clone)]
struct Life {
    width: u32,
    height: u32,
    wrap_indexes: bool,
    data: Vec<bool>,
    next_value: fn(u32, u32, &Life) -> bool,
}

impl Life {
    fn draw_blinker(&mut self, x: u32, y: u32) {
        self.set(x + 1, y, true);
        self.set(x + 1, y + 1, true);
        self.set(x + 1, y + 2, true);
    }

    fn draw_glider(&mut self, x: u32, y: u32) {
        self.set(x + 1, y + 0, true);
        self.set(x + 2, y + 1, true);
        self.set(x + 0, y + 2, true);
        self.set(x + 1, y + 2, true);
        self.set(x + 2, y + 2, true);
    }

    fn randomize(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set(x, y, rand::random::<bool>());
            }
        }
    }

    fn wrap_coord(value: i64, max_value: u32) -> u32 {
        if value < 0 {
            return (max_value as i64 + (value % (max_value as i64))) as u32;
        } else {
            return (value % (max_value as i64)) as u32;
        }
    }

    fn count_neighbors(&self, x: u32, y: u32) -> i32 {
        let mut count = 0;

        for rel_x in -1..=1 {
            for rel_y in -1..=1 {
                if !(rel_x == 0 && rel_y == 0)
                    && self.at_wrap(x as i64 + rel_x, y as i64 + rel_y) == true
                {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn next(&self) -> Life {
        let mut result: Life = self.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let next = (self.next_value)(x, y, self);
                result.set(x, y, next);
            }
        }

        return result;
    }

    fn set(&mut self, x: u32, y: u32, value: bool) {
        *self.at_mut(x, y) = value;
    }

    fn at_wrap(&self, x: i64, y: i64) -> bool {
        if !self.wrap_indexes
            && (x < 0 || y < 0 || x >= (self.width as i64) || y >= (self.height as i64))
        {
            return false;
        }

        // x and y should both, mathematically, be >= 0 and < width or height when this is called
        return *self
            .data
            .get(
                (Life::wrap_coord(y, self.height) * self.width + Life::wrap_coord(x, self.width))
                    as usize,
            )
            .unwrap();
    }

    fn at(&self, x: u32, y: u32) -> bool {
        return *self.data.get((y * self.width + x) as usize).unwrap();
    }

    fn at_mut(&mut self, x: u32, y: u32) -> &mut bool {
        return self.data.get_mut((y * self.width + x) as usize).unwrap();
    }
}

fn conway_rules(x: u32, y: u32, life: &Life) -> bool {
    let neighbors = life.count_neighbors(x, y);
    let alive = life.at(x, y);
    return match (alive, neighbors) {
        (true, 2) => true,
        (true, 3) => true,
        (false, 3) => true,
        (_, _) => false,
    };
}

// google says rust doesn't have constructors?
fn make_life(width: u32, height: u32) -> Life {
    let life = Life {
        width: width,
        height: height,
        wrap_indexes: true,
        data: vec![false; (width * height) as usize],
        next_value: conway_rules,
    };
    return life;
}

fn main() -> Result<(), String> {

    let context = sdl2::init();

    let sdl_context = match context {
        Ok(result) => result,
        Err(message) => {
            println!("SDL reported error: '{}'", message);
            return Ok(());
        }
    };

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(16.0, 16.0);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame = 0;

    let mut history_of_life = std::collections::VecDeque::<Life>::new();

    history_of_life.push_back(|| -> Life {
        let mut life = make_life(50, 50);
        life.randomize();
        life
    }());


    'running: loop {
        frame += 1;

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();


        let mut draw = |life:&Life, greyscale:u8| {
            canvas.set_draw_color(Color::RGB(greyscale, greyscale, greyscale));

            for x in 0..life.width {
                for y in 0..life.height {
                    if life.at(x, y) {
                        canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
                    }
                }
            }
        };

        let mut grey = 125 - (10 * history_of_life.len());

        for life in &history_of_life {
            grey += 10;
            if grey >= 125 {
                grey = 255;
            }
            draw(&life, grey as u8);
        }

        if frame % 3 == 0 {
            history_of_life.push_back(history_of_life.back().unwrap().next());

            if history_of_life.len() > 10 {
                history_of_life.pop_front();
            }
        }

//        if frame % 100 == 0 {
//            history_of_life.back_mut().unwrap().draw_glider(25, 20);
//        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
