use std::ops::Add;

#[inline]
pub fn new_grid<T: Default + Clone>(ni: usize, nj: usize) -> Vec<Vec<T>> {
    (0..ni).map(|_| {
        vec![T::default(); nj]
    }).collect()
}

#[derive(Default, Debug)]
pub struct GridNeighbours<T: Default + Copy> {
    pub n: Option<T>,
    pub ne: Option<T>,
    pub e: Option<T>,
    pub se: Option<T>,
    pub s: Option<T>,
    pub sw: Option<T>,
    pub w: Option<T>,
    pub nw: Option<T>,
}

impl<T: Default + Copy> GridNeighbours<T> {
    pub fn get_neighbours_for_coord(grid: &Vec<Vec<T>>, i: usize, j: usize) -> Self {
        let mut neighbours = GridNeighbours::<T>::default();
        let ni = grid.len();
        let nj = grid[0].len();

        neighbours.w = if j > 0 { Some(grid[i][j - 1]) } else { None };
        neighbours.e = if j < nj - 1 { Some(grid[i][j + 1]) } else { None };
        if i > 0 { 
            neighbours.nw = if j > 0 { Some(grid[i - 1][j - 1]) } else { None };
            neighbours.n = Some(grid[i - 1][j]);
            neighbours.ne = if j < nj - 1 { Some(grid[i - 1][j + 1]) } else { None };
        }
        if i < ni - 1 { 
            neighbours.sw = if j > 0 { Some(grid[i + 1][j - 1]) } else { None };
            neighbours.s = Some(grid[i + 1][j]);
            neighbours.se = if j < nj - 1 { Some(grid[i + 1][j + 1]) } else { None };
        }
        neighbours
    }

    pub fn into_vec(self) -> Vec<Option<T>> {
        vec![self.n, self.ne, self.e, self.se, self.s, self.sw, self.w, self.nw]
    }
}

impl<T: Default + Copy> Into<Vec<Option<T>>> for GridNeighbours<T> {
    /// Returns the elements of the neighbouring cells in a vec.
    /// The vec contains the elements moving clockwise from north. 
    fn into(self) -> Vec<Option<T>> {
        self.into_vec()
    }
}

pub fn find_coords_for<T: Eq + PartialEq>(haystack: &Vec<Vec<T>>, needle: T) -> Vec<(usize, usize)> {
    haystack.iter().enumerate().map(|(irow, row)| {
        let mut start = 0;
        let mut indices = Vec::new();
        while let Some(i) = row[start..].iter().position(|val| *val == needle) {
            indices.push((irow, start + i));
            start += i + 1;
        }
        indices
    }).flatten().collect()
}

/// Transposes the vec of Vecs.
/// Thanks to https://stackoverflow.com/a/64499219. 
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn rotate_clockwise_90<T: Default + Copy>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rotated = transpose(grid.clone());
    rotated.iter().map(|row| row.iter().rev().cloned().collect()).collect()
}

pub fn add_grids<T>(grid1: Vec<Vec<T>>, grid2: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Add<Output = T> + Clone {
    grid1
        .iter()
        .zip(grid2)
        .map(|(row1, row2)| {
            row1.iter().zip(row2).map(|(val1, val2)|{ val1.clone() + val2 }).collect::<Vec<T>>()
        }).collect()
}