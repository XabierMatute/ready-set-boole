/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:23:49 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 20:09:14 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![cfg(test)]

mod tests {
    use crate::ex10::curve::map;
    use std::collections::HashSet;

    // Complete test, will probably cause the pc to restart
    // #[test]
    // fn test_map_complete() {
    //     let mut testmap : HashMap<(u16, u16), f64> = HashMap::new();

    //     for x in 0..=u16::MAX {
    //         for y in 0..=u16::MAX {
    //             println!("Testing map for ({}, {})", x, y);
    //             let result = map(x, y);
    //             assert!(result >= 0.0 && result <= 1.0, "Mapped value for ({}, {}) is out of range: {}", x, y, result);
    //             let key = (x, y);
    //             if let Some(&prev_result) = testmap.get(&key) {
    //                 assert_eq!(prev_result, result, "Mapped value for ({}, {}) changed: {} != {}", x, y, prev_result, result);
    //             } else {
    //                 testmap.insert(key, result);
    //             }
    //         }
    //     }
    // }

    #[test]
    fn test_map_properties() {
        // Prueba casos límite
        let edge_cases = [
            (0, 0),
            (0, u16::MAX),
            (u16::MAX, 0),
            (u16::MAX, u16::MAX),
            (12345, 54321),
        ];

        for &(x, y) in &edge_cases {
            let result = map(x, y);
            assert!(result >= 0.0 && result <= 1.0, "Mapped value for ({}, {}) is out of range: {}", x, y, result);
        }
    }

    #[test]
    fn test_map_uniqueness_sample() {
        let mut results = HashSet::new();
        // Prueba una muestra aleatoria para verificar la unicidad
        // (con una muestra más pequeña para que el test sea rápido)
        for x in (0..=u16::MAX).step_by(1024) { // Salta de 1024 en 1024
            for y in (0..=u16::MAX).step_by(1024) {
                let result = map(x, y);
                // Comprueba que cada resultado es único en la muestra
                assert!(results.insert(result.to_bits()), "Collision detected for ({}, {}), result: {}", x, y, result);
            }
        }
    }

}