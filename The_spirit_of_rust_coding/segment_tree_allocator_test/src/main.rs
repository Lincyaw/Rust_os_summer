use bit_field::BitArray;
use bitflags::_core::cmp::min;

fn main() {
    // println!("Hello, world!");
    //
    // let mut t = SegmentTreeAllocator::new(8);
    // t.alloc();
    // t.alloc();
    // t.alloc();
    // t.alloc();
    // t.alloc();
    //
    //
    // t.dealloc(1);

    // let mut a = vec![1u8,2u8,3u8];
    // a.set_bit(4,true);
    // println!("{:?}",a);
    // println!("{:?}",a.get_bit(9));
    // println!("{:?}",a.get_bit(14));
    // println!("{:?}",a.get_bit(15));
    // println!("{:?}",a.get_bit(16));

    // let mut t = SegmentTree::new(6);
    // t.alloc();
    // t.alloc();
    // t.alloc();
    // t.alloc();
    // t.alloc();
    //
    //
    // t.dealloc(1);


}
pub struct SegmentTreeAllocator {
    /// 树本身
    tree: Vec<u8>,
}

impl SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        assert!(capacity >= 8);
        // 完全二叉树的树叶数量
        println!("开始新建一棵树");
        let leaf_count = capacity.next_power_of_two();
        println!("leaf_count = {}", leaf_count);
        let mut tree = vec![0u8; 2 * leaf_count];
        println!("tree = {:?},  len = {}", tree, tree.len());
        // 去除尾部超出范围的空间
        println!("((capacity + 7) / 8) = {}, (leaf_count / 8) = {}.",((capacity + 7) / 8), (leaf_count / 8));
        for i in ((capacity + 7) / 8)..(leaf_count / 8) {
            tree[leaf_count / 8 + i] = 255u8;
        }
        println!("tree = {:?},  len = {}", tree, tree.len());
        for i in capacity..(capacity + 8) {
            tree.set_bit(leaf_count + i, true);
        }
        println!("tree = {:?},  len = {}", tree, tree.len());
        // 沿树枝向上计算
        for i in (1..leaf_count).rev() {
            let v = tree.get_bit(i * 2) && tree.get_bit(i * 2 + 1);
            tree.set_bit(i, v);
        }
        println!("{:?}", tree);
        println!("树建立结束\n");
        Self { tree }
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.tree.get_bit(1) {
            None
        } else {
            println!("开始alloc");
            let mut node = 1;
            // 递归查找直到找到一个值为 0 的树叶
            while node < self.tree.len() / 2 {
                if !self.tree.get_bit(node * 2) {
                    node *= 2;
                } else if !self.tree.get_bit(node * 2 + 1) {
                    node = node * 2 + 1;
                } else {
                    panic!("tree is full or damaged");
                }
            }
            // 检验
            assert!(!self.tree.get_bit(node), "tree is damaged");
            // 修改树
            self.update_node(node, true);

            println!("tree = {:?},  len = {}", self.tree, self.tree.len());
            println!("alloc结束");
            Some(node - self.tree.len() / 2)
        }
    }

    fn dealloc(&mut self, index: usize) {
        let node = index + self.tree.len() / 2;
        assert!(self.tree.get_bit(node));

        self.update_node(node, false);
        println!("tree = {:?},  len = {}", self.tree, self.tree.len());
    }
}

impl SegmentTreeAllocator {
    /// 更新线段树中一个树叶，然后递归更新其祖先
    fn update_node(&mut self, mut index: usize, value: bool) {
        println!("正在更新祖先");
        self.tree.set_bit(index, value);
        while index > 1 {
            index /= 2;
            let v = self.tree.get_bit(index * 2) && self.tree.get_bit(index * 2 + 1);
            self.tree.set_bit(index, v);
        }
        println!("祖先更新结束");
    }
}






pub struct SegmentTree {
    segment: Vec<u8>,
    capacity:usize,
    leaf_count:usize,
}

impl SegmentTree {
    fn new(capacity: usize) -> Self {
        println!("开始新建一棵树");
        let leaf_count = capacity.next_power_of_two();
        let mut tree =vec![0u8;(2*leaf_count/8)];
        for i in capacity..leaf_count{
            tree.set_bit(leaf_count+i,true);
        }
        for i in (1..leaf_count).rev(){
            let v = tree.get_bit(i * 2) && tree.get_bit(i * 2 + 1);
            tree.set_bit(i,v);
        }
        println!("{:?}", tree);
        println!("树建立结束");
        Self{segment:tree,capacity:capacity,leaf_count:leaf_count}
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.segment.get_bit(1){
            None
        }else{
            let mut node =1;
            while node<self.leaf_count{
                if !self.segment.get_bit(node*2) {
                    node *= 2;
                }else if !self.segment.get_bit(node*2+1){
                    node = node*2+1;
                }else{
                    panic!("tree damaged");
                }
            }
            self.update_node(node, true);
            println!("alloc开始");
            println!("{:?}", self.segment);
            println!("alloc结束");
            Some(node-self.leaf_count)

        }
    }

    fn dealloc(&mut self, index: usize) {

        let node = index + self.leaf_count;
        assert!(self.segment.get_bit(node));

        self.update_node(node, false);
        println!("dealloc开始");
        println!("{:?}", self.segment);
        println!("dealloc结束");
    }
    fn update_node(&mut self, mut index: usize, value: bool) {
        self.segment.set_bit(index,value);
        while index>1{
            index/=2;
            let v = self.segment.get_bit(index * 2) && self.segment.get_bit(index * 2 + 1);
            self.segment.set_bit(index,v);
        }
    }
}

