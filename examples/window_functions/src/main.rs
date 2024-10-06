use window_functions::*;

// All of the Window function that are available
enum WindowFunction {
    HAMMING,
    HANNING,
    BARTLETT,
    BLACKMAN,
}

fn get_window(window: WindowFunction, size: usize) -> Vec<f32> {
    match window {
        WindowFunction::HAMMING => window_functions::Hamming::window(size),
        WindowFunction::HANNING => window_functions::Hanning::window(size),
        WindowFunction::BARTLETT => window_functions::Bartlett::window(size),
        WindowFunction::BLACKMAN => window_functions::Blackman::window(size),
    }
}


fn main() {
    let window = get_window(WindowFunction::BLACKMAN, 51);
}
