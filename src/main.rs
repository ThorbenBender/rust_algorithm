use std::env;
use std::rc::Rc;

#[derive(PartialEq)]
enum Node {
    Node { value: i32, next: Rc<Node> },
    None,
}

fn main() {
    let mut list = Rc::new(Node::None);
    env::args()
        .skip(1)
        .filter_map(|arg| arg.parse().ok())
        .for_each(|num| {
            let n = Rc::new(Node::Node {
                value: num,
                next: Rc::clone(&list),
            });

            list = n;
        });

    let mut ptr = list;
    while let Node::Node { value, next } = &*ptr {
        println!("{}", value);
        ptr = next.clone();
    }
}
