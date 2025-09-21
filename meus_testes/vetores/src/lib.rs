pub fn vetor() {
    let mut meu_vetor = vec![1,5,2,4,7,6];

    println!("Vetor original: {:?}", meu_vetor);

    meu_vetor.push(3); // Adiciona o elemento 3 ao final do vetor
    println!("Vetor após adicionar o número 3: {:?}", meu_vetor);

    meu_vetor.remove(3); // Remove o elemento 3 do vetor
    println!("Vetor após remover o número 5: {:?}", meu_vetor);

    meu_vetor.sort(); // Ordena o vetor em ordem crescente
    println!("Vetor ordenado: {:?}", meu_vetor);

    meu_vetor.reverse(); // Inverte a ordem dos elementos do vetor
    println!("Vetor invertido: {:?}", meu_vetor);

    let mut novo_vetor = &meu_vetor.clone(); // Cria uma cópia do vetor original que não permite mutabilidade.
    println!("Cópia do vetor: {:?}", novo_vetor);

    //novo_vetor.push(3); // Adiciona o elemento 3 ao final do novo vetor
    //println!("Novo vetor após adicionar o número 3: {:?}", novo_vetor);

    println!("Indice 3 do meu novo_vetor: {}", novo_vetor[5]); // Acessa o elemento no índice 5 do vetor
}
