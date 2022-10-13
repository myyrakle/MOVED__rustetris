pub fn rotate_right<T, const LEN: usize>(input: &mut [[T; LEN]; LEN], length: usize)
where
    T: Default + Copy,
{
    let mut output = [[T::default(); LEN]; LEN];

    for (y, row) in input.into_iter().enumerate() {
        if y >= length {
            break;
        }

        for (x, cell) in row.into_iter().enumerate() {
            if x >= length {
                break;
            }

            let (x, y) = (length - 1 - y, x);

            output[y][x] = cell.to_owned();
        }
    }

    *input = output;
}
