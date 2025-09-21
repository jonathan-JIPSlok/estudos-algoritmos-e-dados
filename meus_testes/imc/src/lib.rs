use std::io;

fn coletar_altura() -> f64 {
    let mut altura_usuario = String::new(); //Cria uma váriavel tipo Stirng
    println!("Insira sua altura:"); 
    io::stdin().read_line(&mut altura_usuario).unwrap(); //lê a linha digitada pelo usuário
    altura_usuario = altura_usuario.replace(",", ".").replace(" ", ""); //substitui vírgulas por pontos e remove espaços em branco
    let altura_usuario: f64 = altura_usuario.trim().parse().unwrap(); // converte a string para um número de ponto flutuante (f64)
    
    return altura_usuario
}

fn coletar_peso() -> f64 {
    let mut peso_usuario = String::new(); //Cria uma váriavel tipo String
    println!("Insira seu peso:");
    io::stdin().read_line(&mut peso_usuario).unwrap(); // lê a linha digitada pelo usuário
    peso_usuario = peso_usuario.replace(" ", "").replace(",", ".");
    let peso_usuario: f64 = peso_usuario.trim().parse().unwrap(); //Converte a string para um número de ponto flutuante (f64)

    return peso_usuario;
}

fn definir_grau_ibesidade(imc: f64) -> &'static str {
    let mut grau_obesidade: &str = "";

    if imc < 18.5 {
        grau_obesidade = "Abaixo do peso normal";
    } else if imc >= 18.5 && imc < 25.0 {
        grau_obesidade = "Peso Normal";
    } else if imc >= 25.0 && imc < 30.0 {
        grau_obesidade = "Excesso de Peso";
    } else if imc >= 30.0 && imc < 35.0 {
        grau_obesidade = "Obesidade classe I";
    } else if imc >= 35.0 && imc < 40.0 {
        grau_obesidade = "Obesidade classe II";
    } else if imc >= 40.0 {
        grau_obesidade = "Obesidade classe III";
    }

    return grau_obesidade;
}

pub fn calculo_imc() {
    println!("{:->50}", "-");
    println!("{:^50}", "Cálculo de IMC");
    println!("{:->50}", "-");

    let altura = coletar_altura();
    let peso = coletar_peso();

    let imc: f64 = peso / (altura * altura);
    
    let obesidade = definir_grau_ibesidade(imc);

    println!("Seu IMC é '{:.1}' e você está classificado como: {}", imc, obesidade);
}