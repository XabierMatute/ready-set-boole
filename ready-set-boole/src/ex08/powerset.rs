/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   powerset.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:23:37 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 18:24:30 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let n = set.len();

    // Generate all subsets using bit manipulation
    for i in 0..(1 << n) {
        let mut subset = Vec::new();
        for j in 0..n {
            if i & (1 << j) != 0 {
                subset.push(set[j]);
            }
        }
        result.push(subset);
    }

    result
}