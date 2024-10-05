
trait Window<T> {
    fn window(num: usize) -> T;
}

trait CosineWindow<T> {
    fn cosine_window(window: &mut Vec<T>);
}

pub const PI: f32 = 3.14159265358979323846264338327950288_f32;

struct Hanning {}
struct Hamming {}

impl CosineWindow<f32> for Hanning {
    fn cosine_window(window: &mut Vec<f32>) {
        let size: usize = window.len();
        window.iter_mut().for_each(|x| {
            let multiplier = PI * 2_f32 * *x / (size as f32 - 1_f32);
            *x = 0.5 - 0.5 * multiplier.cos();
        });
    }
}

impl CosineWindow<f32> for Hamming {
    fn cosine_window(window: &mut Vec<f32>) {
        let size: usize = window.len();
        window.iter_mut().for_each(|x| {
            let multipler = PI * 2_f32 * *x / (size as f32 - 1_f32);
            *x = 0.54 - 0.46 * multipler.cos();
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
}
