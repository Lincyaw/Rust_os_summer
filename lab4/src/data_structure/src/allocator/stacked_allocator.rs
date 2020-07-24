//! 提供栈结构实现的分配器 [`StackedAllocator`]

use super::Allocator;
use alloc::{vec, vec::Vec};

/// 使用栈结构实现分配器
///
/// 在 `Vec` 末尾进行加入 / 删除。
/// 每个元素 tuple `(start, end)` 表示 [start, end) 区间为可用。
pub struct StackedAllocator {
    list: Vec<(usize, usize)>,
}

impl Allocator for StackedAllocator {
    fn new(capacity: usize) -> Self {
        Self {
            list: vec![(0, capacity)],
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        self.alloc_frames(1)
    }
    fn alloc_frames(&mut self,num:usize) -> Option<usize>
    {

        if let Some((start, end)) = self.list.pop() {
            if end - start > num {
                self.list.push((start + num, end));
            }
            Some(start)
        } else {
            None
        }
    }
    fn dealloc(&mut self, index: usize,num:usize) {
        self.list.push((index, index + num-1));

    }
}