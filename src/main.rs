enum Node<T> {
    Data(T, Box<Self>),
    Empty
}

fn main() {
    let data: Node<u8> = Node::Data(2, Box::new(Node::Empty));
    {
        let data2: Node<u8> = Node::Data(2, Box::new(data));
        let data3: Node<u8> = Node::Data(3, Box::new(data));
    }
    
}