/*
    heap
    This question requires you to implement a binary heap function
实现了一个泛型的二叉堆数据结构，支持最小堆和最大堆两种模式。堆的性质通过比较函数comparator来维护，
用户可以通过add方法向堆中添加元素，并通过迭代器接口next方法取出堆顶元素。代码还提供了两个辅助结构体MinHeap和MaxHeap，
用于创建特定类型的堆。此外，代码中包含了一些测试用例，以验证堆的正确性和功能完整性。
总之，这段代码的主要功能是提供一个灵活且高效的二叉堆实现，适用于需要快速访问最小或最大元素的场景。
*/

//use std::cmp::Ord; 和 use std::default::Default; 导入了 Rust 标准库中的两个 trait，Ord 用于比较元素的顺序，Default 用于为类型提供默认值。
use std::cmp::Ord;
use std::default::Default;

//count 表示堆中元素的数量，
//items 是一个 Vec<T> 用于存储堆中的元素，comparator 是一个比较函数，用于确定堆的性质（最小堆或最大堆）。
pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    //new 方法用于创建一个新的堆，并初始化比较函数 comparator。

    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }
    //len 和 is_empty 方法分别用于获取堆中元素的数量以及判断堆是否为空。

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    //add 方法用于向堆中添加新的元素。添加后，通过比较函数和父节点的比较，将新元素上浮到正确的位置，以保持堆的性质。

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;

        let mut idx = self.count;
        //idx > 1：确保当前索引不是根节点（索引为1）
        //比较当前节点和其父节点的值。如果当前节点满足堆的性质（根据 comparator 的定义），则继续上浮。
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {
            let parent_idx = self.parent_idx(idx); //计算当前节点的父节点索引。
            self.items.swap(idx, parent_idx); //交换当前节点和其父节点的位置。
            idx = self.parent_idx(idx); //更新索引为父节点的索引，以便继续向上比较。
        }
    }
    //parent_idx、left_child_idx 和 right_child_idx 方法分别用于计算父节点、左子节点和右子节点的索引。

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }
    //children_present 用于检查给定节点是否有子节点。

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
    /*
        根据堆的性质（通过比较函数 comparator 决定），找到当前节点的最小子节点或最大子节点的索引。具体步骤如下：
        1、计算当前节点的左子节点和右子节点的索引。
        2、检查右子节点是否存在。
        3、如果右子节点不存在，返回左子节点的索引。
        4、如果右子节点存在，使用比较函数 comparator 比较左右子节点的值，返回较小的（或较大的）子节点的索引。
    */
    fn smallest_child_idx(&self, idx: usize) -> usize {
        //left_idx: 通过调用 left_child_idx 方法计算当前节点的左子节点索引。在二叉堆中，左子节点的索引为当前节点索引的两倍。
        //right_idx: 通过调用 right_child_idx 方法计算当前节点的右子节点索引。右子节点的索引是左子节点索引加一。
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        if right_idx > self.count {
            left_idx //如果右子节点不存在，返回左子节点的索引。因为在二叉堆中，每个非叶子节点至少有一个左子节点
        } else if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            // 使用堆的比较函数 comparator 比较左子节点和右子节点的值。
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    //next 方法用于取出堆顶元素（最小堆或最大堆），并重新调整堆以保持其性质。取出堆顶元素后，将最后一个元素移动到堆顶，并向下调整以恢复堆的性质。
    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        let next = Some(self.items.swap_remove(1));
        self.count -= 1;

        if self.count > 0 {
            let mut idx = 1;
            while self.children_present(idx) {
                let cdx = self.smallest_child_idx(idx);
                if !(self.comparator)(&self.items[idx], &self.items[cdx]) {
                    self.items.swap(idx, cdx);
                }
                idx = cdx;
            }
        }

        next
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
