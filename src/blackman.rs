use crate::{CosineWindow, Window};
pub struct Blackman;

impl CosineWindow<f32> for Blackman {
    fn cosine_window(window: &mut Vec<f32>) {
        let co1 = 0.42; 
        let co2 = 0.5; 
        let co3 = 0.08;
        let size: usize = window.len();
        
        window.iter_mut().for_each(|x| {
            let multiplier_left = std::f32::consts::PI * 2_f32 * *x / size as f32;
            let multiplier_right = std::f32::consts::PI * 4_f32 * *x / size as f32;
            *x = co1 - co2 * (multiplier_left.cos() + co3 * multiplier_right.cos());
        });
    }
}

impl Window<Vec<f32>> for Blackman {
    fn window(num: usize) -> Vec<f32> {
        let mut window_blackman = fill_vec!(num);
        Self::cosine_window(&mut window_blackman);
        window_blackman
    }
}

#[cfg(test)]
mod tests {
    use crate::Window;
    use super::Blackman;

    fn get_sample_data() -> Vec<f32> {
        vec![-1.38777878e-17, 5.08696327e-02, 2.58000502e-01, 6.30000000e-01, 9.51129866e-01, 9.51129866e-01, 6.30000000e-01, 2.58000502e-01, 5.08696327e-02, -1.38777878e-17]
    }

    #[test]
    fn test_window() {
        let window: Vec<f32> = Blackman::window(10);
        let test_data = get_sample_data();  
        assert_with_decimal_places!(window, test_data);
    }
}