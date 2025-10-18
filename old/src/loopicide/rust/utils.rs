struct Stack<T> {
    elements: Vec<T>,
}

struct Point {
    x: f64,
    y: f64,
}

struct PointList {
    points: Vec<Point>,
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