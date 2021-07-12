const KEYFRAMES: [i32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 7, 6, 5, 4, 3, 2, 1, 0];

extern "C" {
    fn set_led(led_index: i32, r: i32, g: i32, b: i32);
}

#[no_mangle]
pub extern "C" fn sensor_update(_sensor_id: i32, _sensor_value: f64) -> f64 {
    // NO-OP, don't really care about sensor values
    0.0
}

/// apply animates the LEDs based on the frame.
///
/// #Arguments
///
/// frame - A i32 parameter to calculate the KEYFRAME index to pass to light the LED.

#[no_mangle]
pub extern "C" fn apply(frame: i32) {
    let idx = frame % 16;

    for index in 0..8 {
        unsafe {
            set_led(index, 0, 0, 0);
        }
    }
    unsafe {
        set_led(KEYFRAMES[idx as usize], 255, 0, 0); // (4)
    }
}
