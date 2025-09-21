/*fn main() {
    println!("Hello, world!");

    let mut x = 10; //let cria uma váriavel, mut torna ela mutável

    x = 11; //muda o valor da váriavel
    println!("Á variável X passa a ser {}", x); //imprime o valor da váriavel

    let texto: &str = "Olá Mundo!!"; //Cria uma váriavel do tipo string

    let numero: i32 = 11; //Cria uma váriavel do tipo inteiro 32bit

    println!("a váriavel texto tem tipágem de string {}, enquanto a váriavel de numero tem tipagem de inteiro 32bit {}", texto, numero);

    fn soma(a: i32, b: i32) -> i32 {
        a + b
    } //função que soma dois números inteiros e retorna o resultado. Sempre retorna o valor da última expressão.

    println!("A soma entre numero e x é: {}", soma(numero, x));

    for i in 0..5 {
        println!("{}", i);
    } //loop que imprime os números de 0 a 4, 0..5 é um range no rust.

    let mut vetor = vec![1,2,5,4]; //Cria um vetor mutável com os valores 1,2,5,4
    vetor.push(4); //Adiciona o valor 4 ao final do vetor

    println!("{:?}", vetor); //Imprime o vetor, {:?} é usado para imprimir estruturas de dados complexas como vetores.

    if x > 10 { //Estrutura condicional que verifica se x é maior que 10
        println!("Maior");
    } else {
        println!("Menor ou igual");
    }

    let cor = "azul"; //Cria uma váriavel do tipo string

    match cor { //Estrutura condicional que verifica o valor da váriavel cor
        "vermelho" => println!("É vermelho"),
        _ => println!("Outra cor"),
    }

    //let mut numero = 1; //Cria uma váriavel do tipo inteiro mutável
    //let olha_numero = numero; //Cria uma referência para a váriavel numero que é imutável

    //olha_numero = 2; //Muda o valor da váriavel numero que é impedido pelo fato de que olha_numero é uma referência imutável
    //println!("O número é: {}", numero); //Imprime o valor da váriavel numero através da referência

    use impar_par::impar_par;
    println!("{}", if impar_par(6) == false {
        "impar"
    } else {
        "par"
    }); // importei minha biblioteca que retor valor booleano para dizer se um numero é impar ou par e mudei para que ela mostre "impar", "par".

    use imc::calculo_imc;
    calculo_imc();

} */

fn main() {

    use vetores::vetor;
    vetor();
}