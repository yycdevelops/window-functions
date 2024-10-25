pub trait Window<T> {
    fn window(num: usize) -> T;
}

trait CosineWindow<T> {
    fn cosine_window(window: &mut Vec<T>);
}

// Macro for filling vector 
#[macro_export]
macro_rules! fill_vec{
    ( $($x: expr),* ) => {
        {
            $(  
                (0u8..$x as u8).map(f32::from).collect()
            )*
        }
    };
}

#[macro_export]
macro_rules! assert_with_decimal_places {
    ($mand_1:expr, $mand_2:expr) => {
        $mand_1.iter().zip($mand_2).for_each(|(a, b)| {
            let x = format!("{:.05}", a);
            let y = format!("{:.05}", b);
            assert_eq!(x, y);
        });
    };
}


mod hanning;
mod hamming;
mod bartlett;
mod blackman;

