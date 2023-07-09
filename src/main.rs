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
    let mut valores_matriz: Vec<i32> = regex
        .find_iter(&contents)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();

    // Calculando tamanho da matriz
    let tam_matriz: u32 = (valores_matriz.len() as f64).sqrt() as u32;
    // Criando os numeros para permutar
    let mut permutation_numbers: Vec<i32> = Vec::new();
    for i in 0..tam_matriz {
        permutation_numbers.push(i as i32);
    }

    println!("Tam matriz: {}", tam_matriz);
    println!("Matriz: {:?}", valores_matriz);

    let mut best_size = i32::MAX;

    println!();

    let start = SystemTime::now();
    pinzon_rodrigues_lisboa(
        permutation_numbers.as_mut_slice(),
        0,
        &mut valores_matriz,
        &mut best_size,
    );
    let total_time = SystemTime::now().duration_since(start).unwrap();

    println!(
        "\nBest size (dps de rodar pinzon-rodrigues-lisboa): {}",
        best_size
    );

    print!(
        "Execução terminada em: {} segundos",
        total_time.as_secs_f64()
    );
}

fn pinzon_rodrigues_lisboa(
    numbers: &mut [i32],
    start: usize,
    matriz: &mut Vec<i32>,
    size_path: &mut i32,
) {
    if start == numbers.len() {
        let temp_size_path = calcula_tam_caminho(&numbers, matriz);

        if *size_path > temp_size_path {
            *size_path = temp_size_path;
        }
        return;
    }

    for i in start..numbers.len() {
        numbers.swap(start, i);
        pinzon_rodrigues_lisboa(numbers, start + 1, matriz, size_path);
        numbers.swap(start, i);
    }
}

fn calcular_index(lin: i32, col: i32, cols: i32) -> i32 {
    return (lin * cols) + col;
}

fn calcula_tam_caminho(path: &[i32], matriz: &mut Vec<i32>) -> i32 {
    let mut size = 0;
    let mut i = 0;
    while i + 1 < path.len() {
        size = matriz
            .get(calcular_index(path[i], path[i + 1], path.len() as i32) as usize)
            .unwrap()
            + size;
        i += 1;
    }
    size = matriz
        .get(calcular_index(path[i], path[0], path.len() as i32) as usize)
        .unwrap()
        + size;

    return size;
}
