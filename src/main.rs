use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

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

    println!("Conteudo:\n{}", contents);
}
