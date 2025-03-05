/*
    stack
    This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}
impl<T> Stack<T> {
    //new()：创建一个新的空栈。
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }
    //is_empty()：判断栈是否为空。
    fn is_empty(&self) -> bool {
        0 == self.size
    }
    //len()：返回栈中元素的数量。
    fn len(&self) -> usize {
        self.size
    }
    //clear()：清空栈。
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }
    //push()：将元素压入栈中。
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }
    //pop()：从栈中弹出元素。
    fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.data.pop().unwrap())
        } else {
            None
        }
    }
    //peek()：返回栈顶元素的引用，但不弹出。
    fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }
        self.data.get(self.size - 1)
    }
    //peek_mut()：返回栈顶元素的可变引用，但不弹出。
    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }
    //into_iter()：将栈转换为一个迭代器。
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    //iter()：返回栈的不可变迭代器。
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }
    //	iter_mut()：返回栈的可变迭代器。
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}
struct IntoIter<T>(Stack<T>);
// IntoIter 是一个迭代器，每次调用 next() 会弹出栈顶元素。
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}
struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
//Iter 是一个不可变迭代器，每次调用 next() 会返回栈顶元素的引用。
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
//IterMut 是一个可变迭代器，每次调用 next() 会返回栈顶元素的可变引用。
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

/*
bracket_match()：判断括号匹配是否正确。
1、遍历字符串中的每个字符。
2、如果遇到左括号 (, [, {，将其压入栈中。
3、如果遇到右括号 ), ], }，弹出栈顶元素并检查是否与当前右括号匹配。
4、如果栈为空或栈顶元素不匹配，返回 false。
5、遍历结束后，如果栈为空，说明所有括号都匹配，返回 true；否则返回 false。
 */
fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    for ch in bracket.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => continue,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
