pub struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn from(data: Vec<T>) -> Self {
        Stack { top: 0, data }
    }

    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    // return None when stack is empty
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }

    // return None when stack is empty
    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}
