use core::f32;

pub trait Window<T> {
    fn window(num: usize) -> T;
}

trait CosineWindow<T> {
    fn cosine_window(window: &mut Vec<T>);
}

pub struct Hanning {}
pub struct Hamming {}
pub struct Bartlett {}
pub struct Blackman {}


impl CosineWindow<f32> for Hanning {
    fn cosine_window(window: &mut Vec<f32>) {
        let size: usize = window.len();
        window.iter_mut().for_each(|x| {
            let multiplier = f32::consts::PI * 2_f32 * *x / (size as f32 - 1_f32);
            *x = 0.5 - 0.5 * multiplier.cos();
        });
    }
}

impl CosineWindow<f32> for Hamming {
    fn cosine_window(window: &mut Vec<f32>) {
        let size: usize = window.len();
        window.iter_mut().for_each(|x| {
            let multipler = f32::consts::PI * 2_f32 * *x / (size as f32 - 1_f32);
            *x = 0.54 - 0.46 * multipler.cos();
        });
    }
}

impl CosineWindow<f32> for Blackman {
    fn cosine_window(window: &mut Vec<f32>) {
        let co1 = 0.42; 
        let co2 = 0.5; 
        let co3 = 0.08;
        let size: usize = window.len();
        
        window.iter_mut().for_each(|x| {
            let multiplier_left = f32::consts::PI * 2_f32 * *x / size as f32;
            let multiplier_right = f32::consts::PI * 4_f32 * *x / size as f32;
            *x = co1 - co2 * multiplier_left.cos() + co3 * multiplier_right.cos();
        });
    }
}

impl Window<Vec<f32>> for Hanning {
    fn window(num: usize) -> Vec<f32> {
        let mut window: Vec<_> = (0u8 .. num as u8).map(f32::from).collect();
        Self::cosine_window(&mut window);
        window  
    }
}

impl Window<Vec<f32>> for Hamming {
    fn window(num: usize) -> Vec<f32> {
        let mut window: Vec<_> = (0u8..num as u8).map(f32::from).collect();
        Self::cosine_window(&mut window);
        window
    }
}

impl Window<Vec<f32>> for Bartlett {
    fn window(num: usize) -> Vec<f32> {
        let window_bartlett: Vec<_> = (0..num).map(|x| {
            let multi = 2_f32 * (x as f32 - 0.5 * (num as f32- 1_f32)) / (num as f32 - 1_f32);
            1_f32- multi.abs()
        }).collect();
        window_bartlett
    }
}

impl Window<Vec<f32>> for Blackman {
    fn window(num: usize) -> Vec<f32> {
        let mut window_blackman: Vec<_> = (0u8..num as u8).map(f32::from).collect();
        Self::cosine_window(&mut window_blackman);
        window_blackman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanning_window() {
        let window = Hanning::window(51);
        println!("{:?}", window);
    }

    #[test]
    fn test_hamming_window() {
        let window = Hamming::window(51);
        println!("{:?}", window);
    }

    #[test]
    fn test_barlet() {
        let window = Bartlett::window(51);
        println!("{:?}", window);
    }

    #[test]
    fn test_blackman() {
        let window = Blackman::window(51);
        println!("{:?}", window);
    }
}
