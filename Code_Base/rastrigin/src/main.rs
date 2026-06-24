extern crate rand; 
//extern crate rand_derive;  

use crate::rand::SeedableRng;
use crate::rand::Rng;
use rand::rngs::StdRng;

use std::time::{Instant};
use core::num;
use std::collections::btree_map::RangeMut;
use std::fs::File; 
use std::io::{self, BufReader, BufRead, Write}; 
use std::f64;
use std::vec;
use std::vec::Vec;
use std::iter::Iterator;
use std::f64::consts::PI;
use std::env;

#[derive(Clone, Debug)]
struct Individual{
    solution: Vec<f64>,
}
#[derive(Debug, Clone)]
struct Individual_coded{
    solution: Vec<usize>,
}
#[derive(Debug)]
struct ParametrosAG {
    pop_size: usize,
    generaciones: usize,
    p_cruza: f64,
    p_mutacion: f64,
    p_torneo: f64,
    semilla_inicial: u64, // Es mejor pedirla directamente como u64
}
fn main() {
    let mut entrada = String::new();

    // 1. Menú inicial interactivo
    print!("Versión del algoritmo (1. Binario, 2. Real): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error");
    let version: i32 = entrada.trim().parse().unwrap_or(1);

    entrada.clear();
    print!("Problema a resolver (1. Esfera, 2. Rastrigin): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error");
    let opcion: i32 = entrada.trim().parse().unwrap_or(1);

    
    let parametros = capturar_parametros();

    
    println!("\nProblema {}", opcion);
    println!("Tamaño de población: {}", parametros.pop_size);
    println!("Generaciones: {}", parametros.generaciones);
    println!("P. cruza: {}", parametros.p_cruza);
    println!("P. mutación: {}", parametros.p_mutacion);
    println!("P. torneo (si aplica): {}", parametros.p_torneo);
    println!("Semilla (si aplica): {}\n", parametros.semilla_inicial);

    /* PARAMETROS FIJOS */
    let (lim_inf, lim_sup) = if opcion == 1 {
        (-10.0, 10.0) 
    } else {
        (-5.12, 5.12) 
    };

    let presicion: i32 = 3; 
    let l = encode_digits(&presicion, &lim_inf, &lim_sup); 

    let fitness: fn(&Individual) -> f64 = if opcion == 1 { fitness_f1 } else { fitness_f2 };

    let mut rng = StdRng::seed_from_u64(parametros.semilla_inicial);
    let mut fit_best_solution = f64::INFINITY;

    if version == 1 {
        // --- AG BINARIO ---
        let mut generacion_actual = 0;
        let mut pop = initial_population_binaria(&parametros.pop_size, &mut rng, &l);   
        ordering_pop_coded(&mut pop, &l, &lim_inf, &lim_sup, &fitness);
        let mut best_solution = pop[0].clone();
        
        fit_best_solution = fitness(&decode(&best_solution, &l, &lim_inf, &lim_sup));

        while generacion_actual <= parametros.generaciones {
            // IMPRIMIR RESULTADOS DE LA GENERACIÓN ACTUAL 
            let best_decoded = decode(&best_solution, &l, &lim_inf, &lim_sup);
            println!("Gen {}", generacion_actual);
            
            
            let formatted_sol: Vec<String> = best_decoded.solution.iter().map(|v| format!("{:.2}", v)).collect();
            println!("[{}] f(x) = {:.2}", formatted_sol.join(", "), fit_best_solution);

           
            if generacion_actual == parametros.generaciones { break; }

            
            let selected_parents = selection_binaria(&pop, &mut rng, &l, &lim_inf, &lim_sup, &fitness);
            let mut offspring_pop: Vec<Individual_coded> = Vec::new();
            
            for pareja in selected_parents.chunks(2) {
                if pareja.len() != 2 { break; } 
                let padre1 = pareja[0];
                let padre2 = pareja[1];
                
                let flip_cruza: bool = rng.gen_bool(parametros.p_cruza);
                if flip_cruza {
                    let (hijo1, hijo2) = crossover_binaria(&pop[padre1], &pop[padre2], &mut rng);
                    if rng.gen_bool(parametros.p_mutacion) { offspring_pop.push(mutation_binaria(&hijo1, &mut rng, &parametros.p_mutacion)); } else { offspring_pop.push(hijo1); }
                    if rng.gen_bool(parametros.p_mutacion) { offspring_pop.push(mutation_binaria(&hijo2, &mut rng, &parametros.p_mutacion)); } else { offspring_pop.push(hijo2); }
                } else {
                    if rng.gen_bool(parametros.p_mutacion) { offspring_pop.push(mutation_binaria(&pop[padre1], &mut rng, &parametros.p_mutacion)); } else { offspring_pop.push(pop[padre1].clone()); }
                    if rng.gen_bool(parametros.p_mutacion) { offspring_pop.push(mutation_binaria(&pop[padre2], &mut rng, &parametros.p_mutacion)); } else { offspring_pop.push(pop[padre2].clone()); }
                }
            }
            
            
            while offspring_pop.len() < parametros.pop_size { offspring_pop.push(best_solution.clone()); }
            while offspring_pop.len() > parametros.pop_size { offspring_pop.pop(); }

            ordering_pop_coded(&mut offspring_pop, &l, &lim_inf, &lim_sup, &fitness);
            let fit_best_offspring = fitness(&decode(&offspring_pop[0], &l, &lim_inf, &lim_sup));

            // ELITISMO Y REEMPLAZO
            if fit_best_offspring < fit_best_solution {
                pop = offspring_pop;
                best_solution = pop[0].clone();
                fit_best_solution = fit_best_offspring; 
            } else {
                pop = offspring_pop;
                let ultimo = pop.len() - 1; 
                pop[ultimo] = best_solution.clone();
            }
            
            generacion_actual += 1;
        }
        
    } else {
        // --- AG REAL ---
        let mut evaluaciones_realizadas = 0; 
        let mut generacion_actual = 0;
        let mut pop = initial_population_real(&parametros.pop_size, &mut rng, &lim_inf, &lim_sup);
        ordering_pop_real(&mut pop, &fitness);
        let mut best_solution = pop[0].clone();
        
        fit_best_solution = fitness(&best_solution);

        while generacion_actual <= parametros.generaciones {
         
            println!("Gen {}", generacion_actual);
            
            
            let formatted_sol: Vec<String> = best_solution.solution.iter().map(|v| format!("{:.2}", v)).collect();
            println!("[{}] f(x) = {:.2}", formatted_sol.join(", "), fit_best_solution);

            
            if generacion_actual == parametros.generaciones { break; }

            // CREAR DESCENDENCIA
            let selected_parents = selection_real(&mut pop, &mut rng, &parametros.pop_size, &mut evaluaciones_realizadas, &fitness);
            let mut offspring_pop: Vec<Individual> = Vec::new();
            
            for trio in selected_parents.chunks(3) {
                if trio.len() != 3 { break; } 
                let padre1 = trio[0];
                let padre2 = trio[1];
                let padre3 = trio[2]; 
                
                let flip_cruza: bool = rng.gen_bool(parametros.p_cruza);
                if flip_cruza {
                    let (mut hijo1, mut hijo2) = crossover_real(&pop[padre1], &pop[padre2], &pop[padre3], &mut rng, &lim_inf, &lim_sup);
                    if rng.gen_bool(parametros.p_mutacion) { offspring_pop.push(mutation_real(&mut hijo1, &mut rng, &lim_inf, &lim_sup)); } else { offspring_pop.push(hijo1); }
                    if rng.gen_bool(parametros.p_mutacion) { offspring_pop.push(mutation_real(&mut hijo2, &mut rng, &lim_inf, &lim_sup)); } else { offspring_pop.push(hijo2); }
                } else {
                    if rng.gen_bool(parametros.p_mutacion) { offspring_pop.push(mutation_real(&mut pop[padre1], &mut rng, &lim_inf, &lim_sup)); } else { offspring_pop.push(pop[padre1].clone()); }
                    if rng.gen_bool(parametros.p_mutacion) { offspring_pop.push(mutation_real(&mut pop[padre2], &mut rng, &lim_inf, &lim_sup)); } else { offspring_pop.push(pop[padre2].clone()); }
                }
            }
            
           
            while offspring_pop.len() < parametros.pop_size { offspring_pop.push(best_solution.clone()); }
            while offspring_pop.len() > parametros.pop_size { offspring_pop.pop(); }

            ordering_pop_real(&mut offspring_pop, &fitness);
            let fit_best_offspring = fitness(&offspring_pop[0]);
            
            
            if fit_best_offspring < fit_best_solution {
                pop = offspring_pop;
                best_solution = pop[0].clone(); 
                fit_best_solution = fit_best_offspring; 
            } else {
                pop = offspring_pop;
                let ultimo = pop.len() - 1; 
                pop[ultimo] = best_solution.clone();
            }
            
            generacion_actual += 1;
        }
    }
    println!("\nEjecución finalizada. Presione Enter para salir...");
    io::stdout().flush().unwrap();
    let mut pausa = String::new();
    io::stdin().read_line(&mut pausa).expect("Error al leer");
}




/*funciones de los geneticos */
fn fitness_f1(sol: &Individual) -> f64{
    let mut fit = 0.0; 
    for gen_value in sol.solution.iter(){
        fit += gen_value.powf(2.0);
    }
    fit
}
fn fitness_f2(sol: &Individual) -> f64{
    let mut fit = 0.0; 
    for gen_value in sol.solution.iter(){
        fit += gen_value.powf(2.0) - 10.0 * (2.0 * PI * gen_value).cos() ;
    }
    fit +=100.0;
    fit
}
fn encode_digits(eps:&i32,
            l_inf:&f64,
            l_sup:&f64) -> usize {  
    /*** Valores necesarios para codificar ***/
    let l_dif: f64 = l_sup - l_inf;
    let log_dif:f64 = (l_dif * 10.0_f64.powi(*eps) ).log2();
    let l = log_dif.ceil(); 
    l as usize
}
fn decode(indiv_coded: &Individual_coded, digits:&usize, l_inf:&f64, l_sup:&f64)->Individual{
    let codificacion = indiv_coded.solution.clone();
    let mut counter_digits:usize = 0;// te ubica en la posicion del vector sum_digits
    let mut counter_positions:usize = 0; //cuenta los digitos
    let mut sum_digits:Vec<f64> = vec![0.0;10];//son 10 variables reales
    let mut counter_digits_incoded:usize = *digits;
    //la variable anterior servira para revisar de l en l digitos de derecha a
    //izquierda las codificaciones 
    for _ in 0..codificacion.len(){
        ////println!("{:?}",codificacion[counter_digits_incoded - counter_positions - 1]);
        let i = codificacion[counter_digits_incoded - counter_positions - 1];
        
        if i==1 {
            sum_digits[counter_digits] += 2.0_f64.powf(counter_positions as f64);

        }
        counter_positions += 1;
        if counter_positions==*digits{
            counter_digits += 1;
            counter_positions = 0;
            counter_digits_incoded += *digits;
           // //println!("{:?}", counter_digits_incoded);

        }
    }
    let ldif = l_sup - l_inf;
    
    for idx in 0..sum_digits.len(){
        let valor_real = l_inf + (sum_digits[idx] * ldif)/(2.0_f64.powf(*digits as f64) - 1.0);
        
        // El redondeo a 3 decimales 
        sum_digits[idx] = (valor_real * 1000.0).trunc() / 1000.0;
    }
    let indiv_decoded = Individual{
        solution: sum_digits
    };
    indiv_decoded
}
fn initial_population_real(pop_size:&usize, 
                            rng :&mut StdRng, 
                            lim_inf:&f64, 
                            lim_sup:&f64)->Vec<Individual>{
    let mut pop:Vec<Individual> = Vec::new();
    for _ in 0..*pop_size{
        let mut sol:Vec<f64> = Vec::new();
        for _ in 0..10{ //la solucion es un vector solo de 10 variables 
            sol.push((rng.gen_range(*lim_inf..*lim_sup) * 1000.0).trunc() / 1000.0);
        }
        let indiv = Individual{
            solution: sol
        };
        pop.push(indiv);
    }
    pop
}
fn initial_population_binaria(pop_size:&usize,
                              rng :&mut StdRng, 
                              l:&usize)->Vec<Individual_coded>{
    let mut pop:Vec<Individual_coded> = Vec::new();
    for _ in 0..*pop_size{
        let mut sol:Vec<usize> = Vec::new();
        for _ in 0..(*l * 10){//se requiren 10 variables pero cada una se codifica con l digitos
            sol.push(rng.gen_range(0..2));
        }
        let indiv = Individual_coded{
            solution: sol
        };
        pop.push(indiv);
    }
    pop
}
fn selection_binaria(population:&Vec<Individual_coded>, 
                    rng:&mut StdRng,
                    l:&usize,
                    lim_inf:&f64,
                    lim_sup:&f64,
                    fitness:&fn(&Individual)->f64
                    )-> Vec<usize>{
                    //regresa los indices de los individuos de la poblacion que han sido elegidos
                    //para cruzar
    //println!("En esta versión no aplica la Probabilidad de Torneo");
    let mut fit_vec = Vec::new();
    for indiv in population{
        let indiv_decoded = decode(indiv, &l, &lim_inf, &lim_sup);
        let fit = fitness(&indiv_decoded);
        fit_vec.push(fit);
    } 
    let min_fitness = fit_vec.iter()
                    .map(|fit| *fit)
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0);  

    fit_vec = fit_vec.iter()
                .map(|f| f + min_fitness.abs())
                .collect();

    let mean_fit = fit_vec.iter().sum::<f64>() / fit_vec.len() as f64;
    let mut expected_values = Vec::new();
    for fit in fit_vec{
        expected_values.push(fit/mean_fit);
    }

    let expected_sum = expected_values.iter().sum::<f64>();
    let mut selectec_pop = Vec::new();
    for _ in 0..population.len(){
        let r = rng.gen_range(0.0..expected_sum);
        let mut sum = 0.0;
        for idx in 0..population.len(){
            sum += expected_values[idx];
            if sum >= r{
                selectec_pop.push(idx);
                break;
            }
        }
    }
    selectec_pop
}
fn crossover_binaria(parent1:&Individual_coded, 
                    parent2:&Individual_coded,
                    rng:&mut StdRng)->(Individual_coded, Individual_coded){
    let crossover_point = rng.gen_range(0..parent1.solution.len());
    let mut child1_solution = parent1.solution.clone();
    let mut child2_solution = parent2.solution.clone();
    for i in crossover_point..parent1.solution.len(){
        child1_solution[i] = parent2.solution[i];
        child2_solution[i] = parent1.solution[i];
    }
    let child1 = Individual_coded{
        solution: child1_solution
    }; 
    let child2 = Individual_coded{
        solution: child2_solution
    };
    (child1, child2)
}
fn mutation_binaria(indiv_coded: &Individual_coded, 
                    rng:&mut StdRng, 
                    p_mutacion:&f64)->Individual_coded{
    let mut sol = indiv_coded.solution.clone();
    for i in 0..sol.len(){
        if rng.gen_range(0.0..1.0) < *p_mutacion{
            sol[i] = 1 - sol[i];
        } 
    }
    let indiv = Individual_coded{
        solution: sol
    };
    indiv
}
fn selection_real(population: &mut Vec<Individual>,
                  rng :&mut StdRng, 
                  pop_size:&usize,
                  evals:&mut usize,
                  fitness:&fn(&Individual)->f64) -> Vec<usize>{
    let mut selected_parents: Vec<usize> = Vec::new();
    let parents_needed: i32 = (*pop_size as i32 ) * 3 / 2; 
    //se necesitan 3 padres para generar dos hijos.
    //dadas las restricciones de colinealidad se prohibira que haya 
    //padres repetidos por cada 3 seleccionados, es decir, si se selecciona un padre,
    // ese padre no podrá ser seleccionado nuevamente hasta que se hayan seleccionado 2 padres más. 
    let mut selected_trio:Vec<usize> = Vec::new();
    while selected_parents.len()  < parents_needed as usize {
        
        let mut candidatos: Vec<Individual> = Vec::new();
        for _ in 0..5 {//tamaño del torneo
            let candidato = rng.gen_range(0..population.len() as i32) as usize; 
            candidatos.push(population[candidato].clone());
        }
        
        let mut candidatos_fitness: Vec<(f64, usize)> = Vec::new();
        for (i,indiv) in candidatos.iter().enumerate(){
            let fit = fitness(&indiv);
            *evals+=1; // Sumamos la evaluación del candidato
            candidatos_fitness.push((fit, i));
        }
        candidatos_fitness.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let ganador_idx = candidatos_fitness[0].1;
        let ganador = &candidatos[ganador_idx];
        let ganador_idx_in_pop = population.iter().position(|x| x.solution == ganador.solution).unwrap();
        if !selected_trio.contains(&ganador_idx_in_pop) {
            selected_trio.push(ganador_idx_in_pop);
        }
        if selected_trio.len() == 3 {
            for idx in &selected_trio {
                selected_parents.push(*idx);
            }
            selected_trio.clear();
        }
    }
    selected_parents
}
fn crossover_real(parent1: &Individual, 
    parent2: &Individual, 
    parent3: &Individual,
    rng: &mut StdRng,
    lim_inf: &f64,
    lim_sup: &f64
) -> (Individual, Individual) {
    
    let n = parent1.solution.len(); // D = 10 variables
    
    // --- FASE 1: Distancia y Vector de Dirección (e0) ---
    let mut d = vec![0.0; n];
    let mut e0 = vec![0.0; n];
    let mut norm_d_sq = 0.0;

    for i in 0..n {
        d[i] = parent2.solution[i] - parent1.solution[i];
        norm_d_sq += d[i].powi(2);
    }
    let norm_d = norm_d_sq.sqrt();

    // Vector unitario e0 (línea principal de búsqueda)
    if norm_d > 0.0 {
        for i in 0..n {
            e0[i] = d[i] / norm_d;
        }
    }

    // Calcular la distancia D del parent3 a la línea que conecta parent1 y parent2
    let mut v = vec![0.0; n];
    let mut dot_v_e0 = 0.0;
    for i in 0..n {
        v[i] = parent3.solution[i] - parent1.solution[i];
        dot_v_e0 += v[i] * e0[i];
    }

    let mut dist_sq = 0.0;
    for i in 0..n {
        let ortogonal = v[i] - (dot_v_e0 * e0[i]);
        dist_sq += ortogonal.powi(2);
    }
    let d_distance = dist_sq.sqrt(); 

    
    let sigma_eta = 0.35 / (n as f64).sqrt();
    let std_dev_eta = d_distance * sigma_eta;

    let mut t = vec![0.0; n];
    for i in 0..n {
        
        let u1: f64 = rng.gen_range(0.000001..1.0); 
        let u2: f64 = rng.gen_range(0.0..1.0);
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
        
        
        t[i] = z * std_dev_eta;
    }

    
    let mut dot_t_e0 = 0.0;
    for i in 0..n {
        dot_t_e0 += t[i] * e0[i];
    }
    for i in 0..n {
        t[i] = t[i] - (dot_t_e0 * e0[i]);
    }

    let u1_xi: f64 = rng.gen_range(0.000001..1.0);
    let u2_xi: f64 = rng.gen_range(0.0..1.0);
    let xi_z = (-2.0 * u1_xi.ln()).sqrt() * (2.0 * PI * u2_xi).cos();
    let xi = xi_z * 0.25;

    for i in 0..n {
        t[i] = t[i] + (xi * d[i]);
    }

    
    let mut child1_sol = vec![0.0; n];
    let mut child2_sol = vec![0.0; n];

    for i in 0..n {
        let mean = (parent1.solution[i] + parent2.solution[i]) / 2.0;
        let c1_val = mean + t[i];
        let c2_val = mean - t[i];

        
        child1_sol[i] = if c1_val > *lim_sup { *lim_sup } else if c1_val < *lim_inf { *lim_inf } else { c1_val };
        child2_sol[i] = if c2_val > *lim_sup { *lim_sup } else if c2_val < *lim_inf { *lim_inf } else { c2_val };
    }

    (Individual { solution: child1_sol }, Individual { solution: child2_sol })
}
fn mutation_real( indiv: &mut Individual,
                 rng :&mut StdRng,
                 lim_inf: &f64, 
                 lim_sup: &f64)->Individual{
    let mut indiv_2 = indiv.clone();
    let k = rng.gen_range(0..indiv_2.solution.len());
    indiv_2.solution[k] = rng.gen_range(*lim_inf..*lim_sup);
    indiv_2
}

fn ordering_pop_coded(population: &mut Vec<Individual_coded>,
                l:&usize,
                lim_inf:&f64,
                lim_sup:&f64,
                fitness:&fn(&Individual)->f64) {
    let mut fit_vec: Vec<(f64, usize)> = Vec::new();
    for (i, indiv) in population.iter().enumerate() {
        let indiv_decoded = decode(indiv, l, lim_inf, lim_sup);
        let fit = fitness(&indiv_decoded);
        fit_vec.push((fit, i));
    }
    fit_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut sorted_population = Vec::new();
    for (_, original_idx) in fit_vec {
        sorted_population.push(population[original_idx].clone()); 
    }
    *population = sorted_population;
}
fn ordering_pop_real(population: &mut Vec<Individual>,
                    fitness:&fn(&Individual)->f64){
    let mut fit_vec: Vec<(f64, usize)> = Vec::new();
    for (i,indiv) in population.iter().enumerate(){
        let fit = fitness(&indiv);
        fit_vec.push((fit, i));
    }
    fit_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut sorted_population = Vec::new();
    for (_, original_idx) in fit_vec {
        sorted_population.push(population[original_idx].clone()); 
    }
    *population = sorted_population;
}
fn capturar_parametros() -> ParametrosAG {
    let mut entrada = String::new();
    //println!("\n--- Configuración del Algoritmo Genético ---");
    print!("Tamaño de población: ");
    io::stdout().flush().unwrap();
    entrada.clear();
    io::stdin().read_line(&mut entrada).expect("Error");
    let pop_size: usize = entrada.trim().parse().expect("Error");

    print!("Número de generaciones: ");
    io::stdout().flush().unwrap();
    entrada.clear();
    io::stdin().read_line(&mut entrada).expect("Error");
    let generaciones: usize = entrada.trim().parse().expect("Error");

    print!("Probabilidad de cruza (0.0 a 1.0): ");
    io::stdout().flush().unwrap();
    entrada.clear();
    io::stdin().read_line(&mut entrada).expect("Error");
    let p_cruza: f64 = entrada.trim().parse().expect("Error");

    print!("Probabilidad de mutación (0.0 a 1.0): ");
    io::stdout().flush().unwrap();
    entrada.clear();
    io::stdin().read_line(&mut entrada).expect("Error");
    let p_mutacion: f64 = entrada.trim().parse().expect("Error");

    print!("Probabilidad de torneo: ");
    io::stdout().flush().unwrap();
    entrada.clear();
    io::stdin().read_line(&mut entrada).expect("Error");
    let p_torneo: f64 = entrada.trim().parse().expect("Error");

    print!("Semilla inicial: ");
    io::stdout().flush().unwrap();
    entrada.clear();
    io::stdin().read_line(&mut entrada).expect("Error");
    let semilla_inicial: u64 = entrada.trim().parse().expect("Error"); // Directamente a u64

    // Empaquetamos y regresamos los valores
    ParametrosAG {
        pop_size,
        generaciones,
        p_cruza,
        p_mutacion,
        p_torneo,
        semilla_inicial,
    }
}