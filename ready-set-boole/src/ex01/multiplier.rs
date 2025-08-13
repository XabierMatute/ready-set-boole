/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   multiplier.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:02:07 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 17:03:08 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::ex00::adder::adder;

// Recursive version of the multiplier function
// pub fn multiplier(a: u32, b: u32) -> u32 {
//     match b {
//         0 => 0,
//         1 => a,
//         _ if b & 1 == 1 => adder(a, multiplier(a << 1, b >> 1)),
//         _ => multiplier(a << 1, b >> 1),
//     }
// }

// Iterative version of the multiplier function
// pub fn multiplier(a: u32, b: u32) -> u32 {
//     let mut result: u32 = a;
//     let mut multiplier: u32 = b;
//     let mut carry: u32 = 0;

//     if b == 0 {
//         return 0;
//     }
//     while multiplier > 1 {
//         if multiplier & 1 == 1 {
//             carry = adder(result, carry);
//         }
//         result <<= 1;
//         multiplier >>= 1;
//     }
//     result = adder(result, carry);

//     result
// }

// Iterative version of the multiplier function with a fixed number of iterations (Undoubtly O(1) time and space complexity)
pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut result: u32 = a;
    let mut multiplier: u32 = b;
    let mut carry: u32 = 0;

    if b == 0 {
        return 0;
    }
    for _ in 0..32 {
        if multiplier & 1 == 1 {
            carry = adder(result, carry);
        }
        result <<= 1;
        multiplier >>= 1;
    }
    result = adder(result, carry);

    result
}