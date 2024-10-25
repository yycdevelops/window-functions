use core::f32;

use crate::{CosineWindow, Window};

pub struct Hamming;

impl CosineWindow<f32> for Hamming {
    fn cosine_window(window: &mut Vec<f32>) {
        let size: usize = window.len();
        window.iter_mut().for_each(|x| {
            let multipler = f32::consts::PI * 2_f32 * *x / (size as f32 - 1_f32);
            *x = 0.54 - 0.46 * multipler.cos();
        });
    }
}

impl Window<Vec<f32>> for Hamming {
    fn window(num: usize) -> Vec<f32> {
        //let mut window: Vec<_> = (0u8..num as u8).map(f32::from).collect();
        let mut window: Vec<_> = fill_vec!(num);
        Self::cosine_window(&mut window);
        window
    }
}

#[cfg(test)]
mod tests {
    use crate::Window;
    use super::Hamming;

    fn get_sample_data() -> Vec<f32> {
        vec![0.08, 0.18761956, 0.46012184, 0.77, 0.97225861, 0.97225861, 0.77, 0.46012184, 0.18761956, 0.08]
    }

    #[test]
    fn test_window() {
        let window: Vec<f32> = Hamming::window(10);
        let test_data = get_sample_data();  
        assert_with_decimal_places!(window, test_data);
    }
}