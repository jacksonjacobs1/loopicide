use pyo3::prelude::*;
mod utils;
use utils::Point;
use utils::PointList;
use utils::Stack;



#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
pub fn remove_loops(points: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {

    for first_segment_index in 0..points.len() {
        for second_segment_index in 0..points.len() {

            // Skip checking the same segment
            if second_segment_index == first_segment_index {
                continue;
            }

            
        }
    }
    
    Ok(points)
}

