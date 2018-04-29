pub use controller::gamepad;
use controller::gamepad::Gamepad;
use model::game::SnakeGame;
use model::shared::DOWN;
use model::shared::LEFT;
use model::shared::RIGHT;
use model::shared::UP;
pub use self::evdev::*;
pub use self::evdev::enums::*;
use view::unicornhd::*;
use controller::gamepad::print_event;
/* TODO AVOID THIS REIMPORT */
extern crate evdev_rs as evdev;


pub fn process_input_event(snake_game : &mut self::SnakeGame, _gamepad : &Gamepad, evt: &InputEvent) {

    match evt.event_type {
        EventType::EV_ABS => {
            match evt.event_code {
                EventCode::EV_ABS(EV_ABS::ABS_Y) => {
                    if evt.value == 0 {
                        snake_game.get_snake_mut().set_next(UP);
                    } else if evt.value == 255 {
                        snake_game.get_snake_mut().set_next(DOWN);
                    } else if evt.value == 127 {
                            //This is button release, do nothing
                    } else {
                        print_event(evt);
                        panic!("Unknown Y value {:?}", evt.value);
                    }

                },
                EventCode::EV_ABS(EV_ABS::ABS_X) => {
                    if evt.value == 0 {
                        snake_game.get_snake_mut().set_next(LEFT);
                    } else if evt.value == 255 {
                        snake_game.get_snake_mut().set_next(RIGHT);
                    } else if evt.value == 127 {
                        //This is button release, do nothing
                    } else {
                        print_event(evt);
                        panic!("Unknown X value {:?}", evt.value);
                    }

                },
                _ => {}
            }
        },
        EventType::EV_KEY => {
            match evt.event_code {
                EventCode::EV_KEY(EV_KEY::BTN_TRIGGER) => {
                    if evt.value == 1 {
                        update_current_snake_color(&BLUE);
                    }
                },
                EventCode::EV_KEY(EV_KEY::BTN_TOP) => {
                    if evt.value == 1 {
                        update_current_snake_color(&GREEN);
                    }
                },
                EventCode::EV_KEY(EV_KEY::BTN_THUMB) => {
                    if evt.value == 1 {
                        update_current_snake_color(&RED);
                    }
                },
                EventCode::EV_KEY(EV_KEY::BTN_THUMB2) => {
                    if evt.value == 1 {
                        update_current_snake_color(&YELLOW);
                    }
                },
                _ => {}
            }
        }
        _ => {}

    }
}