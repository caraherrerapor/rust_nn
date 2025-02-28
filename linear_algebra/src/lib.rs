// src/lib.rs

// Paso 2: Estructuras de Datos Básicas
#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize> {
    data: [f64; N],
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize> {
    data: [[f64; N]; M],
}

// Paso 3: Implementación de Operaciones Básicas
impl<const N: usize> Vector<N> {
    pub fn new(data: [f64; N]) -> Self {
        Vector { data }
    }

    pub fn add(&self, other: &Vector<N>) -> Vector<N> {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = self.data[i] + other.data[i];
        }
        Vector::new(result)
    }
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new(data: [[f64; N]; M]) -> Self {
        Matrix { data }
    }

    // Método para multiplicar matrices
    pub fn multiply<const P: usize>(&self, other: &Matrix<N, P>) -> Matrix<M, P> {
        let mut result = [[0.0; P]; M];
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Matrix::new(result)
    }
}

// Paso 4: Pruebas Unitarias
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_addition() {
        let v1 = Vector::new([1.0, 2.0, 3.0]);
        let v2 = Vector::new([4.0, 5.0, 6.0]);
        let result = v1.add(&v2);
        assert_eq!(result.data, [5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_matrix_multiplication() {
        let m1 = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        let m2 = Matrix::new([[2.0, 0.0], [1.0, 2.0]]);
        let result = m1.multiply(&m2);
        assert_eq!(result.data, [[4.0, 4.0], [10.0, 8.0]]);
    }
}