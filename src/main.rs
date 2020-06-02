use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;

mod test;

// is it possible to pass width and height as compile-time constants?
#[derive(Clone)]
struct Life<ValueType> {
    width: u32,
    height: u32,
    wrap_indexes: bool,
    data: Vec<ValueType>,
    default_value: ValueType,
    next_value: fn(u32,u32,&Life) -> ValueType,
}

impl<ValueType: Copy + PartialEq> Life<ValueType> {
    fn wrap_coord(value: i64, max_value: u32) -> u32 {
        if value < 0 {
            return max_value + ((value % (max_value as i64)) as u32);
        } else {    
            return (value % (max_value as i64)) as u32;
        }
    }

    fn count_neighbors(&self, x: u32, y: u32, expected: ValueType) {
        let count = 0;

        for rel_x in -1..=1 {
            for rel_y in -1..=1 {
                if !(rel_x == 0 && rel_y == 0)
                    && *self.at(x as i64 + rel_x, y as i64 + rel_y) == expected
                {
                    count += 1;
                }
            }
        }
    }

    fn next(&self) -> Life<ValueType> {
        let mut result: Life<ValueType> = self.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let next = (self.next_value)(x,y,self);
                result.at_mut(x as i64, y as i64) = next;
            }
        }

        return result;
    }

    fn at(&self, x: i64, y: i64) -> &ValueType {
        if !self.wrap_indexes
            && (x < 0 || y < 0 || x >= (self.width as i64) || y >= (self.height as i64))
        {
            return &self.default_value;
        }

        // x and y should both, mathematically, be >= 0 and < width or height when this is called
        return self
            .data
            .get(
                (Life::<ValueType>::wrap_coord(y, self.height) * self.height
                    + Life::<ValueType>::wrap_coord(x, self.width)) as usize,
            )
            .unwrap();
    }

    // allow signed integral large enough to index any u32 * u32 position
    // but allow wrap around for ease of use for edge cases
    fn at_mut(&mut self, x: i64, y: i64) -> &mut ValueType {
        if x < 0 || y < 0 || x >= (self.width as i64) || y >= (self.height as i64) {
            assert!(self.wrap_indexes);
        }

        // x and y should both, mathematically, be >= 0 and < width or height when this is called
        return self
            .data
            .get_mut(
                (Life::<ValueType>::wrap_coord(y, self.height) * self.height
                    + Life::<ValueType>::wrap_coord(x, self.width)) as usize,
            )
            .unwrap();
    }
}

fn next_value(x:u32, y:u32, life: &Life<bool>) -> bool
{
    return false;
}

// google says rust doesn't have constructors?
fn make_life<ValueType: Copy + PartialEq>(
    width: u32,
    height: u32,
    default_value: ValueType,
) -> Life<ValueType> {
    let life = Life::<ValueType> {
        width: width,
        height: height,
        wrap_indexes: true,
        data: vec![default_value; (width * height) as usize],
        default_value: default_value,
        next_value: next_value
    };
    return life;
}

fn main() -> Result<(), String> {
    let life = make_life(32, 32, false);

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
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
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
