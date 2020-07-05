pub struct Grid([[u32; 20]; 20]);

pub const GRID: Grid = Grid {
    0: [
        [
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        [
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        [
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        [
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        [
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        [
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        [
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        [
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        [
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        [
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        [
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        [
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        [
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        [
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        [
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        [
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        [
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        [
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        [
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        [
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ],
};

pub struct RowIterator<'a> {
    grid: &'a Grid,
    n_row: usize,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Vec<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_row > 19 {
            None
        } else {
            let result = self.grid.row(self.n_row);
            self.n_row = self.n_row + 1;
            Some(result)
        }
    }
}

pub struct ColumnIterator<'a> {
    grid: &'a Grid,
    n_col: usize,
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n_col > 19 {
            None
        } else {
            let result = self.grid.col(self.n_col);
            self.n_col = self.n_col + 1;
            Some(result)
        }
    }
}

pub struct DiagonalIterator<'a> {
    grid: &'a Grid,
    n_diag: usize,
}

impl<'a> Iterator for DiagonalIterator<'a> {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n_diag > 38 * 2 {
            None
        } else if self.n_diag > 38 {
            let result = self.grid.left_diagonal(self.n_diag - 38);
            self.n_diag = self.n_diag + 1;
            Some(result)
        } else {
            let result = self.grid.right_diagonal(self.n_diag);
            self.n_diag = self.n_diag + 1;
            Some(result)
        }
    }
}

impl Grid {
    pub fn row(&self, row: usize) -> Vec<u32> {
        self.0[row].iter().cloned().collect()
    }

    pub fn rows(&self) -> RowIterator {
        RowIterator {
            grid: self,
            n_row: 0,
        }
    }

    pub fn col(&self, col: usize) -> Vec<u32> {
        self.0.iter().map(|r| r[col]).collect()
    }

    pub fn cols(&self) -> ColumnIterator {
        ColumnIterator {
            grid: self,
            n_col: 0,
        }
    }

    pub fn diags(&self) -> DiagonalIterator {
        DiagonalIterator {
            grid: self,
            n_diag: 0,
        }
    }

    pub fn right_diagonal(&self, start: usize) -> Vec<u32> {
        if start < 20 {
            self.right_diag_from_left(start)
        } else {
            self.right_diag_from_top(start)
        }
    }

    pub fn left_diagonal(&self, start: usize) -> Vec<u32> {
        if start < 20 {
            self.left_diag_from_left(start)
        } else {
            self.left_diag_from_bot(start)
        }
    }

    fn right_diag_from_left(&self, start: usize) -> Vec<u32> {
        let mut c = 0;
        let mut result = Vec::new();

        for r in (19 - start)..20 {
            result.push(self.0[r][c]);
            c = c + 1;
        }

        result
    }

    fn right_diag_from_top(&self, start: usize) -> Vec<u32> {
        let mut r = 0;
        let mut result = Vec::new();
        for c in (start - 19)..20 {
            result.push(self.0[r][c]);
            r = r + 1;
        }

        result
    }

    fn left_diag_from_left(&self, start: usize) -> Vec<u32> {
        let mut result = Vec::new();
        let mut c = 0;

        let mut r = start;
        loop {
            result.push(self.0[r][c]);
            if r == 0 {
                break;
            }
            r = r - 1;
            c = c + 1;
        }

        result
    }

    fn left_diag_from_bot(&self, start: usize) -> Vec<u32> {
        let mut r = 19;
        let mut result = Vec::new();

        for c in (start - 19)..20 {
            result.push(self.0[r][c]);
            r = r - 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_diag() {
        let d1 = GRID.right_diagonal(1);
        assert_eq!(d1.len(), 2);
        assert_eq!(d1, vec![20, 70]);

        let d2 = GRID.right_diagonal(36);
        assert_eq!(d2, vec![77, 62, 65]);
    }

    #[test]
    fn test_left_diag() {
        let d1 = GRID.left_diagonal(2);
        assert_eq!(d1, vec![81, 49, 22]);

        let d2 = GRID.left_diagonal(36);
        assert_eq!(d2, vec![19, 5, 16]);
    }

    #[test]
    fn test_column() {
        let c = GRID.col(4);
        assert_eq!(
            c,
            vec![38, 17, 55, 4, 51, 99, 64, 2, 66, 75, 22, 96, 35, 5, 97, 57, 38, 72, 78, 83]
        );
    }
}
