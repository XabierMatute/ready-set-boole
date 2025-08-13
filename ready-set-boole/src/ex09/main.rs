/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 19:34:56 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex09::set_evaluation::eval_set;
use std::env;

pub fn main() {
    println!("Ready, Set, Evaluation!");
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <formula> <set1> <set2> ...", args[0]);
        return;
    }
    let formula_str = &args[1];
    let set_strs = &args[2..];
    let sets: Vec<Vec<i32>> = set_strs.iter()
        .map(|s| {
            s.split_whitespace()
                .map(|num| num.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
                .expect("Failed to parse set elements")
        })
        .collect();
    let result = eval_set(formula_str, sets.clone());
    println!("Original formula: \"{}\"", formula_str);
    println!("Sets:            {:?}", sets);
    println!("Evaluation result: {:?}", result);
}

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 18:25:34 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// use ready_set_boole::ex09::set_evaluation::eval_set;

// pub fn main() {
//     println!("Ready, Set, Evaluation! (Hardcoded Example)");

//     // Define la fórmula y los conjuntos directamente en el código.
//     // Fórmula: !(A & B)
//     let formula_str = "AB&!";
//     // Conjuntos: A = {1, 2}, B = {2, 3}
//     let sets = vec![
//         vec![1, 2], // Corresponde a 'A'
//         vec![2, 3], // Corresponde a 'B'
//     ];

//     // Llama a la función de evaluación.
//     // Se clona `sets` para poder imprimirlo después de que `eval_set` tome posesión.
//     let result = eval_set(formula_str, sets.clone());

//     // Imprime los resultados.
//     println!("Original formula: \"{}\"", formula_str);
//     println!("Sets:            {:?}", sets);
//     println!("Evaluation result: {:?}", result);
// }