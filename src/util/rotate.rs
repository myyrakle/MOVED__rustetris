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

pub fn rotate_left<T, const LEN: usize>(input: &mut [[T; LEN]; LEN], length: usize)
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

            let (x, y) = (y, length - 1 - x);

            output[y][x] = cell.to_owned();
        }
    }

    *input = output;
}

//SRS TABLE: IF ROTATION FAILS, TRY THESE
/*
const kickIndexI = [
    [[-2, 0], [1, 0], [-2, -1], [1, 2]], //0R -> kickIndexI[0][i] for i in (0,1,2,3)
    [[-1, 0], [2, 0], [-1, 2], [2, -1]], //1R
    [[2, 0], [-1, 0], [2, 1], [-1, -2]], //2R
    [[1, 0], [-2, 0], [1, -2], [-2, 1]], //3R
    [[2, 0], [-1, 0], [2, 1], [-1, -2]], //0L
    [[1, 0], [-2, 0], [1, -2], [-2, 1]], //1L
    [[-2, 0], [1, 0], [-2, -1], [1, 2]], //2L
    [[-1, 0], [2, 0], [-1, 2], [2, -1]]  //3L
];

const kickIndexJLTSZ = [
    [[-1, 0], [-1, 1], [0, -2], [-1, -2]], //0R -> kickIndexJLTSZ[0][i] for i in (0,1,2,3)
    [[1, 0], [1, -1], [0, 2], [1, 2]],     //1R
    [[1, 0], [1, -1], [0, 2], [1, 2]],     //2R
    [[-1, 0], [-1, 1], [0, -2], [-1, -2]], //3R
    [[1, 0], [1, 1], [0, -2], [1, -2]],    //0L
    [[-1, 0], [-1, -1], [0, 2], [-1, 2]],  //1L
    [[-1, 0], [-1, -1], [0, 2], [-1, 2]],  //2L
    [[1, 0], [1, 1], [0, -2], [1, -2]]     //3L
];
*/