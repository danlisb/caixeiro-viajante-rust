use core::num;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::time::SystemTime;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Você deve inserir um path para um arquivo de entrada e um valor de debug!");
        exit(exitcode::USAGE);
    }

    // Pega o argumento de arquivo
    let filepath = &args[1];

    println!("Arquivo: {}", filepath);
    let f = File::open(filepath);

    let mut f = match f {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Arquivo não encontrado!");
            exit(exitcode::IOERR);
        }
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Erro ao ler o arquivo!");

    println!("Conteudo:\n{}\n", contents);

    // Pegando os valores da matriz com regex
    let regex = Regex::new(r"\d+").unwrap();
    let mut valores_matriz: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let temp = regex
            .find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();

        valores_matriz.push(temp);
    }

    // Calculando tamanho da matriz
    let tam_matriz: u32 = valores_matriz.len() as u32;

    // Criando os numeros para permutar
    let mut permutation_numbers: Vec<usize> = Vec::new();
    for i in 0..tam_matriz {
        permutation_numbers.push(i as usize);
    }

    println!("Tam matriz: {}", tam_matriz);
    println!("Matriz: {:?}", valores_matriz);

    let mut best_size = i32::MAX;
    let min_path: &mut Vec<usize> = &mut Vec::new();

    println!();

    let start = SystemTime::now();
    pinzon_rodrigues_lisboa(
        permutation_numbers.as_mut_slice(),
        1,
        &valores_matriz,
        &mut best_size,
        min_path,
    );
    let total_time = SystemTime::now().duration_since(start).unwrap();
    
    // Puts the first element in the end of the vector (return to start)
    min_path.push(min_path[0]);

    println!(
        "\nBest size (dps de rodar pinzon-rodrigues-lisboa): {}",
        best_size
    );

    println!("Best path: {:?}", min_path);

    print!(
        "Execução terminada em: {} segundos",
        total_time.as_secs_f64()
    );
}

fn pinzon_rodrigues_lisboa(
    numbers: &mut [usize],
    start: usize,
    matriz: &Vec<Vec<i32>>,
    size_path: &mut i32,
    min_path: &mut Vec<usize>,
) {
    if start == numbers.len() {
        let temp_size_path = calcula_tam_caminho(numbers, matriz);

        if *size_path > temp_size_path {
            *size_path = temp_size_path;
            *min_path = numbers.to_vec();
        }
        return;
    }

    for i in start..numbers.len() {
        numbers.swap(start, i);
        pinzon_rodrigues_lisboa(numbers, start + 1, matriz, size_path, min_path);
        numbers.swap(start, i);
    }
}

fn calcula_tam_caminho(path: &[usize], matriz: &Vec<Vec<i32>>) -> i32 {
    let mut size = 0;
    let mut i = 0;

    while i + 1 < path.len() {
        size += matriz[path[i]][path[i + 1]];
        i += 1;
    }
    size += matriz[path[i]][path[0]];

    return size;
}
