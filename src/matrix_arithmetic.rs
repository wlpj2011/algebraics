use crate::traits::Field;

pub(crate) fn determinant<F: Field>(mut mat: Vec<Vec<F>>) -> F {
    let n = mat.len();
    let mut det = F::one();

    for i in 0..n {
        // Find pivot
        let mut pivot = i;
        while pivot < n && mat[pivot][i] == F::zero() {
            pivot += 1;
        }

        if pivot == n {
            return F::zero(); // singular matrix
        }

        // Swap rows if needed
        if pivot != i {
            mat.swap(i, pivot);
            det = -det;
        }

        let pivot_val = mat[i][i].clone();
        det = det * pivot_val.clone();

        // Normalize row and eliminate below
        for j in i + 1..n {
            let factor = mat[j][i].clone() * pivot_val.clone().inv().unwrap();
            for k in i..n {
                mat[j][k] = mat[j][k].clone() - factor.clone() * mat[i][k].clone();
            }
        }
    }

    det
}
