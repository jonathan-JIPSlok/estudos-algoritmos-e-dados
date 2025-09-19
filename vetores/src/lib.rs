// src/lib.rs

pub struct MyVec { //Cira uma estrutura publica chamada MyVec
    data: Vec<i32>, // Diz que vai guardar um vetor com valores 32 bit na memória (i32)
}

impl MyVec { //Implementa o MyVec
    pub fn new() -> MyVec { //Função Construtor do MyVec
        MyVec {data: Vec::new() } //Aqui inicializa um novo vetor
    }

    pub fn push(&mut self, value: i32) { //Aqui ele tranforma o vetor em mutável
        self.data.push(value); //Aquie ele insere um valor no vetor
    }

    pub fn get(&self, index: usize) -> Option<&i32> { //Aqui ele coleta um valor no vetor, usize(para saber o tamanho do vetor)
        self.data.get(index) //Aqui ele traz o elemento
    }
}

// módulo de testes: esse código sera compilado e executado apenas quando rodarmos os testes.
#[cfg(test)]
mod tests {
    // Traz as definições do escopo superior (como a estrutura MyVec) para o módulo de testes.
    use super::*;

    #[test]
    fn test_push_and_get() {
        // "let mut vec = MyVec::new();"
        //
        // "let" é usado para declarar uma variável.
        // "mut" indica que a variável é mutável, ou seja, seu valor pode ser alterado depois de definida.
        // "MyVec::new()" chama a função associada "new" da estrutura MyVec para criar uma nova instância do vetor.
        let mut vec = MyVec::new();


        //Adiciona dois elementos.
        vec.push(10);
        vec.push(20);

        // Verifica se os elementos foram inseridos corretamente.
        assert_eq!(vec.get(0), Some(&10));
        assert_eq!(vec.get(1), Some(&20));
        // Um indice que não existe deve retornar None.
        assert_eq!(vec.get(2), None);
    }
}