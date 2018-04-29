use model::game::GameState;
use model::game::SnakeGame;
use self::rgb::*;
use self::unicorn_hat_hd::UnicornHatHd;
use view::snakedisplay::SnakeDisplay;

extern crate rgb;
extern crate unicorn_hat_hd;

pub static GREEN: RGB8 = RGB8 { r: 0, g: 255, b: 0 };
pub const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
pub const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 255 };
pub const YELLOW: RGB8 = RGB8 { r: 241, g: 244, b: 66 };
pub const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };
pub const WHITE: RGB8 = RGB8 { r: 255, g: 255, b: 255 };


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
            for y in 0..16 {
                for x in 0..16 {
                    hat_hd.set_pixel(x, y, BLACK);
                }
            }
        }

        fn render_finished(hat_hd: &mut UnicornHatHd, _snake_game: &SnakeGame) {
            for y in 0..16 {
                for x in 0..16 {
                    hat_hd.set_pixel(x, y, RED);
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

