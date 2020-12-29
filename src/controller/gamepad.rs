pub use self::evdev::*;
pub use self::evdev::enums::*;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::os::unix::prelude::*;

extern crate evdev_rs as evdev;


pub struct Gamepad {
    devpath: String,
    device: Device,
    raw_fd: i32,
    last_event: Option<MyInputEvent>
}

#[derive(Clone)]
struct MyInputEvent {
    pub event_type: EventType,
    pub event_code: EventCode,
    pub value: i32
}

impl MyInputEvent {
    fn from(input_event : &InputEvent) -> MyInputEvent {
        MyInputEvent {
            event_code: input_event.event_code.clone(),
            event_type: input_event.event_type.clone(),
            value: input_event.value
        }
    }
}

impl Hash for Gamepad {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.devpath.hash(state);
    }
}

impl PartialEq for Gamepad {
    fn eq(&self, other: &Gamepad) -> bool {
        self.devpath == other.devpath
    }
}

impl Eq for Gamepad {}

pub fn from_path(path: &String) -> io::Result<Gamepad> {

    let f = File::open(&path).expect("file not found");
    let rawfd = f.as_raw_fd(); //maybe not so nice as rawfd could outlive f (which is owned by device)
    let d = Device::new_from_fd(f).expect("could not open device");

    let gp = Gamepad {
        devpath: path.clone(),
        device: d,
        raw_fd: rawfd,
        last_event: None
    };

    return Ok(gp);
}

#[allow(dead_code)]
fn print_info(d: &Device) {
    println!("Input device ID: bus 0x{:x} vendor 0x{:x} product 0x{:x}",
             d.bustype(),
             d.vendor_id(),
             d.product_id());
    println!("Evdev version: {:x}", d.driver_version());
    println!("Input device name: \"{}\"", d.name().unwrap_or(""));
    println!("Phys location: {}", d.phys().unwrap_or(""));
    println!("Uniq identifier: {}", d.uniq().unwrap_or(""));
}

#[allow(dead_code)]
fn print_bits(dev: &Device) {
    println!("Supported events:");

    for ev_type in  EventType::EV_SYN.iter() {
        if dev.has(&ev_type) {
            println!("  Event type: {} ", ev_type);
        }

        match ev_type {
            EventType::EV_KEY => print_code_bits(dev, &EventCode::EV_KEY(EV_KEY::KEY_RESERVED),
                                                 &EventCode::EV_KEY(EV_KEY::KEY_MAX)),
            EventType::EV_REL => print_code_bits(dev, &EventCode::EV_REL(EV_REL::REL_X),
                                                 &EventCode::EV_REL(EV_REL::REL_MAX)),
            EventType::EV_ABS => print_code_bits(dev, &EventCode::EV_ABS(EV_ABS::ABS_X),
                                                 &EventCode::EV_ABS(EV_ABS::ABS_MAX)),
            EventType::EV_LED => print_code_bits(dev, &EventCode::EV_LED(EV_LED::LED_NUML),
                                                 &EventCode::EV_LED(EV_LED::LED_MAX)),
            _ => (),
        }
    }
}

#[allow(dead_code)]
fn print_abs_bits(dev: &Device, axis: &EV_ABS) {

    let code = EventCode::EV_ABS(axis.clone());

    if !dev.has(&code) { return; }

    let abs = dev.abs_info(&code).unwrap();

    println!("	Value	{}", abs.value);
    println!("	Min	{}", abs.minimum);
    println!("	Max	{}", abs.maximum);
    if abs.fuzz != 0 {
        println!("	Fuzz	{}", abs.fuzz);
    }
    if abs.flat != 0 {
        println!("	Flat	{}", abs.flat);
    }
    if abs.resolution != 0 {
        println!("	Resolution	{}", abs.resolution);
    }
}

#[allow(dead_code)]
fn print_code_bits(dev: &Device, ev_code: &EventCode, max: &EventCode) {
    for code in ev_code.iter() {
        if code == *max {
            break;
        }
        if !dev.has(&code) {
            continue;
        }

        println!("    Event code: {}", code);
        match code {
            EventCode::EV_ABS(k) => print_abs_bits(dev, &k),
            _ => ()
        }
    }
}

#[allow(dead_code)]
fn print_props(dev: &Device) {
    println!("Properties:");

    for input_prop in InputProp::INPUT_PROP_POINTER.iter() {
        if dev.has(&input_prop) {
            println!("  Property type: {}", input_prop);
        }
    }
}

pub fn print_event(ev: &InputEvent) {
    match ev.event_type {
        EventType::EV_SYN => {
            println!("Event: time {}.{}, ++++++++++++++++++++ {} +++++++++++++++",
                     ev.time.tv_sec,
                     ev.time.tv_usec,
                     ev.event_type);
        }
        _ =>  {
            println!("Event: time {}.{}, type {} , code {} , value {}",
                     ev.time.tv_sec,
                     ev.time.tv_usec,
                     ev.event_type,
                     ev.event_code,
                     ev.value);
        }
    }
}

fn print_sync_dropped_event(ev: &InputEvent) {
    print!("SYNC DROPPED: ");
    print_event(ev);
}

impl Gamepad {

    #[allow(dead_code)]
    pub fn set_last_event(&mut self, evt: &InputEvent) {
        self.last_event = Some(MyInputEvent::from(evt));
    }

    pub fn get_id(&self) -> usize {
        self.get_fd() as usize
    }

    pub fn get_fd(&self) -> i32 {
        self.raw_fd
    }

    pub fn process_gamepad_event<F: FnMut(&Gamepad, &InputEvent)>(&self, mut f: F) {
       loop {
            let a = self.device.next_event(evdev::ReadFlag::NORMAL);
            if a.is_ok() {
                let mut result = a.ok().unwrap();
                match result.0 {
                    ReadStatus::Sync => {
                        println!("::::::::::::::::::::: dropped ::::::::::::::::::::::");
                        while result.0 == ReadStatus::Sync {
                            print_sync_dropped_event(&result.1);
                            let a = self.device.next_event(evdev::ReadFlag::SYNC);
                            if a.is_ok() {
                                result = a.ok().unwrap();
                            } else {
                                break;
                            }
                        }
                        println!("::::::::::::::::::::: re-synced ::::::::::::::::::::");
                    },
                    ReadStatus::Success => {
                        f(self, &result.1);
                    },
                }
            } else {
                let err = a.err().unwrap();
                match err.raw_os_error() {
                    Some(libc::EAGAIN) => {
                        break;
                    },
                    _ => {
                        println!("{}", err);
                        break;
                    }
                }
            }
        }
    }


}
/* use traits */
