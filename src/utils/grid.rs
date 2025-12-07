use std::fmt::{self, Debug};
// Fixed size grid
#[derive(Clone)]
pub struct Grid<T> {
    width: usize,  // Num of columns
    height: usize, // Num of rows
    // [0, 1, 2, 3, 4, 5, 6, 7, 8] =>
    //   0 1 2
    // 0 0 1 2
    // 1 3 4 5
    // 2 6 7 8
    grid: Vec<T>,
}
impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows()).finish()
    }
}
impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(grid: Vec<Vec<T>>) -> Self {
        Self::new_from(grid)
    }
}
impl<T> FromIterator<Vec<T>> for Grid<T> {
    // Iterator should be rows of columns
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut grid = Vec::with_capacity(iter.size_hint().0);
        let mut width = 0;
        let mut height = 0;
        for row in iter {
            // first iteration shouldn't check the width
            assert!(height == 0 || width == row.len(), "Grid input must be rectangular");
            width = row.len();
            grid.extend(row.into_iter());
            height += 1;
        }
        Self { width, height, grid }
    }
}

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(width * height);
        grid.resize_with(width * height, T::default);
        Self { width, height, grid }
    }
}
impl<T> Grid<T> {
    pub fn new_from(igrid: Vec<Vec<T>>) -> Self {
        igrid.into_iter().collect()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.grid[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.grid[y * self.width + x] = value;
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.grid.chunks(self.width)
    }
}
