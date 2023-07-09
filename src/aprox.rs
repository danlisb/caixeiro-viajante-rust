use std::collections::HashSet;

fn nearest_neighbor(graph: &[Vec<f64>]) -> Vec<usize> {
    let mut path: Vec<usize> = Vec::new();
    let num_nodes = graph.len();
    let mut unvisited: HashSet<usize> = (0..num_nodes).collect();

    // Escolha um nó arbitrário como ponto de partida
    let start_node = 0;
    path.push(start_node);
    unvisited.remove(&start_node);

    while !unvisited.is_empty() {
        let current_node = *path.last().unwrap();
        let mut next_node = 0;
        let mut min_distance = f64::INFINITY;

        // Encontre o nó mais próximo não visitado
        for &neighbor in unvisited.iter() {
            let distance = graph[current_node][neighbor];
            if distance < min_distance {
                min_distance = distance;
                next_node = neighbor;
            }
        }

        path.push(next_node);
        unvisited.remove(&next_node);
    }

    path.push(start_node); // Volte ao ponto de partida para completar o ciclo

    path
}

fn main() {
    // Exemplo de grafo de distâncias entre cidades
    let graph = vec![
        vec![0.0, 2.0, 9.0, 10.0],
        vec![2.0, 0.0, 6.0, 4.0],
        vec![9.0, 6.0, 0.0, 8.0],
        vec![10.0, 4.0, 8.0, 0.0],
    ];

    let path = nearest_neighbor(&graph);
    println!("Caminho encontrado: {:?}", path);
}
