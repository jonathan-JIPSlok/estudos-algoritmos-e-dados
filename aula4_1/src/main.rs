// Trabalhando com a estrutura Box -> faz uma alocação dinâmica e uma alocação segura da memória
// Box é nativo do Rust

struct Node {
    value: i32,
    next: Option<Box<Node>>, // Box<Node> está alocando uma memória para o Node (nó)
}

impl Node {
    fn new(value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            next: None,
        })
    }

    fn append(node: Box<Node>, value: i32) -> Box<Node> {
        Box::new(Node { // Aqui Box::new(Node) - Ele não está criando um novo, ele está apenas referenciando o que já foi criado ali em cima.
            value,
            next: Some(node),
        })
    }
}

fn main() {
    let first = Node::new(10);
    let second = Node::append(first, 20);

    println!("Primeiro nó: {}", second.next.as_ref().unwrap().value);
    println!("Segundo nó: {}", second.value);
}