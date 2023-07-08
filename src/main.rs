use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Você deve inserir um path para um arquivo de entrada!");
        exit(exitcode::USAGE);
    }

    // Pega o argumento
    let filepath = &args[1];

    println!("Arquivo: {}", filepath);
    let mut f = File::open(filepath).expect("Arquivo não encontrado!");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Erro ao ler o arquivo!");

    println!("Conteudo:\n{}\n", contents);

    // Pegando os valores da matriz com regex
    let regex = Regex::new(r"\d+").unwrap();
    let valores_matriz: Vec<i32> = regex
        .find_iter(&contents)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();

    // Calculando tamanho da matriz
    let tam_matriz: u32 = (valores_matriz.len() as f64).sqrt() as u32;

    println!("Tam matriz: {}", tam_matriz);
    println!("Matriz: {:?}", valores_matriz);


}
