use enigo::{Enigo, MouseControllable, MouseButton};
use gilrs::{Gilrs, Button, Event, EventType, Axis};

fn convert_to_lines(pressure: f32) -> i32 {
    // covers 0 case
    if pressure.abs() <= 2.0 {
        return 0
    }

    if pressure <= 5.0 {
        (1.0 * pressure.signum()) as i32
    } else {
        (2.0 * pressure.signum()) as i32
    }
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
    let mut cycle: i32 = 0;
    let mut enigo = Enigo::new();

    loop {
        cycle += 1;
        if cycle == 10000 {
            enigo.mouse_scroll_y(scroll_y_vel);
            enigo.mouse_scroll_x(scroll_x_vel);
            enigo.mouse_move_relative(mouse_x_vel, mouse_y_vel);
            cycle = 0;
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
                _ => {println!("{:?}", event);}
            }
        }
    }
}