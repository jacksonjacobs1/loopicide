use geo::{Line, coord};


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
        self.get_line_segment(seg1index).(&self.get_line_segment(seg2index))
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
