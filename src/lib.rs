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
            *x = co1 - co2 * (multiplier_left.cos() + co3 * multiplier_right.cos());
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
    use helpers::assert_with_decimal_places;
    use test_setup::*;  
    /*
        Given the Widow function implementation is a direct match to what numpy provides, the tests 
        consist of comparing data returned in numpy (Python) to the values produced. 

    */  

    mod test_setup {
        use super::Bartlett;
        use super::Blackman;
        use super::Hamming;
        use super::Hanning;

        pub trait SampleData {
            fn sample_data() -> Vec<f32>;
        }

        impl SampleData for Hanning {
            fn sample_data() -> Vec<f32> {
                vec![0.0, 0.11697778, 0.41317591, 0.75, 0.96984631, 0.96984631, 0.75, 0.41317591, 0.11697778, 0.0]
            }
        }

        impl SampleData for Hamming {
            fn sample_data() -> Vec<f32> {
                vec![0.08, 0.18761956, 0.46012184, 0.77, 0.97225861, 0.97225861, 0.77, 0.46012184, 0.18761956, 0.08]
            }
        }

        impl SampleData for Bartlett {
            fn sample_data() -> Vec<f32> {
                vec![0.0, 0.22222222, 0.44444444, 0.66666667, 0.88888889, 0.88888889, 0.66666667, 0.44444444, 0.22222222, 0.0]
            }
            
        }

        impl SampleData for Blackman {
            fn sample_data() -> Vec<f32> {
                vec![-1.38777878e-17, 5.08696327e-02, 2.58000502e-01, 6.30000000e-01, 9.51129866e-01, 9.51129866e-01, 6.30000000e-01, 2.58000502e-01, 5.08696327e-02, -1.38777878e-17]
            }
        }
    }
    
    mod helpers {
        pub fn assert_with_decimal_places(actual: Vec<f32>, sample: Vec<f32>) {
        
            actual.iter().zip(sample).for_each(|(a, b)| {
                let y = format!("{:.05}", *a);
                let z = format!("{:.05}", b);
                assert_eq!(y,z);
            });

        }
    }
    
    #[test]
    fn test_hanning_window() {
        let window = Hanning::window(10);
        let test_data = Hanning::sample_data();
        assert_with_decimal_places(window, test_data);
    }

    #[test]
    fn test_hamming_window() {
        let window = Hamming::window(10);
        let test_data = Hamming::sample_data();
        assert_with_decimal_places(window, test_data);
    }

    #[test]
    fn test_barlet() {
        let window = Bartlett::window(10);
        let test_data = Bartlett::sample_data();
        assert_with_decimal_places(window, test_data);
    }

    #[test]
    fn test_blackman() {
        let window = Blackman::window(10);
        let test_data = Blackman::sample_data();
        assert_with_decimal_places(window, test_data);   
    }
}
