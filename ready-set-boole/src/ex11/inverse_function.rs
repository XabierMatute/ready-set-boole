/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   inverse_function.rs                                :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:35:34 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 18:25:26 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn reverse_map(n: f64) -> (u16, u16) {
    // Denormalize the input value back to a 32-bit Morton code
    let morton_code = (n * u32::MAX as f64).round() as u32;

    let mut x: u16 = 0;
    let mut y: u16 = 0;

    // Extract interleaved bits for x and y
    for i in 0..16 {
        x |= (((morton_code >> (2 * i)) & 1) << i) as u16;
        y |= (((morton_code >> (2 * i + 1)) & 1) << i) as u16;
    }

    (x, y)
}
