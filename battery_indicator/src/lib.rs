#[derive(PartialEq, Debug, Clone)]
struct LedColor(i32, i32, i32);

const SENSOR_BATTERY: i32 = 20;

const OFF: LedColor = LedColor(0, 0, 0);
const YELLOW: LedColor = LedColor(255, 255, 0);
const GREEN: LedColor = LedColor(0, 255, 0);
const RED: LedColor = LedColor(255, 0, 0);
const PCT_PER_PIXEL: f64 = 12.5_f64;

extern "C" {
    fn set_led(led_index: i32, r: i32, g: i32, b: i32);
}

/// sensor_update function lights up the LEDs based on sensor input.
///
/// #Arguments
///
/// sensor_id - An i32 parameter denoting the id of a sensor.
/// sensor_value - A f64 parameter denoting the battery percentage.
///
/// #Return
///
/// An f64 value denoting battery percentage.

#[no_mangle]
pub extern "C" fn sensor_update(sensor_id: i32, sensor_value: f64) -> f64 {
    if sensor_id == SENSOR_BATTERY {
        set_leds(get_led_values(sensor_value));
    }
    sensor_value
}

#[no_mangle]
pub extern "C" fn apply(_frame: u32) {
    // NO OP, not an animated indicator
}

/// get_led_values tells the number of lEDs to light up and their respective colors.
///
/// #Arguments
///
/// battery_remaining - A f64 parameter denoting the remaining battery percentage.
///
/// #Return
///
/// A vector of LedColor type objects denoting LEDs and their colors.

fn get_led_values(battery_remaining: f64) -> [LedColor; 8] {
    let mut arr: [LedColor; 8] = [OFF, OFF, OFF, OFF, OFF, OFF, OFF, OFF];
    if battery_remaining < 0 as f64 || battery_remaining > 100 as f64 {
        return arr.clone();
    }
    let lit = (battery_remaining / PCT_PER_PIXEL).ceil();

    // 0 - 20 : Red
    // 21 - <50 : Yellow
    // 51 - 100 : Green

    let color = if 0.0 <= battery_remaining && battery_remaining <= 20.0 {
        RED
    } else if battery_remaining > 20.0 && battery_remaining < 50.0 {
        YELLOW
    } else {
        GREEN
    };

    for idx in 0..lit as usize {
        arr[idx] = color.clone();
    }

    arr
}

/// set_leds function sets the desired LEDs to ON state with proper colors.
///
/// #Arguments
///
/// values - A vector of LedColor type objects denoting LEDs and their colors

fn set_leds(values: [LedColor; 8]) {
    for x in 0..8 {
        let LedColor(r, g, b) = values[x];
        unsafe {
            set_led(x as i32, r, g, b);
        }
    }
}
#[cfg(test)]
mod tests {

    use {get_led_values, GREEN, OFF, RED, YELLOW};

    #[test]
    fn get_led_values_success_0() {
        assert_eq!(
            get_led_values(0.0),
            [OFF, OFF, OFF, OFF, OFF, OFF, OFF, OFF]
        );
    }

    #[test]
    fn get_led_values_success_15() {
        assert_eq!(
            get_led_values(15.0),
            [RED, RED, OFF, OFF, OFF, OFF, OFF, OFF]
        );
    }

    #[test]
    fn get_led_values_success_49() {
        assert_eq!(
            get_led_values(49.0),
            [YELLOW, YELLOW, YELLOW, YELLOW, OFF, OFF, OFF, OFF]
        );
    }

    #[test]
    fn get_led_values_success_75() {
        assert_eq!(
            get_led_values(75.0),
            [GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, OFF, OFF]
        );
    }

    #[test]
    fn get_led_values_success_100() {
        assert_eq!(
            get_led_values(100.0),
            [GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN]
        );
    }

    #[test]
    fn get_led_values_failure() {
        assert_eq!(
            get_led_values(-10.0),
            [OFF, OFF, OFF, OFF, OFF, OFF, OFF, OFF]
        );
    }
}
