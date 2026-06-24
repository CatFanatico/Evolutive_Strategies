extern crate rand; 

use crate::rand::SeedableRng;
use crate::rand::Rng;
use rand::rngs::StdRng;
use rand::seq::index;

use std::fs::File;
use std::io::Write;



#[derive(Clone)]
struct Individual {
    board: Vec<Vec<u8>>,
    atack: u32,
}

fn main() {
    println!("Algoritmo genético_ Problema de las 8 reinas");
    //Parametros iniciales:
    
    let population_size = 300; //tamaño cde la población
    let mutation_rate:f64 = 0.1; //tasa de mutación
    let generations = 30; //número de generaciones
    let torunament_size = 5; //tamaño del torneo para la selección de individuos
    let best_tournament_size = 3; //tamaño del torneo para seleccionar a los mejores individuos
    let selection_percentage = 0.5; //porcentaje de la población seleccionada para cruzar y mutar
    let cross_over_probabiliy = 0.9; //tasa de cruce

    let seeds = vec![1,2,3,4,5,6,7,8,8,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30 ]; //semillas para la generación de números aleatorios
    let mut unsucceful_seeds= seeds.clone(); //semillas no exitosas hasta que se demuestre lo contrario
    let mut succeful_seeds: Vec<usize> = Vec::new(); //semillas exitosas
    let mut bests: Vec<u32>=Vec::new();

    for seed in seeds {
        println!("===============================");
        let mut rng = StdRng::seed_from_u64(seed as u64); //se crea un generador de números aleatorios
        let mut generation_succes:usize = 0; //parametro de exito de la generacion
        let mut key:bool = true;

        //Elementos de guardado de informacion
        let mut atacks_by_gen: Vec<Vec<u32>> = Vec::new();

        println!("Algoritmo Genético para el problema de las 8 reinas_Version permutaciones");
        println!("Semilla{}:", seed);
        println!("===============================");
        println!("SE INICIA EL ALGORITMO GENÉTICO");
        println!("===============================");
        println!("Generando población inicial...");
        println!("Fase 1: Generación de la población inicial y evaluacion de aptitud");
        //En esta parte del algoritmo se genera la poblacion inicial de manera aleatoria
        //cada individuo es representado como una matriz de 8x8 donde cada entrada de la matriz
        //representa una casilla del tablero de ajedrez, y el valor de cada entrada indica si es que hay una reina
        //en esa casilla o no. Por ejemplo, un valor de 1 indicaria que hay una reina en esa casilla, mientras
        // que un valor de 0 indicaria que no hay una reina en esa casilla.
        let mut initial_population:Vec<Individual>=initial_population(population_size,&mut rng);
        let mut generation = 1; //contador de generaciones
        
        while generation<=generations{
            println!("Generación: {}", generation);
            println!("Fase 2: Selección");
            //En esta parte del algoritmo se seleccionan a un porcentaje aleatorio de la población inicial
            // se realiza un torneo de selección, donde se seleccionan al azar un número determinado de 
            //individuos de la población inicial hasta completar una cantidad par de parejas

            let mut parents_selected:Vec<Individual> = Vec::new(); //padres seleccionados
            let mut tournament:usize = 0; //contador de torneos realizados
            while parents_selected.len()<(population_size as f64 * selection_percentage) as usize {
                // Se seleccionan índices aleatorios de la población inicial para el torneo de selección.
                let index_selected = index::sample(&mut rng, population_size, torunament_size).into_vec(); 
                //println!("--->Índividuos seleccionados para torneo: \n{:?}", index_selected);
                let mut selected_population: Vec<Individual> = (0..torunament_size)
                                                                .map(|i| initial_population[index_selected[i]]
                                                                .clone())
                                                                .collect(); // individuos seleccionados para cruzar y mutar

                let index_selected_cross:Vec<usize> = orderin_sols(&mut selected_population);
                let mut aux_index1:Vec<usize> = Vec::new();
                for i in 0..best_tournament_size {
                    parents_selected.push(selected_population[i].clone());
                    aux_index1.push(index_selected[index_selected_cross[i]]);
                }
                //println!("--->Índices ordenados por aptitud: \n{:?}", aux_index1);
                tournament+=1;
            }
            println!(">Número de torneos realizados: {}", tournament);
            if parents_selected.len()%2!=0{
                parents_selected.pop(); 
                //en lugar de agregar uno, se quita el ultimo, manteniendo la cantidad de parejas par para el cruce

            }

            //println!(">Padres seleccionados para cruzar y mutar: {:?}", parents_selected.len());
            println!("================================");
            println!("Fase 4: Cruza");
            // se recorre la lista de padres seleccionados de dos en dos, 
            // y se realiza el cruce entre cada pareja de padres para generar un nuevo individuo hijo. 
            // El cruce se realiza tomando aleatoriamente cada columna del tablero del padre 1 o del padre 2 
            // para formar el tablero del hijo.
            let mut childs:Vec<Individual> = Vec::new();
            for i in (0..parents_selected.len()).step_by(2){

                // Se genera un valor booleano aleatorio para decidir si se realiza el cruce o no
                // devuelve true con una probabilidad igual a cross_over_probabiliy 
                // y false con probabilidad 1 - cross_over_probabiliy.
                let parent1 = &parents_selected[i];
                let parent2 = &parents_selected[i+1];
                let flag = rng.gen_bool(cross_over_probabiliy);
                if flag{
                    childs.push(cross(&parent1, &parent2, &mut rng));
                } else {
                    if parent1.atack < parent2.atack {
                        childs.push(parent1.clone());
                    }
                    else{
                        childs.push(parent2.clone());
                    }   
                }
            }
            println!("Se han generado {} hijos a partir de los padres seleccionados", childs.len());
            println!("================================");
            println!("Fase 5: Mutación");

            // se recorre la lista de hijos generados y se realiza la mutación en cada hijo con una probabilidad determinada.
            for child in &mut childs{
                mutation(child, mutation_rate, &mut rng); //se muta a los hijos
                repair_solutions(child, &mut rng);
            }

            println!("================================");
            println!("Fase 6: Reemplazo");
            let mut current_population = initial_population.clone();
            current_population.extend(childs);
            orderin_sols(&mut current_population);
            current_population.truncate(population_size);
            initial_population = current_population.clone();
            atacks_by_gen.push(initial_population.iter().map(|f| f.atack).collect());
            if initial_population[0].atack==0 && key == true {
                generation_succes=generation;
                succeful_seeds.push(seed);
                unsucceful_seeds.retain(|&x| x != seed);
                key=false;
                //break; condicion de paro del algoritmo, desactivada salvo que se necesite.
            }
            println!(">Población actualizada con los hijos generados y mutados");
            generation+=1;
        }

        print!("===============================");
        println!("\nMejor solución encontrada:");
        print_board(&initial_population[0].board);
        bests.push(initial_population[0].atack);
        println!("Número de ataques entre reinas: {}", initial_population[0].atack);
        println!("Generacion de exito: {:}",generation_succes);  
        
        // Guardar información en un archivo CSV
        //informacion para observar el comportamiento de una semilla particular
        // Guardar información en un archivo CSV

        if seed==15{ //la mediana de 1 a 30 es 15, dependiendo este valor debe ser cambiado
            let filename = format!("atacks_by_gen_matrix_seed_{}.csv", seed);    
            let mut file = File::create(&filename).expect("No se pudo crear el archivo CSV");
            
            if !atacks_by_gen.is_empty() {
                let headers: Vec<String> = (1..=atacks_by_gen.len()).map(|g| format!("Gen_{}", g)).collect();
                writeln!(file, "{}", headers.join(",")).expect("Error al escribir los encabezados");

                let num_individuals = atacks_by_gen[0].len();
                for ind_idx in 0..num_individuals {
                    let row_values: Vec<String> = (0..atacks_by_gen.len()).map(|gen_idx| atacks_by_gen[gen_idx][ind_idx]
                                                    .to_string()).collect();
                    writeln!(file, "{}", row_values.join(",")).expect("Error al escribir la línea");
                }
            }
            println!("Los datos de los ataques por generación fueron guardados exitosamente en '{}'", filename);
        }
    }
    println!("===============================");
    println!("Semillas exitosas:{:?}", succeful_seeds);
    println!("semillas no exitosas:{:?}", unsucceful_seeds);
    println!("Mejores resultados (ataques) por semilla: {:?}", bests);

   // Guardar los resultados globales en un archivo CSV
    let mut file_resumen = File::create("resumen_semillas_matrix.csv").expect("No se pudo crear el archivo CSV");
    writeln!(file_resumen, "Semillas exitosas,Semillas no exitosas,Mejores resultados (ataques)").expect("Error al escribir los encabezados");
    let max_len = succeful_seeds.len().max(unsucceful_seeds.len()).max(bests.len());
    for i in 0..max_len {
        let succ = if i < succeful_seeds.len() { succeful_seeds[i].to_string() } else { String::new() };
        let unsucc = if i < unsucceful_seeds.len() { unsucceful_seeds[i].to_string() } else { String::new() };
        let best = if i < bests.len() { bests[i].to_string() } else { String::new() };
        writeln!(file_resumen, "{},{},{}", succ, unsucc, best).expect("Error al escribir la línea");
    }
    println!("Resultados globales exportados a 'resumen_semillas_matrix.csv'");
}

fn initial_population(population_size: usize, rng: &mut StdRng) -> Vec<Individual> {
    let mut population = Vec::new();
      
    for _ in 0..population_size {
        let mut positions: Vec<Vec<i32>> = Vec::new(); // se crea un vector de vectores para almacenar las posiciones de las reinas en cada individuo
        let mut tablero = vec![vec![0; 8]; 8];
        for i in 0..8 {
            // el ciclo recorre cada columna del tablero
            let queen_position = rng.gen_range(0..8); // se genera una posición aleatoria para la reina en la columna actual
            tablero[queen_position][i] = 1;
            positions.push(vec![queen_position as i32, i as i32]); // se almacena la posición de la reina en el vector de posiciones 
        }
        //Contar el número de ataques entre las reinas
        let mut atack = 0;
        let mut count_positions : usize= 0;
        for pos_a in &positions{
            for pos in &positions[count_positions+1..]{
                // Verificar si las reinas están en la misma fila
                //println!("Posición A: {:?}, Posición B: {:?}", pos_a, pos);
                if pos_a[0] == pos[0] {
                    atack += 1;
                }
                else{
                // Verificar si las reinas están en la misma diagonal
                    if (pos_a[0] - pos[0]).abs() == (pos_a[1] - pos[1]).abs() {
                        atack += 1;
                    }
                }
            }
            count_positions+=1;
        }
        population.push(Individual {
        board: tablero,
        atack: atack as u32,});
    }
    population
}
fn orderin_sols(solutions:&mut Vec<Individual>)-> Vec<usize>{
    let mut sol_indx: Vec<usize> = (0..solutions.len()).collect(); 
    //println!("Índices originales: {:?}", sol_indx);
    let atacks_by_sol:Vec<u32>= solutions[0..solutions.len()].iter().map(|x| x.atack).collect();
    //println!("atacks por solucion: {:?}", atacks_by_sol);
    sol_indx.sort_by(|&i, &j| atacks_by_sol[i].cmp(&atacks_by_sol[j]));
    let sols_aux = solutions.clone();
    for i in 0..solutions.len() {
        solutions[i] = sols_aux[sol_indx[i]].clone();
    }
    sol_indx
}
fn fitness(individual: &mut Individual) -> i32 {
    let mut positions: Vec<Vec<i32>> = Vec::new();
    let fit: i32 = 0;
    for i in 0..8 {
        for j in 0..8 {
            if individual.board[i][j] == 1 {
                positions.push(vec![i as i32, j as i32]); // se almacena la posición de la reina en el vector de posiciones 
            }
        }
    }
    let mut atack = 0;
    let mut count_positions : usize= 0;
    for pos_a in &positions{
        for pos in &positions[count_positions+1..]{
            // Verificar si las reinas están en la misma fila
            //println!("Posición A: {:?}, Posición B: {:?}", pos_a, pos);
            if pos_a[0] == pos[0] {
                atack += 1;
            }
            else{
            // Verificar si las reinas están en la misma diagonal
                if (pos_a[0] - pos[0]).abs() == (pos_a[1] - pos[1]).abs() {
                    atack += 1;
                }
            }
        }
        count_positions+=1;
    }
    individual.atack = atack as u32;
    fit
}
fn cross(parent1:&Individual, parent2:&Individual, rng: &mut StdRng) -> Individual {
    let mut board_c = parent1.board.clone(); 
    /*println!("Padre 1:\n");
    print_board(&parent1.board);
    println!("Padre 2:\n");
    print_board(&parent2.board);*/
    for i in 0..8{
        let flag = rng.gen_range(0..2);
        for j in 0..8{    
            if flag == 0{
                board_c[j][i] = parent1.board[j][i];
            }
            else{
                board_c[j][i] = parent2.board[j][i];
            }
        }
    }
    /*println!("Hijo:\n");
    print_board(&board_c);*/
    let mut child = Individual{
        board : board_c,
        atack : 0,        
    };
    fitness(&mut child);
    child
}
fn mutation(child: &mut Individual,mutation_rate:f64, rng: &mut StdRng) {
    let flag = rng.gen_bool(mutation_rate);
    if flag{
        let col = rng.gen_range(0..8);// se sabe que en cada columna solo hay una reina
        for row in 0..8{
            if child.board[row][col]==1{
                child.board[row][col]=0; // se elimina la reina de su posición actual
                let mut  row_2 = rng.gen_range(0..8); 
                let mut  col_2 = rng.gen_range(0..8);
                while child.board[row_2][col_2]!=0 {
                    // se genera una nueva posición aleatoria para la reina hasta encontrar una posición vacía
                    row_2 = rng.gen_range(0..8);
                    col_2 = rng.gen_range(0..8);                    
                }
                child.board[row_2][col_2]=1;
                break;
            }
        }
    }
    fitness(child);
}
fn print_board(board: &Vec<Vec<u8>>) {
    for row in board {
        for cell in row {
            if *cell == 1 {
                print!("Q  ");
            } else {
                print!(".  ");
            }
        }
        println!();
    }
}
fn repair_solutions(individual: &mut Individual, rng: &mut StdRng) {
    let mut queens_positions: Vec<Vec<usize>> = Vec::new(); // vector para almacenar las posiciones de las reinas
    let mut empty_positions: Vec<Vec<usize>> = Vec::new(); // vector para almacenar las posiciones vacías

    for i in 0..8 {
        for j in 0..8 {
            if individual.board[i][j] == 1 {
                queens_positions.push(vec![i,j]);
            } else {
                empty_positions.push(vec![i,j]);
            }
        }
    }

    for col in 0..8 {
        let mut queens_in_col: Vec<usize> = Vec::new();

        for row in 0..8 {
            if individual.board[row][col] == 1 {
                queens_in_col.push(row);
            }
        }

        // Si hay más de una reina en la columna → eliminar extras
        while queens_in_col.len() > 1 {
            let idx = rng.gen_range(0..queens_in_col.len());
            let row = queens_in_col.remove(idx);

            individual.board[row][col] = 0;

            // actualizar vector global
            if let Some(pos) = queens_positions.iter().position(|x| x[0]==row && x[1]==col) {
                queens_positions.remove(pos);
            }

            empty_positions.push(vec![row,col]);
        }
    }

    if queens_positions.len() > 8 {
        let remove = queens_positions.len() - 8;
        for _ in 0..remove {
            let index = rng.gen_range(0..queens_positions.len());
            let pos = queens_positions.remove(index);

            individual.board[pos[0]][pos[1]] = 0;
            empty_positions.push(pos);
        }
    }

    if queens_positions.len() < 8 {
        let add = 8 - queens_positions.len();

        for _ in 0..add {

            // elegir columna sin reina
            let mut col = rng.gen_range(0..8);

            // asegurar que la columna esté vacía
            let mut attempts = 0;
            while (0..8).any(|row| individual.board[row][col] == 1) && attempts < 10 {
                col = rng.gen_range(0..8);
                attempts += 1;
            }

            // si encontramos columna válida
            for row in 0..8 {
                if individual.board[row][col] == 0 {
                    individual.board[row][col] = 1;
                    queens_positions.push(vec![row,col]);

                    if let Some(pos) = empty_positions.iter().position(|x| x[0]==row && x[1]==col) {
                        empty_positions.remove(pos);
                    }

                    break;
                }
            }
        }
    }

    fitness(individual);
}