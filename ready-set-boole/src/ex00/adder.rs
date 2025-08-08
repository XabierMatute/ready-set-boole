/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   adder.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:01:26 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/08 17:53:30 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


// Recursive version of the adder function
// pub fn adder(a: u32, b: u32) -> u32 {
//     if b == 0 {a}
//     else {adder(a ^ b, (a & b) << 1)}
// }

// Iterative version of the adder function
// pub fn adder(a: u32, b: u32) -> u32 {
//     let mut sum: u32 = a;
//     let mut to_add: u32 = b;

//     while to_add != 0 {
//         let carry: u32 = sum & to_add;
//         sum = sum ^ to_add;
//         to_add = carry << 1;
//     }

//     sum
// }

// Iterative version of the adder function with a fixed number of iterations (Undoubtly O(1) time and space complexity)
pub fn adder(a: u32, b: u32) -> u32 {
    let mut sum: u32 = a;
    let mut to_add: u32 = b;

    for _ in 0..32 {
        let carry: u32 = sum & to_add;
        sum = sum ^ to_add;
        to_add = carry << 1;
    }

    sum
}