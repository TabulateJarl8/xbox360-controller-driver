use enigo::{Enigo, MouseControllable, MouseButton, KeyboardControllable};
use gilrs::{Gilrs, Button, Event, EventType, Axis};

fn convert_to_lines(pressure: f32) -> i32 {
    // covers 0 case
    if pressure.abs() <= 2.0 {
        return 0
    }

    // if pressure <= 5.0 {
    //     (1.0 * pressure.signum()) as i32
    // } else {
    //     (2.0 * pressure.signum()) as i32
    // }
    (1.0 * pressure.signum()) as i32
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut mouse_x_vel: i32 = 0;
    let mut mouse_y_vel: i32 = 0;
    let mut scroll_x_vel: i32 = 0;
    let mut scroll_y_vel: i32 = 0;
    let mut backspace: bool = false;
    let mut enigo = Enigo::new();

    let mut cycle: i32 = 0;
    let mut slower_cycle: i32 = 0;

    loop {
        cycle += 1;
        if cycle == 20000 {
            enigo.mouse_scroll_y(scroll_y_vel);
            enigo.mouse_scroll_x(scroll_x_vel);
            enigo.mouse_move_relative(mouse_x_vel, mouse_y_vel);

            slower_cycle += 1;
            cycle = 0;

            if slower_cycle == 15 {
                if backspace == true {
                    enigo.key_sequence_parse("{+UNICODE}\u{0008}{-UNICODE}");
                }
                slower_cycle = 0;
            }
        }


        // Examine new events
        while let Some(Event { id: _, event, .. }) = gilrs.next_event() {
            // println!("{:?} New event from {}: {:?}", time, id, event);

            match event {
                EventType::AxisChanged(Axis::LeftStickX, direction, _) => {
                    mouse_x_vel = ((direction * 10.0).floor()) as i32;
                    if mouse_x_vel.abs() <= 2 {
                        mouse_x_vel = 0;
                    }
                }
                EventType::AxisChanged(Axis::LeftStickY, direction, _) => {
                    mouse_y_vel = -((direction * 10.0).floor()) as i32;
                    if mouse_y_vel.abs() <= 2 {
                        mouse_y_vel = 0;
                    }
                }
                EventType::AxisChanged(Axis::RightStickX, direction, _) => {
                    scroll_x_vel = convert_to_lines((direction * 10.0).floor());
                }
                EventType::AxisChanged(Axis::RightStickY, direction, _) => {
                    scroll_y_vel = -convert_to_lines((direction * 10.0).floor());
                }
                EventType::ButtonPressed(Button::South, _) => {
                    enigo.mouse_down(MouseButton::Left);
                }
                EventType::ButtonReleased(Button::South, _) => {
                    enigo.mouse_up(MouseButton::Left);
                }
                EventType::ButtonPressed(Button::East, _) => {
                    backspace = true;
                }
                EventType::ButtonReleased(Button::East, _) => {
                    backspace = false;
                }
                EventType::ButtonChanged(..) => {}
                _ => {println!("{:?}", event);}
            }
        }
    }
}