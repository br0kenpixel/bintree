///! A binary tree library.
use std::ops::Deref;

mod lang_extensions;
#[cfg(tests)]
mod tests;

/// A binary tree.
pub struct BinTree<T> {
    inner: T,
    left: Option<Box<BinTree<T>>>,
    right: Option<Box<BinTree<T>>>,
}

impl<T> BinTree<T> {
    /// Creates a new binary tree root node.
    /// Left and right nodes are automatically set to `None`.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree: BinTree<i32> = BinTree::new(8);
    /// ```
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            left: None,
            right: None,
        }
    }

    /// Creates a new binary tree root node, but also allows immediatly setting the
    /// left and right nodes.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree: BinTree<i32> = BinTree::new_with_nodes(1, 2, 3);
    /// assert_eq!(my_tree.get_inner(), &1);
    /// assert_eq!(my_tree.get_left().unwrap().get_inner(), &2);
    /// assert_eq!(my_tree.get_right().unwrap().get_inner(), &3);
    /// ```
    pub fn new_with_nodes(inner: T, left: T, right: T) -> Self {
        let left = Self::new(left);
        let right = Self::new(right);

        Self {
            inner,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    /// Returns a [`Box`](Box)ed version of the node itself.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree: BinTree<i32> = BinTree::new(25);
    /// let boxed: Box<BinTree<i32>> = my_tree.boxed();
    /// // `my_tree` is no longer valid
    /// assert_eq!(boxed.get_inner(), &25);
    /// ```
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Sets the inner value of the node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new(8);
    ///
    /// assert_eq!(my_tree.get_inner(), &8);
    ///
    /// my_tree.set_inner(9);
    ///
    /// assert_eq!(my_tree.get_inner(), &9);
    /// ```
    pub fn set_inner(&mut self, value: T) {
        self.inner = value;
    }

    /// Sets the left value of the node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new(8);
    /// my_tree.set_left(Some(9));
    ///
    /// assert_eq!(my_tree.get_left().unwrap().get_inner(), &9);
    /// ```
    pub fn set_left(&mut self, value: Option<T>) {
        if let Some(value) = value {
            self.left = Some(Self::new(value).boxed());
        } else {
            self.clear_left();
        }
    }

    /// Clears the node's left value.
    /// This is the same as calling [set_left(None)](BinTree::set_left) but it's more clear.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new_with_nodes(1, 2, 3);
    /// my_tree.clear_left();
    ///
    /// assert_eq!(my_tree.get_inner(), &1);
    /// assert!(my_tree.get_left().is_none());
    /// assert_eq!(my_tree.get_right().unwrap().get_inner(), &3);
    /// ```
    pub fn clear_left(&mut self) {
        self.left = None;
    }

    /// Sets the right value of the node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new(8);
    /// my_tree.set_right(Some(9));
    ///
    /// assert_eq!(my_tree.get_right().unwrap().get_inner(), &9);
    /// ```
    pub fn set_right(&mut self, value: Option<T>) {
        if let Some(value) = value {
            self.right = Some(Self::new(value).boxed());
        } else {
            self.clear_right();
        }
    }

    /// Clears the node's right value.
    /// This is the same as calling [set_right(None)](BinTree::set_right) but it's more clear.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new_with_nodes(1, 2, 3);
    /// my_tree.clear_right();
    ///
    /// assert_eq!(my_tree.get_inner(), &1);
    /// assert_eq!(my_tree.get_left().unwrap().get_inner(), &2);
    /// assert!(my_tree.get_right().is_none());
    /// ```
    pub fn clear_right(&mut self) {
        self.right = None;
    }

    /// Returns a borrowed reference to the inner value of the node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new(1);
    /// assert_eq!(my_tree.get_inner(), &1);
    /// ```
    pub fn get_inner(&self) -> &T {
        &self.inner
    }

    /// Returns a borrowed __mutable__ reference to the inner value of the node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new(1);
    ///
    /// assert_eq!(my_tree.get_inner(), &1);
    ///
    /// let inner = my_tree.get_inner_mut();
    /// *inner = 99;
    ///
    /// assert_eq!(my_tree.get_inner(), &99);
    /// ```
    pub fn get_inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Returns the inner value of the node while also consuming the node itself.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree = BinTree::new(1);
    /// assert_eq!(my_tree.take_inner(), 1);
    /// ```
    pub fn take_inner(self) -> T {
        self.inner
    }

    /// Returns the left value of the node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree = BinTree::new_with_nodes(1, 2, 3);
    /// assert_eq!(my_tree.get_left().unwrap().get_inner(), &2);
    /// ```
    #[allow(clippy::borrowed_box)]
    pub fn get_left(&self) -> Option<&Self> {
        self.left.as_deref()
    }

    /// Returns a __mutable__ reference to the left value of the node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new_with_nodes(1, 2, 3);
    ///
    /// assert_eq!(my_tree.get_left().unwrap().get_inner(), &2);
    ///
    /// let left = my_tree.get_left_mut().unwrap();
    /// left.set_inner(99);
    ///
    /// assert_eq!(my_tree.get_left().unwrap().get_inner(), &99);
    /// ```
    pub fn get_left_mut(&mut self) -> Option<&mut Self> {
        self.left.as_deref_mut()
    }

    /// Returns the left value of the node while also consuming the node itself.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree = BinTree::new_with_nodes(1, 2, 3);
    /// let left_node = my_tree.take_left().unwrap();
    /// assert_eq!(left_node.get_inner(), &2);
    /// ```
    pub fn take_left(self) -> Option<Box<Self>> {
        self.left
    }

    /// Pops the left value of the node, leaving `None` in it's place.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new_with_nodes(1, 2, 3);
    /// let left_value = my_tree.pop_left();
    /// assert!(my_tree.get_left().is_none());
    /// ```
    pub fn pop_left(&mut self) -> Option<Box<Self>> {
        self.left.take()
    }

    /// Returns the right value of the node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree = BinTree::new_with_nodes(1, 2, 3);
    /// assert_eq!(my_tree.get_right().unwrap().get_inner(), &3);
    /// ```
    #[allow(clippy::borrowed_box)]
    pub fn get_right(&self) -> Option<&Self> {
        self.right.as_deref()
    }

    /// Returns a __mutable__ reference to the right value of the node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new_with_nodes(1, 2, 3);
    ///
    /// assert_eq!(my_tree.get_right().unwrap().get_inner(), &3);
    ///
    /// let right = my_tree.get_right_mut().unwrap();
    /// right.set_inner(99);
    ///
    /// assert_eq!(my_tree.get_right().unwrap().get_inner(), &99);
    /// ```
    pub fn get_right_mut(&mut self) -> Option<&mut Self> {
        self.right.as_deref_mut()
    }

    /// Returns the right value of the node while also consuming the node itself.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree = BinTree::new_with_nodes(1, 2, 3);
    /// let right_node = my_tree.take_right().unwrap();
    /// assert_eq!(right_node.get_inner(), &3);
    /// ```
    pub fn take_right(self) -> Option<Box<Self>> {
        self.right
    }

    /// Pops the right value of the node, leaving `None` in it's place.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new_with_nodes(1, 2, 3);
    /// let right_value = my_tree.pop_right();
    /// assert!(my_tree.get_right().is_none());
    /// ```
    pub fn pop_right(&mut self) -> Option<Box<Self>> {
        self.right.take()
    }

    /// Returns a tuple containing:
    /// 1. The inner value
    /// 2. The left value
    /// 3. The right value
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree = BinTree::new_with_nodes(-77, 31, 128);
    /// let numbers = my_tree.collect_values();
    ///
    /// assert_eq!(numbers.0, -77);
    /// assert_eq!(numbers.1, Some(31));
    /// assert_eq!(numbers.2, Some(128));
    /// ```
    pub fn collect_values(self) -> (T, Option<T>, Option<T>) {
        (
            self.inner,
            self.left.map(|boxed| boxed.inner),
            self.right.map(|boxed| boxed.inner),
        )
    }

    /// Calculate the amount of values the binary tree is holding.
    /// # Note
    /// This method also counts the head node.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree = BinTree::new_with_nodes(-77, 31, 128);
    /// assert_eq!(my_tree.len(), 3);
    /// ```
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.collect_nodes().len() + 1
    }

    /// Returns a [`Vec`](Vec) of references to all nodes in the binary tree.
    /// This allows "browsing" the tree as if it was an array.
    ///
    /// ## Notes
    /// This method is recursive. Traveling left is prioritized.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree = BinTree::new_with_nodes(-77, 31, 128);
    /// let nodes = my_tree.collect_nodes();
    ///
    /// assert_eq!(nodes[0].get_inner(), &31);
    /// assert_eq!(nodes[1].get_inner(), &128);
    /// assert!(nodes.get(2).is_none());
    /// ```
    pub fn collect_nodes(&self) -> Vec<&BinTree<T>> {
        let mut values = Vec::new();

        if let Some(node) = &self.left {
            values.push(node.deref());
            values.extend(node.collect_nodes());
        }

        if let Some(node) = &self.right {
            values.push(node.deref());
            values.extend(node.collect_nodes());
        }

        values
    }

    /// Returns a [`Vec`](Vec) of references to all values the binary tree is holding.
    /// The returned array will always start with the head node's value.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let my_tree = BinTree::new_with_nodes(-77, 31, 128);
    /// let values = my_tree.collect_all_values();
    ///
    /// assert_eq!(values[0], &-77);
    /// assert_eq!(values[1], &31);
    /// assert_eq!(values[2], &128);
    /// assert!(values.get(3).is_none());
    /// ```
    pub fn collect_all_values(&self) -> Vec<&T> {
        let mut values = vec![&self.inner];
        values.extend(self.collect_nodes().iter().map(|node| &node.inner));
        values
    }

    /// Creates a very simple [`Vec`](Vec) iterator by collecting all values
    /// using [`collect_all_values()`](Self::collect_all_values).
    ///
    /// This is equivalent to `collect_all_values().into_iter()`.
    ///
    /// # Notes
    /// Creating this kind of iterator may be slow, as the tree needs to be
    /// iterated to collect all the values first. Since [`collect_all_values()`](Self::collect_all_values)
    /// returns references, not copies, the memory usage by the iterator should be roughly
    /// `size_of::<&T>() * tree.`[`len()`](Self::len).
    pub fn into_fast_iter(&self) -> std::vec::IntoIter<&T> {
        self.collect_all_values().into_iter()
    }

    /// Returns the leftmost node in the tree.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new_with_nodes(-77, 31, 128);
    /// let left = my_tree.get_left_mut().unwrap();
    /// left.set_left(Some(69));
    ///
    /// /*
    ///   Tree:
    ///      (-77)
    ///      /   \
    ///    (31)  (128)
    ///    /
    ///  (69)
    /// */
    ///
    /// assert_eq!(my_tree.leftmost().get_inner(), &69);
    /// ```
    pub fn leftmost(&self) -> &Self {
        let mut node = self;

        while let Some(next) = node.get_left() {
            node = next;
        }
        node
    }

    /// Returns the rightmost node in the tree.
    ///
    /// ## Example
    /// ```rust
    /// use bintree::BinTree;
    ///
    /// let mut my_tree = BinTree::new_with_nodes(-77, 36, 128);
    /// let left = my_tree.get_right_mut().unwrap();
    /// left.set_right(Some(79));
    ///
    /// /*
    ///   Tree:
    ///      (-77)
    ///      /   \
    ///    (36)  (128)
    ///              \
    ///              (79)
    /// */
    ///
    /// assert_eq!(my_tree.rightmost().get_inner(), &79);
    /// ```
    pub fn rightmost(&self) -> &Self {
        let mut node = self;

        while let Some(next) = node.get_right() {
            node = next;
        }
        node
    }
}

impl<T: Clone> BinTree<T> {
    /// Returns a cloned copy of the node's inner value.
    pub fn clone_inner(&self) -> T {
        self.inner.clone()
    }

    /// Returns a cloned copy of the node's left value.
    pub fn clone_left(&self) -> Option<T> {
        self.left.as_ref().map(|boxed| boxed.clone_inner())
    }

    /// Returns a cloned copy of the node's right value.
    pub fn clone_right(&self) -> Option<T> {
        self.right.as_ref().map(|boxed| boxed.clone_inner())
    }
}
