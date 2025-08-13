/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   inverse_function.rs                                :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:35:34 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 19:51:50 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// ...existing code...
pub fn reverse_map(n: f64) -> (u16, u16) {
    if n < 0.0 || n > 1.0 {
        panic!("Input must be in the range [0, 1]");
    }
    let morton_code = (n * u32::MAX as f64).round() as u32;

    let mut x: u16 = 0;
    let mut y: u16 = 0;

    for i in 0..16 {
        let bit_x = ((morton_code >> (2 * i)) & 1) as u16;
        let bit_y = ((morton_code >> (2 * i + 1)) & 1) as u16;

        x |= bit_x << i;
        y |= bit_y << i;      
    }

    (x, y)
}
