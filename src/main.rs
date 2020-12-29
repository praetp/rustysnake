use controller::gamepad as gamepad;
use controller::gamepad::Gamepad;
use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::unix::EventedFd;
use model::game::*;
use self::unicorn_hat_hd::UnicornHatHd;
use std::cmp;
use std::env;
use std::time::Duration;
use std::time::SystemTime;
use view::snakedisplay::SnakeDisplay;
use view::unicornhd;

extern crate libc;
extern crate mio;
extern crate unicorn_hat_hd;


mod model;
mod view;
mod controller;

fn main() {
    println!("Hello, world!");
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); /* remove executable */
    match args.len() {
        0 => println!("You need to a one device file"),
        1 => {
            let path = args.first();
            start(path.unwrap());
        },
        _ => println!("Only one device supported")
    }

    println!("Stopping..");

}


fn start(device_path: &String) {

    let gamepad : gamepad::Gamepad = match gamepad::from_path(device_path) {
        Ok(gp) => gp,
        Err(e) => panic!("failed to create Gamepad; err={:?}", e),
    };
    let fd = gamepad.get_fd();

    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFL);
        libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
    }

    let mut display : UnicornHatHd = unicornhd::new();

    let mut snakegame = SnakeGame::new();
    game_loop(&mut snakegame, &gamepad, &mut display);
}



fn poll_timeout(last_update: SystemTime, tick_duration : &Duration) -> Duration {
    let lapsed = SystemTime::now().duration_since(last_update).expect("SystemTime::duration_since failed");
    if *tick_duration < lapsed {
        return Duration::from_millis(1);
    } else {
        let timeout =  *tick_duration - lapsed;
        return timeout;
    }
}

fn game_loop(snakegame : &mut SnakeGame, gamepad: &Gamepad, display: &mut dyn SnakeDisplay) {

    let poll = match Poll::new() {
        Ok(poll) => poll,
        Err(e) => panic!("failed to create Poll instance; err={:?}", e),
    };
    let token = Token(gamepad.get_id());

    poll.register(&EventedFd(&gamepad.get_fd()), token, Ready::readable(), PollOpt::level()).unwrap();


    let mut events = Events::with_capacity(64);
    let mut tick_duration : Duration = Duration::from_millis(500);

    let mut last_update = SystemTime::now();
    while *snakegame.get_state() == GameState::RUNNING {
        poll.poll(&mut events, Some(poll_timeout(last_update, &tick_duration))).unwrap();
        {

            for event in &events {
                if event.token() == Token(gamepad.get_id()) {
                    gamepad.process_gamepad_event(|gamepad, evt| {
                        controller::snake::process_input_event(snakegame, gamepad, evt);

                    });
                }
            }

            let now = SystemTime::now();
            let lapsed = now.duration_since(last_update).expect("SystemTime::duration_since failed");
            if lapsed > tick_duration {
                let result = snakegame.update();
                if result == MoveResult::BAIT {
                    tick_duration = cmp::max(Duration::from_millis(25),
                                             tick_duration - Duration::from_millis(20));
                }

                display.render(snakegame);
                last_update = now;
            }
        }
    }

}