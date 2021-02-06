#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    data: i32,
}


impl Node {
    pub fn new() -> Node {
        Node {
            children: vec!(),
            data: 0
        }
    }

    pub fn expand(&mut self){
        self.children = vec!(Node::new(), Node::new());
    }

    pub fn is_leaf(&self) -> bool{
        self.children.len() == 0
    }

    fn extend(&mut self) {
        if self.is_leaf() {
            self.expand();
        } else {
            let index = 0;
            self.children[index].extend();
        }
        self.data += 1;
    }


}

pub fn run(){
    let mut root = Node::new();
    for _ in 0..10  {
        root.extend();
    }
    print!("{:?}", root);
}
