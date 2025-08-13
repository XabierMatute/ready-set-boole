/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:23:49 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 20:12:35 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![cfg(test)]

mod tests {
    use crate::ex10::curve::map;
    use crate::ex11::inverse_function::reverse_map;

    #[test]
    fn test_reverse_map_roundtrip_edges() {
        let test_cases = [
            (0, 0),
            (u16::MAX, u16::MAX),
            (0, u16::MAX),
            (u16::MAX, 0),
            (12345, 54321),
            (42, 1337),
        ];

        for &(x, y) in &test_cases {
            let mapped_value = map(x, y);
            let (decoded_x, decoded_y) = reverse_map(mapped_value);
            assert_eq!((x, y), (decoded_x, decoded_y), "Round-trip failed for ({}, {})", x, y);
        }
    }

    #[test]
    fn test_reverse_map_roundtrip_random() {
        use rand::Rng;
        let mut rng = rand::rng();

        for _ in 0..1000 { // Probamos 1000 puntos aleatorios
            let x: u16 = rng.random();
            let y: u16 = rng.random();

            let mapped_value = map(x, y);
            let (decoded_x, decoded_y) = reverse_map(mapped_value);
            assert_eq!((x, y), (decoded_x, decoded_y), "Round-trip failed for random point ({}, {})", x, y);
        }
    }

    #[test]
    #[should_panic(expected = "Input must be in the range [0, 1]")]
    fn test_reverse_map_panics_below_zero() {
        reverse_map(-0.1);
    }

    #[test]
    #[should_panic(expected = "Input must be in the range [0, 1]")]
    fn test_reverse_map_panics_above_one() {
        reverse_map(1.1);
    }
}