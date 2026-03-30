use faer::mat::MatRef;
use faer::Mat;
use numpy::{PyArray2, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use ::rust_sysid as sysid;

// ============================================================================
// Type aliases
// ============================================================================

type DoublePyArray2<'py> = Bound<'py, PyArray2<f64>>;

// ============================================================================
// Helper functions
// ============================================================================

/// Convert a NumPy 2D array (row-major) to a faer Mat (column-major)
fn ndarray_to_faer(arr: &PyReadonlyArray2<f64>) -> Mat<f64> {
    let arr = arr.as_array();
    let (rows, cols) = arr.dim();
    Mat::from_fn(rows, cols, |i, j| arr[[i, j]])
}

/// Convert a faer Mat to a NumPy 2D array
fn faer_to_pyarray<'py>(py: Python<'py>, mat: &Mat<f64>) -> DoublePyArray2<'py> {
    let rows = mat.nrows();
    let cols = mat.ncols();
    let arr = numpy::ndarray::Array2::from_shape_fn((rows, cols), |(i, j)| mat[(i, j)]);
    PyArray2::from_owned_array(py, arr)
}

/// Validate that matrix dimensions are compatible for multiplication
fn validate_matmul_dims(a_cols: usize, b_rows: usize) -> PyResult<()> {
    if a_cols != b_rows {
        return Err(PyValueError::new_err(format!(
            "Incompatible matrix dimensions: A has {} columns but B has {} rows",
            a_cols, b_rows
        )));
    }
    Ok(())
}


// ============================================================================
// Python functions
// ============================================================================

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    sysid::add(a, b)
}

#[pyfunction]
#[pyo3(name = "matmul")]
fn matmul_py<'py>(
    py: Python<'py>,
    a: PyReadonlyArray2<'py, f64>,
    b: PyReadonlyArray2<'py, f64>,
) -> PyResult<DoublePyArray2<'py>> {
    // 1. Validate dimensions
    let a_shape = a.shape();
    let b_shape = b.shape();
    validate_matmul_dims(a_shape[1], b_shape[0])?;

    // 2. Convert to faer matrices
    let a_faer = ndarray_to_faer(&a);
    let b_faer = ndarray_to_faer(&b);

    // 3. Perform multiplication
    let result = sysid::matmul(&a_faer, &b_faer);

    // 4. Convert back to numpy
    Ok(faer_to_pyarray(py, &result))
}

/// Fast matrix multiplication using zero-copy views (requires Fortran-contiguous arrays)
#[pyfunction]
#[pyo3(name = "matmul_fast")]
fn matmul_fast_py<'py>(
    py: Python<'py>,
    a: PyReadonlyArray2<'py, f64>,
    b: PyReadonlyArray2<'py, f64>,
) -> PyResult<DoublePyArray2<'py>> {
    // 1. Get array views (must live for duration of function)
    let a_view = a.as_array();
    let b_view = b.as_array();

    // 2. Validate Fortran contiguity
    if a_view.is_standard_layout() {
        return Err(PyValueError::new_err(
            "Array 'a' must be Fortran-contiguous (column-major). Use np.asfortranarray(a) to convert."
        ));
    }
    if b_view.is_standard_layout() {
        return Err(PyValueError::new_err(
            "Array 'b' must be Fortran-contiguous (column-major). Use np.asfortranarray(b) to convert."
        ));
    }

    // 3. Validate dimensions
    let (a_rows, a_cols) = a_view.dim();
    let (b_rows, b_cols) = b_view.dim();
    validate_matmul_dims(a_cols, b_rows)?;

    // 4. Get slices and create zero-copy MatRefs
    let a_slice = a_view.as_slice_memory_order()
        .ok_or_else(|| PyValueError::new_err("Array 'a' is not contiguous in memory"))?;
    let b_slice = b_view.as_slice_memory_order()
        .ok_or_else(|| PyValueError::new_err("Array 'b' is not contiguous in memory"))?;

    let a_ref = MatRef::from_column_major_slice(a_slice, a_rows, a_cols);
    let b_ref = MatRef::from_column_major_slice(b_slice, b_rows, b_cols);

    // 5. Perform multiplication
    let result = sysid::matmul_ref(a_ref, b_ref);

    // 6. Convert back to numpy
    Ok(faer_to_pyarray(py, &result))
}

// ============================================================================
// Module registration
// ============================================================================

#[pymodule]
fn rust_sysid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(matmul_py, m)?)?;
    m.add_function(wrap_pyfunction!(matmul_fast_py, m)?)?;
    Ok(())
}
