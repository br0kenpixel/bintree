use bintree::BinTree;

fn main() {
    let head = BinTree::new_with_nodes(1, 2, 3);
    println!("Head: {head}");
    println!("Left: {}", head.get_left().unwrap());
    println!("Right: {}", head.get_right().unwrap());
}
