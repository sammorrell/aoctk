use std::slice::SliceIndex;

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