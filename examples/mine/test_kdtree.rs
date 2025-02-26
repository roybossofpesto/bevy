
//! test kdtree lib
use kd_tree::{KdPoint, KdTree};

// define your own item type.
struct Item {
    point: [f64; 2],
    id: usize,
}

// implement `KdPoint` for your item type.
impl KdPoint for Item {
    type Scalar = f64;
    type Dim = typenum::U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f64 { self.point[k] }
}

fn main() {

    // construct kd-tree from `Vec<Item>`.
    // Note: you need to use `build_by_ordered_float()` because f64 doesn't implement `Ord` trait.
    let kdtree: KdTree<Item> = KdTree::build_by_ordered_float(vec![
        Item { point: [1.0, 2.0], id: 111 },
        Item { point: [2.0, 3.0], id: 222 },
        Item { point: [3.0, 4.0], id: 333 },
    ]);

    // search nearest item from [1.9, 3.1]
    assert_eq!(kdtree.nearest(&[1.9, 3.1]).unwrap().item.id, 222);

}
