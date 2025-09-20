// Trabalhando com ponteiros -> Rc

use std::rc::Rc; // Utilizando o Rc(Referênce counted) do rc da bibliotace standard

struct Node { // Criamos uma estrutura Node (que é um nó)
    value: i32, // valor é inteiro 32 bit
    next: Option<Rc<Node>>, // next recebe valores RC<Node> ou não recebe nada, Option indica ao rust que aquele valor pode ser o ultimo da lista.
}

impl Node { //Implementa o Node (nó)
    fn new(value: i32) -> Rc<Node> { // Essa não é uma função publica, só pode ser acessada pela estrutura principal.
        Rc::new(Node {
            value,
            next: None,
        })
    }

    fn append(node: &Rc<Node>, value: i32) -> Rc<Node> { // Adiciona um novo nó na lista
        Rc::new(Node {
            value,
            next: Some(Rc::clone(node)),
        })
    }
}

fn main() {
    // Cria o primeiro nó.
    let first = Node::new(10);

    // Adiciona mais um nó a lista.
    let second = Node::append(&first, 20);

    println!("Primeiro nó: {}", first.value);
    println!("Segundo nó: {}", second.value);
}