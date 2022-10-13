pub fn rotate_right<T, const LEN: usize>(input: [[T; LEN]; LEN]) -> [[T; LEN]; LEN]
where
    T: Default + Copy,
{
    let mut output = [[T::default(); LEN]; LEN];

    for (y, row) in input.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            let (x, y) = ((y as i32 - LEN as i32).abs() as usize, x);

            output[y][x] = cell;
        }
    }

    output
}

pub fn rotate_left<T, const LEN: usize>(input: [[T; LEN]; LEN]) -> [[T; LEN]; LEN]
where
    T: Default + Copy,
{
    let mut output = [[T::default(); LEN]; LEN];

    for (y, row) in input.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            let (x, y) = (y, (x as i32 - LEN as i32).abs() as usize);

            output[y][x] = cell;
        }
    }

    output
}
