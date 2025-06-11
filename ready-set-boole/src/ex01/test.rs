/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/05 12:25:11 by xmatute-          #+#    #+#             */
/*   Updated: 2025/06/05 12:47:32 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::multiplier::multiplier;

fn test_multiplier(a: u32, b: u32) -> u32 {
    let optained_result = multiplier(a, b);
    let expected_result = a.wrapping_mul(b);
    assert_eq!(optained_result, expected_result, 
        "Expected {} * {} to equal {}, but got {}", a, b, expected_result, optained_result);
    optained_result
}

#[test]
fn test_multiplier_zero() {
    test_multiplier(0, 0);
    test_multiplier(0, 1);
    test_multiplier(1, 0);
    test_multiplier(0, 100);
    test_multiplier(100, 0);
    test_multiplier(0, 123456789);
    test_multiplier(123456789, 0);
    test_multiplier(0, u32::MAX);
    test_multiplier(u32::MAX, 0);
    test_multiplier(0, 1_000_000);
    test_multiplier(1_000_000, 0);
    test_multiplier(0, 42);
    test_multiplier(42, 0);
}

#[test]
fn test_multiplier_one() {
    test_multiplier(1, 1);
    test_multiplier(2, 1);
    test_multiplier(1, 2);
    test_multiplier(10, 1);
    test_multiplier(1, 10);
    test_multiplier(100, 1);
    test_multiplier(1, 100);
    test_multiplier(123456789, 1);
    test_multiplier(1, 123456789);
    test_multiplier(u32::MAX, 1);
    test_multiplier(1, u32::MAX);
}

#[test]
fn test_multiplier_basic() {
    test_multiplier(2, 3);
    test_multiplier(10, 20);
    test_multiplier(100, 200);
    test_multiplier(123456789, 987654321);
    test_multiplier(42, 42);
    test_multiplier(1_000_000, 1_000_000);
    test_multiplier(1_000_000, 42);
    test_multiplier(42, 1_000_000);
}

#[test]
fn test_multiplier_random_numbers() {
    use rand::Rng;
    let mut rng = rand::rng();
    for _ in 0..100 {
        let a: u32 = rng.random_range(0..=u32::MAX);
        let b: u32 = rng.random_range(0..=u32::MAX);
        test_multiplier(a, b);
    }
}

#[test]
fn test_multiplier_large_numbers() {
    test_multiplier(u32::MAX, u32::MAX);
    test_multiplier(u32::MAX, 1);
    test_multiplier(1, u32::MAX);
    test_multiplier(u32::MAX - 1, u32::MAX - 1);
    test_multiplier(u32::MAX - 1, 1);
    test_multiplier(1, u32::MAX - 1);
}

#[test]
fn test_multiplier_edge_cases() {
    test_multiplier(u32::MAX, 0);
    test_multiplier(0, u32::MAX);
    test_multiplier(u32::MAX - 1, 1);
    test_multiplier(1, u32::MAX - 1);
    test_multiplier(u32::MAX - 1, u32::MAX - 1);
    test_multiplier(u32::MAX, u32::MAX - 1);
    test_multiplier(u32::MAX - 1, u32::MAX);
}
