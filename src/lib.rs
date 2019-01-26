use std::ops::{Index, Add, AddAssign, Mul};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Matrix<T> {
    m : Vec<Vec<T>>,
    rows : usize,
    cols : usize,
}

impl<T : Clone+Default> Matrix<T> {
    pub fn new(rows : usize, cols : usize) -> Self {
        Matrix::<T> {
            m : vec![vec![Default::default() ; cols] ; rows ],
            rows,
            cols,
        }
    }
    
    pub fn transpose(&mut self) {
        // create a new matrix in memory
        let mut tmp = Vec::new();
        for i in 0..self.cols {
            let mut row = Vec::new();
            for j in 0..self.rows {
                row.push(self.m[j][i].clone());
            }
            tmp.push(row);
        }

        
        self.m = tmp;
        // swap row <-> column count
        let c = self.cols;
        self.cols = self.rows;
        self.rows = c;
    }

    // Makes a <copy> of a range of rows of a matrix
    pub fn slice<S>(&self, range : S) -> Matrix<T>
        where S : IntoIterator<Item=usize> {
        let mut tmp = Vec::new();
        for i in range {
            tmp.push(self.m[i].clone());
        }
        Matrix::from(tmp)
    }
}

impl<T : Clone+Default> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(other: Vec<Vec<T>>) -> Self {
        if other.len() == 0 {
            return Matrix::new(0,0);
        } else {
            let mut matrix = Matrix::new(other.len(),other[0].len());
            matrix.m = other;
            return matrix;
        }
    }
}

impl<'a, 'b, T : AddAssign+Clone> Add<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: &'b Matrix<T>) -> Matrix<T> {
        // can only add matrices of the same size
        assert!(self.rows == other.rows);
        assert!(self.cols == other.cols);

        // copy our source matrix
        let mut matrix : Matrix<T> = self.clone();

        for i in 0..self.rows {
            for j in 0..self.cols {
                matrix.m[i][j] += other.m[i][j].clone();
            }
        }

        matrix
    }
}

impl<T: AddAssign+Clone> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(mut self, other : Matrix<T>) -> Matrix<T> {
        // can only add matrices of the same size
        assert!(self.rows == other.rows);
        assert!(self.cols == other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.m[i][j] += other.m[i][j].clone();
            }
        }

        self
    }
}

impl<'a, 'b, T: AddAssign+Clone+Default+Mul<Output=T>> Mul<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &'b Matrix<T>) -> Matrix<T> {
        // can only multiply if LHS.cols == RHS.rows
        assert!(self.cols == rhs.rows);

        let mut matrix = Matrix::new(self.rows, rhs.cols);

        // TODO - research "better" matrix multiplication algos
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut entry = Default::default();
                for k in 0..self.cols {
                    entry += self.m[i][k].clone() * rhs.m[k][j].clone();
                }
                matrix.m[i][j] = entry;
            }
        }

        matrix
    }
}

impl<T: AddAssign+Clone+Default+Mul<Output=T>> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // just use the above implementation for now
        (&self) * (&rhs)
    }
}

impl<T> Into<Vec<Vec<T>>> for Matrix<T> {
    fn into(self) -> Vec<Vec<T>> {
        self.m
    }
}

impl<T> Index<(usize,usize)> for Matrix<T> {
    type Output = T;
    
    fn index(&self, ix : (usize, usize)) -> &T {
        &self.m[ix.0][ix.1]    
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn basic_matrix_transpose() {
        let mut m = Matrix::from(vec![vec![3,5,1],vec![2,9,-1],vec![3,-1,-2]]);
        m.transpose();

        let n = Matrix::from(vec![vec![3,2,3], vec![5,9,-1], vec![1,-1,-2]]);

        assert_eq!(m, n);
    }

    #[test]
    fn basic_matrix_add() {
        let m = Matrix::from(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]);
        let n = Matrix::from(vec![vec![-1,-2,-3],vec![4,-5,-6],vec![-7,0,-9]]);
        let r = Matrix::from(vec![vec![0,0,0], vec![8,0,0], vec![0, 8, 0]]);

        assert_eq!(m+n, r);
    }

    #[test]
    fn basic_matrix_multiply() {
        let m = Matrix::from(vec![vec![2, 1], vec![-1, 1]]);
        let n = Matrix::from(vec![vec![-1,3], vec![2, 2]]);
        let r = Matrix::from(vec![vec![0,8], vec![3,-1]]);

        assert_eq!(m*n, r);
    }

    #[test]
    fn basic_matrix_index_access() {
        let m = Matrix::from(vec![vec![3,5,9],vec![2,2,7],vec![3,5,5]]);

        assert_eq!(m[(1,2)],7);
        assert_eq!(m[(0,1)],5);
    }

    #[test]
    fn basic_matrix_row_slice() {
        let m = Matrix::from(vec![vec![1,1,2,2],vec![3,3,4,4],vec![5,5,6,6],vec![7,7,9,10]]);
        let n = m.slice(1..=2);
        let p = Matrix::from(vec![vec![3,3,4,4],vec![5,5,6,6]]);

        assert_eq!(n, p);
    }
}
