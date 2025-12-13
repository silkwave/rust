use std::fmt;
use std::ops::{Add, Mul, Sub};

/// 행렬을 나타내는 구조체입니다.
#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,         // 행의 수
    cols: usize,         // 열의 수
    data: Vec<Vec<f64>>, // 행렬 데이터 (2차원 벡터)
}

/// MatrixIterator는 Matrix의 요소들을 순회하는 이터레이터입니다.
pub struct MatrixIterator<'a> {
    matrix: &'a Matrix,
    current_row: usize, // 현재 행 인덱스
    current_col: usize, // 현재 열 인덱스
}

// MatrixIterator에 대한 Iterator 트레잇 구현
impl<'a> Iterator for MatrixIterator<'a> {
    type Item = f64; // 이터레이터가 반환하는 요소의 타입

    // 다음 요소를 반환하는 메서드
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row < self.matrix.rows {
            if self.current_col < self.matrix.cols {
                let value = self.matrix.data[self.current_row][self.current_col];
                self.current_col += 1;
                Some(value)
            } else {
                // 현재 행의 끝에 도달하면 다음 행으로 이동
                self.current_row += 1;
                self.current_col = 0;
                self.next() // 다음 행의 첫 번째 요소부터 다시 시작
            }
        } else {
            None // 모든 요소를 순회했으면 None을 반환
        }
    }
}

// &Matrix에 대한 IntoIterator 트레잇 구현
impl<'a> IntoIterator for &'a Matrix {
    type Item = f64;
    type IntoIter = MatrixIterator<'a>;

    // 이터레이터를 생성하는 메서드
    fn into_iter(self) -> Self::IntoIter {
        MatrixIterator {
            matrix: self,
            current_row: 0,
            current_col: 0,
        }
    }
}

impl Matrix {
    /// 모든 요소가 0.0인 새 행렬을 생성합니다.
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];
        Self { rows, cols, data }
    }

    /// 주어진 2차원 벡터로부터 행렬을 생성합니다.
    /// 모든 행의 길이가 동일한지 검사합니다.
    pub fn from_vec(data: Vec<Vec<f64>>) -> Self {
        assert!(!data.is_empty(), "데이터는 비어있으면 안됩니다.");
        let cols = data[0].len();
        assert!(cols > 0, "열의 개수는 0보다 커야 합니다.");
        for r in &data {
            assert!(r.len() == cols, "모든 행의 길이는 동일해야 합니다.");
        }
        let rows = data.len();
        Self { rows, cols, data }
    }

    /// n x n 크기의 단위 행렬을 생성합니다.
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.data[i][i] = 1.0;
        }
        m
    }

    /// 행렬의 크기(행, 열)를 튜플로 반환합니다.
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// 지정된 위치(r, c)의 요소를 반환합니다.
    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r][c]
    }

    /// 지정된 위치(r, c)의 요소를 설정합니다.
    pub fn set(&mut self, r: usize, c: usize, v: f64) {
        self.data[r][c] = v;
    }

    /// 전치 행렬을 반환합니다.
    pub fn transpose(&self) -> Self {
        let mut t = Self::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                t.data[j][i] = self.data[i][j];
            }
        }
        t
    }

    /// 행렬에 스칼라 값을 곱합니다.
    pub fn scalar_mul(&self, k: f64) -> Self {
        let mut out = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                out.data[i][j] *= k;
            }
        }
        out
    }

    /// 두 행렬의 곱을 계산합니다 (self * other).
    pub fn mat_mul(&self, other: &Matrix) -> Self {
        assert!(
            self.cols == other.rows,
            "행렬 곱셈에 호환되지 않는 크기입니다."
        );
        let mut out = Self::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut s = 0.0;
                for k in 0..self.cols {
                    s += self.data[i][k] * other.data[k][j];
                }
                out.data[i][j] = s;
            }
        }
        out
    }

    /// 두 행렬의 덧셈을 계산합니다.
    pub fn add(&self, other: &Matrix) -> Self {
        assert!(
            self.rows == other.rows && self.cols == other.cols,
            "행렬 덧셈을 위해 크기가 같아야 합니다."
        );
        let mut out = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                out.data[i][j] += other.data[i][j];
            }
        }
        out
    }

    /// 두 행렬의 뺄셈을 계산합니다.
    pub fn sub(&self, other: &Matrix) -> Self {
        assert!(
            self.rows == other.rows && self.cols == other.cols,
            "행렬 뺄셈을 위해 크기가 같아야 합니다."
        );
        let mut out = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                out.data[i][j] -= other.data[i][j];
            }
        }
        out
    }

    /// 가우스 소거법(부분 피벗)을 사용하여 행렬식을 계산합니다.
    pub fn determinant(&self) -> f64 {
        assert!(
            self.rows == self.cols,
            "행렬식은 정사각 행렬에 대해서만 정의됩니다."
        );
        let n = self.rows;
        let mut a = self.data.clone();
        let mut det = 1.0;
        let mut swaps = 0; // 행 교환 횟수

        for i in 0..n {
            // 부분 피벗: i 열에서 절대값이 가장 큰 요소를 가진 행을 찾습니다.
            let mut max_row = i;
            let mut max_val = a[i][i].abs();
            for r in (i + 1)..n {
                if a[r][i].abs() > max_val {
                    max_val = a[r][i].abs();
                    max_row = r;
                }
            }

            // 피벗이 0에 가까우면 특이 행렬(singular matrix)입니다.
            if max_val < 1e-12 {
                return 0.0;
            }

            // 필요하면 행을 교환합니다.
            if max_row != i {
                a.swap(i, max_row);
                swaps += 1;
            }

            // 현재 행을 기준으로 아래 행들을 소거합니다.
            let pivot = a[i][i];
            for r in (i + 1)..n {
                let factor = a[r][i] / pivot;
                for c in i..n {
                    a[r][c] -= factor * a[i][c];
                }
            }
        }

        // 대각 요소들의 곱으로 행렬식을 계산합니다.
        for i in 0..n {
            det *= a[i][i];
        }
        // 행 교환 횟수가 홀수이면 부호를 바꿉니다.
        if swaps % 2 == 1 {
            det = -det;
        }
        det
    }

    /// 가우스-조르당 소거법을 사용하여 역행렬을 계산합니다.
    pub fn inverse(&self) -> Option<Self> {
        assert!(
            self.rows == self.cols,
            "역행렬은 정사각 행렬에 대해서만 정의됩니다."
        );
        let n = self.rows;
        let mut a = vec![vec![0.0; 2 * n]; n];
        // 첨가 행렬 [A | I] 생성
        for i in 0..n {
            for j in 0..n {
                a[i][j] = self.data[i][j];
            }
            a[i][n + i] = 1.0;
        }

        // 가우스-조르당 소거법 (부분 피벗 사용)
        for i in 0..n {
            // 피벗 찾기
            let mut max_row = i;
            let mut max_val = a[i][i].abs();
            for r in (i + 1)..n {
                if a[r][i].abs() > max_val {
                    max_val = a[r][i].abs();
                    max_row = r;
                }
            }
            // 특이 행렬이면 역행렬이 존재하지 않습니다.
            if max_val < 1e-12 {
                return None;
            }
            if max_row != i {
                a.swap(i, max_row);
            }

            // 피벗 행을 정규화합니다. (피벗 요소를 1로 만듦)
            let piv = a[i][i];
            for c in 0..2 * n {
                a[i][c] /= piv;
            }

            // 다른 행들을 소거합니다.
            for r in 0..n {
                if r != i {
                    let factor = a[r][i];
                    if factor.abs() > 0.0 {
                        for c in 0..2 * n {
                            a[r][c] -= factor * a[i][c];
                        }
                    }
                }
            }
        }

        // 오른쪽 절반(원래 단위 행렬 부분)에서 역행렬을 추출합니다.
        let mut inv = Self::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                inv.data[i][j] = a[i][n + j];
            }
        }
        Some(inv)
    }
}

/// 행렬을 보기 좋게 출력하기 위한 Display 트레잇 구현
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            writeln!(f, "{:?}", self.data[r])?;
        }
        Ok(())
    }
}

/// `+` 연산자 오버로딩 (Matrix + Matrix)
impl Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {
        Matrix::add(&self, &rhs)
    }
}

/// `+` 연산자 오버로딩 (Matrix + &Matrix)
impl Add<&Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Matrix {
        Matrix::add(&self, rhs)
    }
}

/// `-` 연산자 오버로딩 (Matrix - Matrix)
impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix {
        Matrix::sub(&self, &rhs)
    }
}

/// `-` 연산자 오버로딩 (Matrix - &Matrix)
impl Sub<&Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: &Matrix) -> Matrix {
        Matrix::sub(&self, rhs)
    }
}

/// `*` 연산자 오버로딩 (Matrix * Matrix) -> 행렬 곱
impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        self.mat_mul(&rhs)
    }
}

/// `*` 연산자 오버로딩 (Matrix * &Matrix) -> 행렬 곱
impl Mul<&Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        self.mat_mul(rhs)
    }
}

fn main() {
    // 예시 사용법
    let a = Matrix::from_vec(vec![
        vec![1.0, 2.0, 3.0],
        vec![0.0, 1.0, 4.0],
        vec![5.0, 6.0, 0.0],
    ]);

    let b = Matrix::from_vec(vec![
        vec![-1.0, 3.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![0.0, 1.0, 0.0],
    ]);

    println!("A =\n{}", &a);
    println!("B =\n{}", &b);

    println!("A + B =\n{}", a.clone() + &b);
    println!("A * B =\n{}", a.clone() * &b);
    println!("A^T =\n{}", a.transpose());

    let det_a = a.determinant();
    println!("det(A) = {}", det_a);

    match a.inverse() {
        Some(inv) => {
            println!("A^-1 =\n{}", &inv);
            // 확인: A * A^-1 이 단위 행렬(I)에 가까운지 확인합니다.
            println!("A * A^-1 =\n{}", a.mat_mul(&inv));
        }
        None => println!("A는 특이 행렬이므로 역행렬이 없습니다."),
    }

    println!("\nA 행렬의 모든 요소 순회:");
    for element in &a {
        print!("{} ", element);
    }
    println!();
}
