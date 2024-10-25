use core::f32;

use crate::Window;

pub struct Bartlett;

impl Window<Vec<f32>> for Bartlett {
    fn window(num: usize) -> Vec<f32> {
        let window_bartlett: Vec<_> = (0..num).map(|x| {
            let multi = 2_f32 * (x as f32 - 0.5 * (num as f32- 1_f32)) / (num as f32 - 1_f32);
            1_f32- multi.abs()
        }).collect();
        window_bartlett
    }
}

#[cfg(test)]
mod tests {
    use crate::Window;
    use super::Bartlett;

    fn get_sample_data() -> Vec<f32> {
        vec![0.0, 0.22222222, 0.44444444, 0.66666667, 0.88888889, 0.88888889, 0.66666667, 0.44444444, 0.22222222, 0.0]
    }

    #[test]
    fn test_window() {
        let window: Vec<f32> = Bartlett::window(10);
        let test_data = get_sample_data();  
        assert_with_decimal_places!(window, test_data);
    }
}