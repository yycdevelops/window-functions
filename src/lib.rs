
trait Window<T, I> {
    fn window(num: T) -> I;
}

trait CosineWindow<T> {
    fn cosine_window(v: &mut Vec<T>);
}

pub const PI: f32 = 3.14159265358979323846264338327950288_f32;

struct Hann; 

impl CosineWindow<f32> for Hann {
    fn cosine_window(mut window: &mut Vec<f32>) {
        let size: usize = window.len();
        window.iter_mut().for_each(|x| {
            let multiplier = PI * 2 as f32 * *x as f32 / (size as f32 - 1 as f32);
            *x = 0.5 - 0.5 * multiplier.cos();
        });
    }
}

impl Window<usize, Vec<f32>> for Hann {
    fn window(num: usize) -> Vec<f32> {
        let mut window: Vec<_> = (0u8 .. num as u8).map(f32::from).collect();
        Self::cosine_window(&mut window);
        window  
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanning_window() {
        let window = Hann::window(51);

    }
}