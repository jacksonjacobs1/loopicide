use geo::{coord, Intersects, Line};
use geo::line_intersection::line_intersection;


pub struct Stack<T> {
    elements: Vec<T>,
}

pub struct PointList {
    points: Vec<Vec<f64>>,
}

impl PointList {
    pub fn new(points: Vec<Vec<f64>>) -> Self {
        PointList { points }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

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

    pub fn segments_intersect(&self, seg1index: usize, seg2index: usize) -> bool {
        // Placeholder for actual intersection logic
        line_intersection(self.get_line_segment(seg1index), self.get_line_segment(seg2index)).is_some()
    }
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointlist_new_and_len() {
        let pts = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let pl = PointList::new(pts);
        assert_eq!(pl.len(), 2);
    }

    #[test]
    fn get_line_segment_returns_correct_line() {
        let pts = vec![vec![0.0, 0.0], vec![2.0, 0.0], vec![2.0, 2.0]];
        let pl = PointList::new(pts);

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
        let pl = PointList::new(pts);

        // seg0 is (0,0)-(2,2), seg2 is (0,2)-(2,0)
        assert!(pl.segments_intersect(0, 2));

        // two segments that do not intersect: (0,0)-(1,0) and (2,0)-(3,0)
        let pts2 = vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![2.0, 0.0], vec![3.0, 0.0]];
        let pl2 = PointList::new(pts2);
        assert!(!pl2.segments_intersect(0, 2));
    }

    #[test]
    #[should_panic]
    fn get_line_segment_out_of_bounds_panics() {
        let pts = vec![vec![0.0, 0.0]];
        let pl = PointList::new(pts);
        // index 0 requires index+1 == 1 which is out of bounds and should panic
        let _ = pl.get_line_segment(0);
    }

    #[test]
    fn stack_push_pop_is_empty() {
        let mut s: Stack<i32> = Stack::new();
        assert!(s.is_empty());
        s.push(10);
        assert!(!s.is_empty());
        assert_eq!(s.pop(), Some(10));
        assert!(s.is_empty());
    }
}
