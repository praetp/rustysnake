use crate::model::game::{GameState, SnakeGame};
use self::rgb::*;
use self::unicorn_hat_hd::UnicornHatHd;
use super::snakedisplay::SnakeDisplay;
use image::{ImageBuffer, Pixel, Rgb};
use rusttype::Font;
use rusttype::Scale;

extern crate rgb;
extern crate unicorn_hat_hd;

pub static GREEN: RGB8 = RGB8 { r: 0, g: 255, b: 0 };
pub const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
const RED2 : Rgb<u8> = Rgb([255, 0 , 0]);


pub const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 255 };
pub const YELLOW: RGB8 = RGB8 { r: 241, g: 244, b: 66 };
pub const WHITE: RGB8 = RGB8 { r: 255, g: 255, b: 255 };

const WIDTH: u8 = 16;
const HEIGHT: u8 = 16;


static mut CURRENT_SNAKE_COLOR: RGB8 = YELLOW;


impl SnakeDisplay for UnicornHatHd {

    fn render(&mut self, snake_game: &SnakeGame) {

        fn render_running(hat_hd: &mut UnicornHatHd, snake_game: &SnakeGame) {
            let bait: RGB8 = WHITE;
            let tmp;
            unsafe {
                tmp = CURRENT_SNAKE_COLOR;
            }
            for snake_segment in snake_game.get_snake().get_segments() {
                hat_hd.set_pixel(snake_segment.x as usize, snake_segment.y as usize, tmp);
            }

            let baitpos = snake_game.get_bait();
            hat_hd.set_pixel(baitpos.x as usize, baitpos.y as usize, bait);
        }

        fn clear(hat_hd: &mut UnicornHatHd) {
            hat_hd.clear_pixels();
        }

        fn render_finished(hat_hd: &mut UnicornHatHd, snake_game: &SnakeGame) {
            let font_data: &[u8] = include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf");
            let font: Font<'static> = Font::try_from_bytes(font_data).expect("could not load font");
            let storage = vec![0; 4 * WIDTH as usize * HEIGHT as usize];
            let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(WIDTH.into(), HEIGHT.into(), storage).expect("could not build image buffer");
            
            let score_string = snake_game.get_score().to_string();
            imageproc::drawing::draw_text_mut(&mut image, RED2, 0, 0, Scale {x: 16.0, y: 16.0},  &font, score_string.as_ref());

            hat_hd.clear_pixels();
            for y in 0..16 {
                for x in 0..16 {
                    let pixel = image.get_pixel(x, y);
                    let channels = pixel.channels();
                    let color = RGB8 { r: channels[0], g: channels[1], b: channels[2] };
                    hat_hd.set_pixel(x as usize, y as usize, color);
                }
            }
        }

        clear(self);

        match *snake_game.get_state() {
            GameState::RUNNING => render_running(self, snake_game),
            GameState::FINISHED => render_finished(self, snake_game)
        }

        self.display().unwrap();
    }
}

pub fn new() -> UnicornHatHd {
    UnicornHatHd::default()
}


pub fn update_current_snake_color(new_color: &RGB8) {
    unsafe {
        CURRENT_SNAKE_COLOR = *new_color;
    }
}

