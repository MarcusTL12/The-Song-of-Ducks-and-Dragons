use ndarray::{s, ArrayView2, ArrayViewMut2};

pub fn _input_to_grid(input: &[u8]) -> ArrayView2<'_, u8> {
    let w = input
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == b'\n')
        .map(|(i, _)| i)
        .unwrap();

    let h = input.len() / (w + 1);

    ArrayView2::from_shape([h, w + 1], input)
        .unwrap()
        .slice_move(s![.., 0..w])
}

pub fn input_to_grid_mut(input: &mut [u8]) -> ArrayViewMut2<'_, u8> {
    let w = input
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == b'\n')
        .map(|(i, _)| i)
        .unwrap();

    let h = input.len() / (w + 1);

    ArrayViewMut2::from_shape([h, w + 1], input)
        .unwrap()
        .slice_move(s![.., 0..w])
}
