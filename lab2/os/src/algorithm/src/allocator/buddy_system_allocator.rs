// #![no_std]
use core::alloc::AllocErr;
use super::linked_list;
use alloc::alloc::Layout;
use core::cmp::{max, min};
use core::mem::size_of;
use core::ptr::NonNull;
use core::alloc::GlobalAlloc;
use spin::Mutex;
use core::ops::Deref;
pub struct BuddySystemAllocator {
    // 这里使用了斯坦福大学实现的链表
    free_list: [linked_list::LinkedList; 32], // 长度为32的链表数组,序号为i的链表存放大小为2^i的空间, 这里的空间指地址

    user: usize,      // 用户需求的空间
    allocated: usize, // 实际已分配的空间
    total: usize,     // 总的空间
}

impl BuddySystemAllocator {

    pub const fn new() -> Self {
        BuddySystemAllocator {
            free_list: [linked_list::LinkedList::new(); 32],
            user: 0,
            allocated: 0,
            total: 0,
        }
    }

    /// Create an empty heap
    pub const fn empty() -> Self {
        Self::new()
    }

    // 分配内存到堆, 范围为[start, end)
    pub unsafe fn add_to_heap(&mut self, start: usize, end: usize) {
        let mut total = 0;
        let mut current_start = start;

        while current_start + size_of::<usize>() <= end {
            let lowbit = current_start & (!current_start + 1);
            // 令lowbit等于从右到左第一个不为0的位置, 只可能是1, 10, 100, 1000...
            // lowbit用于判断该地址最大能够存放多大的内存(基于伙伴系统, 地址应该是2^n次), 这种位运算真是巧妙
            let size = min(lowbit, prev_power_of_two(end - current_start));
            // prev_power_of_two(end - current_start) 是需要存放的内存, 即理论上这次能存放的最大的内存(必须是2^n)
            total += size; // size就是实际上能放的大小
                           // 再把这段内存首地址存放到对应的链表里
            self.free_list[size.trailing_zeros() as usize].push(current_start as *mut usize);
            current_start += size;
        }
        self.total += total;
    }

    // 为了对上heap.rs的接口, 封装一下addtoheap
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.add_to_heap(start, start + size);
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        ); // 保证内存对齐, 并且取2^n>=layout.size, n最小的内存作为size
        let class = size.trailing_zeros() as usize; //size有几个0

        // 查看数据
        // println!("\n申请");
        // println!("size: {}",size);
        // println!("class: {}", class);
        // println!("free_list_len: {}", self.free_list.len());

        for i in class..self.free_list.len() {
            // freelist的长度是这块堆区可分割的最大的n, 其中2^n==堆区大小
            // 因为需要可以分配的大小必须大于待分配的大小, 所以只要在堆区中找大于待分配的内存的大小的块即可
            if !self.free_list[i].is_empty() {
                // 如果存在2^i大小的块
                // println!("存在大小为2^{}的块",i);
                for j in (class + 1..i + 1).rev() {
                    //直到找到和size匹配的块
                    if let Some(block) = self.free_list[j].pop() {
                        // println!("freelist里大小为{}的块被成功取出来了, 首地址是{}",j, block as usize);
                        unsafe {
                            self.free_list[j - 1]
                                .push((block as usize + (1 << (j - 1))) as *mut usize);
                            // println!("freelist[{}]里存进去了{}",j-1, block as usize + (1 << (j - 1)));
                            self.free_list[j - 1].push(block);
                            // println!("freelist[{}]也里存进去了{}",j-1,block as usize);
                        }
                    } else {
                        return Err(AllocErr {});
                    }
                }
                //上面分割结束之后, 就拿出一块对应大小的用来给到需求
                let result = NonNull::new(
                    self.free_list[class]
                        .pop()
                        .expect("current block should have free space now") as *mut u8,
                );
                if let Some(result) = result {
                    //如果取出来没问题的话, 就把对应的数值改变一下
                    self.user += layout.size();
                    self.allocated += size;
                    return Ok(result);
                } else {
                    return Err(AllocErr {});
                }
            } else {
                // println!("不存在大小为2^{}的块",i);
            }
        }
        Err(AllocErr {})
    }

pub fn dealloc(&mut self,ptr: NonNull<u8>, layout: Layout) {
    let size = max(
        layout.size().next_power_of_two(),
        max(layout.align(), size_of::<usize>()),
    );
    let class = size.trailing_zeros() as usize;
    // println!("\n释放");
    // println!("size: {}",size);
    // println!("class: {}", class);
    unsafe {
        // Put back into free list 把这块内存重新放到堆区
        self.free_list[class].push(ptr.as_ptr() as *mut usize);

        // Merge free buddy lists
        //查看是否有能够合并的块
        let mut current_ptr = ptr.as_ptr() as usize;
        let mut current_class = class;
        while current_class < self.free_list.len() {
            let buddy = current_ptr ^ (1 << current_class); //改变 1class个0 那个位的取值, 原来是1就变成0, 原来是0就变成1
                                                            // println!("current_ptr: {}", current_ptr);
                                                            // println!("buddy: {}", buddy);
            let mut flag = false;
            for block in self.free_list[current_class].iter_mut() {
                // println!("开始在freelist[{}]中找和block邻近的块",current_class);
                if block.value() as usize == buddy {
                    // println!("block.value() as usize {} 是 buddy", block.value() as usize);
                    block.pop();
                    flag = true;
                    break;
                } else {
                    // println!("block.value() as usize {} 不是 buddy", block.value() as usize);
                }
            }

            // 如果找到了兄弟
            if flag {
                // println!("成功合并了current_ptr和buddy, 合并后的地址为{}", current_ptr);
                self.free_list[current_class].pop();
                current_ptr = min(current_ptr, buddy);
                current_class += 1;
                self.free_list[current_class].push(current_ptr as *mut usize);
            } else {
                break;
            }
        }
    }

    self.user -= layout.size();
    self.allocated -= size;
    }

}
#[cfg(feature = "use_spin")]
pub struct LockedHeap(Mutex<BuddySystemAllocator>);
#[cfg(feature = "use_spin")]
impl LockedHeap {
    /// Creates an empty heap
    pub const fn new() -> LockedHeap {
        LockedHeap(Mutex::new(BuddySystemAllocator::new()))
    }

    /// Creates an empty heap
    pub const fn empty() -> LockedHeap {
        LockedHeap(Mutex::new(BuddySystemAllocator::new()))
    }

    pub unsafe fn init(&self, start: usize, size: usize){
        self.0.lock().init(start, size)
    }
}
#[cfg(feature = "use_spin")]
impl Deref for LockedHeap {
    type Target = Mutex<BuddySystemAllocator>;

    fn deref(&self) -> &Mutex<BuddySystemAllocator> {
        &self.0
    }
}
#[cfg(feature = "use_spin")]
unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(NonNull::new_unchecked(ptr), layout)
    }
}



pub(crate) fn prev_power_of_two(num: usize) -> usize {
    1 << (8 * (size_of::<usize>()) - num.leading_zeros() as usize - 1)
}
