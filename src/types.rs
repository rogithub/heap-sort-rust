// https://github.com/critiqjo/binary-tree

pub struct Node<TValue, TKey> {
    value: TValue,
    key: TKey,
    left: Box<Option<Node<TValue, TKey>>>,
    right: Box<Option<Node<TValue, TKey>>>,
}

impl<TValue, TKey> Node<TValue, TKey> {
    fn new(key: TKey, value: TValue) -> Node<TValue, TKey> {
        Node {
            value,
            key,
            left: Box::new(None),
            right: Box::new(None),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let t = Node::new(1, "hola");

        assert_eq!(1, 1);
    }
}
