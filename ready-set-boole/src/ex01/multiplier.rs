/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   multiplier.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:02:07 by xmatute-          #+#    #+#             */
/*   Updated: 2025/06/05 12:46:58 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn adder(a: u32, b: u32) -> u32 {
    if b == 0 {a}
    else {adder(a ^ b, (a & b) << 1)}
}

fn subtractor(a: u32, b: u32) -> u32 {
    adder(a, adder(!b, 1))
}

// pub fn multiplier(a: u32, b: u32) -> u32 {
//     if b == 0 {0}
//     else if b == 1 {a}
//     else {multiplier(a, subtractor(b, 1)) + a}
// }

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut r = 0;
    for _i in 1..=b {
        r = adder(r, a);
    }
    r
}