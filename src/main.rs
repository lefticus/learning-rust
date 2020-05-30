use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;

mod test;


/*
fn generic_function<'T, Type1, Type2>(val:&'T Type1, val2:&'T Type2) -> &'T Type1
{
    return val;

    // @jntrnr on Twitter
}
*/

fn main() -> Result<(), String> {
    //test::funny_name();
    //generic_function(&4);
    //test::other_function();


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
