/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/05 11:55:01 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/11 11:32:25 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[cfg(test)]
mod tests {
    use crate::ex00::adder::adder;

    fn test_adder(a: u32, b: u32) -> u32 {
        let optained_result = adder(a, b);
        let expected_result = a.wrapping_add(b);
        println!("Testing adder with {} + {} = {}", a, b, optained_result);
        assert_eq!(optained_result, expected_result, 
            "Expected {} + {} to equal {}, but got {}", a, b, expected_result, optained_result);
        optained_result
    }


    #[test]
    fn test_adder_zero() {
        test_adder(0, 0);
        test_adder(0, 1);
        test_adder(1, 0);
        test_adder(0, 100);
        test_adder(100, 0);
        test_adder(0, 123456789);
        test_adder(123456789, 0);
        test_adder(0, u32::MAX);
        test_adder(u32::MAX, 0);
        test_adder(0, 1_000_000);
        test_adder(1_000_000, 0);
        test_adder(0, 42);
        test_adder(42, 0);
    }

    #[test]
    fn test_adder_basic() {
        test_adder(1, 1);
        test_adder(2, 3);
        test_adder(10, 20);
        test_adder(100, 200);
        test_adder(123456789, 987654321);
        test_adder(42, 42);
        test_adder(1_000_000, 1_000_000);
        test_adder(1_000_000, 42);
        test_adder(42, 1_000_000);
    }

    #[test]
    fn test_adder_large_numbers() {
        test_adder(u32::MAX, u32::MAX);
        test_adder(u32::MAX, 1);
        test_adder(1, u32::MAX);
        test_adder(u32::MAX - 1, u32::MAX - 1);
        test_adder(u32::MAX - 1, 1);
        test_adder(1, u32::MAX - 1);
    }

    #[test]
    fn test_adder_random_numbers() {
        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..100 {
            let a: u32 = rng.random_range(0..=u32::MAX);
            let b: u32 = rng.random_range(0..=u32::MAX);
            test_adder(a, b);
        }
    }

    #[test]
    fn test_adder_edge_cases() {
        test_adder(u32::MAX, 0);
        test_adder(0, u32::MAX);
        test_adder(u32::MAX - 1, 1);
        test_adder(1, u32::MAX - 1);
        test_adder(u32::MAX - 2, 2);
        test_adder(2, u32::MAX - 2);
    }
}