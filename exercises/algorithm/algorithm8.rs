/*
queue
This question requires you to use queues to implement the functionality of the stac
使用队列（Queue）来实现栈（Stack）的功能。栈是一种后进先出（LIFO, Last In First Out）的数据结构，
而队列是一种先进先出（FIFO, First In First Out）的数据结构。为了使用队列来实现栈的功能，我们需要利用两个队列来模拟栈的行为
实现步骤:
1、初始化两个队列，queue1和queue2。
2、push操作时，将元素添加到queue1。
3、pop操作时，如果queue2为空，则将queue1的所有元素转移到queue2。然后从queue2弹出队首元素。
4、top操作类似，检查queue2是否为空，转移元素后返回queue2的队首。
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    //TODO
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            //TODO
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //如果q1为空,则直接返回错误
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
        //将q1中除了最后一个元素的所有元素都转移到q2
        while self.q1.size() > 1 {
            let front = self.q1.dequeue().unwrap();
            self.q2.enqueue(front);
        }
        //弹出q1的最后一个元素,也就是栈顶元素
        let top = self.q1.dequeue().unwrap();
        //交换q1和q2的内存空间,使得q1指向q2,q2指向q1
        std::mem::swap(&mut self.q1, &mut self.q2);
        //返回栈顶元素
        Ok(top)
    }

    pub fn is_empty(&self) -> bool {
        //如果q1为空,则栈为空
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
