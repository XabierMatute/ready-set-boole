/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   curve.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:30:37 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 19:42:29 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn map(x: u16, y: u16) -> f64 {
    let mut morton_code: u32 = 0;

    for i in 0..16 {
        let bit_x = (x >> i) & 1;
        let bit_y = (y >> i) & 1;

        morton_code |= (bit_x as u32) << (2 * i);
        morton_code |= (bit_y as u32) << (2 * i + 1);
    }

    morton_code as f64 / u32::MAX as f64
}
