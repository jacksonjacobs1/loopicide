//! Utilities for curve/loop detection.
//! 
//! This module provides:
//! - A small generic Stack<T> helper for LIFO operations used during loop detection.
//! - Curve: a lightweight polyline representation (Vec<[x,y]>) with helpers to obtain
//!   line segments and test for segment intersections.
//! - Loop: a small struct that records start/end segment indices for detected loops.

use geo::{coord, Intersects, Line};
use geo::line_intersection::line_intersection;

/// A simple generic LIFO stack used by the loop detection algorithm.
///
/// This is a thin wrapper around Vec<T> with push/pop/is_empty helpers.
pub struct Stack<T> {
    elements: Vec<T>,
}

/// Represents a detected loop in a curve, by storing the indices of the
/// starting and ending segments that bound the loop.
///
/// Both indices are segment indices (usize) into the Curve's segments.
pub struct Loop {
    loopstart: usize,
    loopend: usize,
}

impl Loop {
    /// Create a new Loop covering segments in the half-open range
    /// [loopstart, loopend].
    pub fn new(loopstart: usize, loopend: usize) -> Self {
        Loop { loopstart, loopend }
    }

    /// Returns the segment index where the loop starts.
    pub fn get_loopstart(&self) -> usize {
        self.loopstart
    }

    /// Returns the segment index where the loop ends.
    pub fn get_loopend(&self) -> usize {
        // fixed to return the stored loopend value
        self.loopend
    }
}

/// A polyline curve stored as a sequence of 2D points: Vec<[x, y]>.
///
/// The Curve API assumes each inner Vec<f64> has at least two elements:
/// [x, y]. Methods that access index+1 will panic if out of bounds.
pub struct Curve {
    points: Vec<Vec<f64>>,
}

impl Curve {
    /// Create a new Curve from a list of [x, y] points.
    pub fn new(points: Vec<Vec<f64>>) -> Self {
        Curve { points }
    }

    /// Number of points in the curve.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Return the Line segment at the given index, i.e., the segment
    /// between points[index] and points[index + 1].
    ///
    /// # Panics
    /// Panics if index + 1 is out of bounds for the internal points vector.
    pub fn get_line_segment(&self, index: usize) -> Line<f64> {
        let startcoord = coord! {
            x: self.points[index][0],
            y: self.points[index][1],
        };

        let endcoord = coord! {
            x: self.points[index + 1][0],
            y: self.points[index + 1][1],
        };

        Line::new(startcoord, endcoord)
    }

    /// Returns true if the two segments at seg1index and seg2index intersect.
    ///
    /// Uses geo::line_intersection under the hood; returns true when an intersection
    /// point exists.
    pub fn segments_intersect(&self, seg1index: usize, seg2index: usize) -> bool {
        // Placeholder for actual intersection logic
        line_intersection(self.get_line_segment(seg1index), self.get_line_segment(seg2index)).is_some()
    }

    /// Attempt to detect loops by checking segment intersections.
    ///
    /// The algorithm uses a Stack to record candidate loop starts and produces
    /// a Vec<Loop> of detected loops. This function traverses pairs of segments
    /// and records a loop when an intersection that completes a loop is found.
    pub fn get_loops(&self) -> Vec<Loop> {
        let mut stack = Stack::<usize>::new();
        let mut loops = Vec::<Loop>::new();
        
        for first_segment_index in 0..self.len() {
            for second_segment_index in 0..self.len() {

                // Skip checking the same segment
                if second_segment_index == first_segment_index {
                    continue;
                }

                if self.segments_intersect(first_segment_index, second_segment_index) {
                    if second_segment_index > first_segment_index + 1 {
                        stack.push(first_segment_index)
                    } else if second_segment_index < first_segment_index - 1 {
                        let loopstart = stack.pop();

                        if stack.is_empty() {
                            loops.push(Loop::)
                        }
                    }
                }

            }
        }

        loops
    }
}

impl<T> Stack<T> {
    /// Create an empty Stack.
    pub fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    /// Push an item onto the stack.
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    /// Pop an item from the stack, returning None if empty.
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Returns true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointlist_new_and_len() {
        let pts = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let pl = Curve::new(pts);
        assert_eq!(pl.len(), 2);
    }

    #[test]
    fn get_line_segment_returns_correct_line() {
        let pts = vec![vec![0.0, 0.0], vec![2.0, 0.0], vec![2.0, 2.0]];
        let pl = Curve::new(pts);

        let line = pl.get_line_segment(0);
        let start = line.start;
        let end = line.end;

        assert_eq!(start.x, 0.0);
        assert_eq!(start.y, 0.0);
        assert_eq!(end.x, 2.0);
        assert_eq!(end.y, 0.0);
    }

    #[test]
    fn segments_intersect_true_and_false() {
        // two segments that cross: (0,0)-(2,2) and (0,2)-(2,0)
        let pts = vec![
            vec![0.0, 0.0],
            vec![2.0, 2.0],
            vec![0.0, 2.0],
            vec![2.0, 0.0],
        ];
        let pl = Curve::new(pts);

        // seg0 is (0,0)-(2,2), seg2 is (0,2)-(2,0)
        assert!(pl.segments_intersect(0, 2));

        // two segments that do not intersect: (0,0)-(1,0) and (2,0)-(3,0)
        let pts2 = vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![2.0, 0.0], vec![3.0, 0.0]];
        let pl2 = Curve::new(pts2);
        assert!(!pl2.segments_intersect(0, 2));
    }

    #[test]
    #[should_panic]
    fn get_line_segment_out_of_bounds_panics() {
        let pts = vec![vec![0.0, 0.0]];
        let pl = Curve::new(pts);
        // index 0 requires index+1 == 1 which is out of bounds and should panic
        let _ = pl.get_line_segment(0);
    }

    #[test]
    fn stack_push_pop_is_empty() {
        let mut s: Stack<usize> = Stack::new();
        assert!(s.is_empty());
        s.push(10);
        assert!(!s.is_empty());
        assert_eq!(s.pop(), Some(10));
        assert!(s.is_empty());
    }
}
