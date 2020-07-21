use bit_field::BitArray;
fn main() {
    println!("Hello, world!");

    let mut t = SegmentTreeAllocator::new(13);
    t.alloc();
    t.alloc();
    t.alloc();
    t.alloc();
    t.alloc();


    t.dealloc(1);

    let mut a = vec![1,2,3];
    a.set_bit(4,true);
    println!("{:?}",a);
    println!("{:?}",a.get_bit(0));

}
pub struct SegmentTreeAllocator {
    /// 树本身
    tree: Vec<u8>,
}

impl SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        assert!(capacity >= 8);
        // 完全二叉树的树叶数量
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
        Self { tree }
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.tree.get_bit(1) {
            None
        } else {
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
        self.tree.set_bit(index, value);
        while index > 1 {
            index /= 2;
            let v = self.tree.get_bit(index * 2) && self.tree.get_bit(index * 2 + 1);
            self.tree.set_bit(index, v);
        }
    }
}
