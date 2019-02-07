use std::fmt;

enum Node<'a, T> {
    Data(T, &'a Self),
    Empty
}

impl<'a, T: fmt::Display> fmt::Display for Node<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Empty => write!(f, "empty"),
            Node::Data(h,t) => write!(f, "{}, {}", h, t)
        }
    }
}

fn prepend<'a, T>(list: &'a Node<T>, value: T) -> Node<'a, T> {
    match list {
        Node::Data(_,_) => Node::Data(value, list),
        Node::Empty => Node::Data(value, &Node::Empty)
    }
}

fn main() {
    let data: Node<u8> = Node::Data(2, &Node::Empty);
    let data2 = prepend(&data, 3);
    println!("{}", data2);
}