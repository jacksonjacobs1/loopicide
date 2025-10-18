use py03::prelude::*;
use crate::utils::PointList;

#[pyfunction]
fn remove_loops() -> () {
    println!("Removing loops from PointList with {} points.", points.points.len());
}