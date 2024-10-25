use core::f32;

use crate::{CosineWindow, Window};

pub struct Hanning;


impl CosineWindow<f32> for Hanning {
    fn cosine_window(window: &mut Vec<f32>) {
        let size: usize = window.len();
        window.iter_mut().for_each(|x| {
            let multiplier = f32::consts::PI * 2_f32 * *x / (size as f32 - 1_f32);
            *x = 0.5 - 0.5 * multiplier.cos();
        });
    }
}

impl Window<Vec<f32>> for Hanning {
    fn window(num: usize) -> Vec<f32> {
        //let mut window: Vec<_> = (0u8 .. num as u8).map(f32::from).collect();
        let mut window: Vec<_> = fill_vec!(num);
        Self::cosine_window(&mut window);
        window  
    }
}


#[cfg(test)]
mod tests {
    use crate::Window;
    use super::Hanning;

    fn get_sample_data() -> Vec<f32> {
        vec![0.0, 0.11697778, 0.41317591, 0.75, 0.96984631, 0.96984631, 0.75, 0.41317591, 0.11697778, 0.0]
    }

    #[test]
    fn test_window() {
        let window: Vec<f32> = Hanning::window(10);
        let test_data = get_sample_data();  
        assert_with_decimal_places!(window, test_data);
    }
}