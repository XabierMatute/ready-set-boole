/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   eval_formula.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:04:10 by xmatute-          #+#    #+#             */
/*   Updated: 2025/06/04 14:04:52 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn eval_formula(formula: &str) -> bool {
    let mut stack : Vec<bool> = Vec::new();
    for c in formula.chars() {
        match c {
            '0' => stack.push(false),
            '1' => stack.push(true),
            '!' => {
                let a = stack.pop().unwrap();
                stack.push(!a);
            }
            '&' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a && b);
            }
            '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a || b);
            }
            '^' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a != b);
            }
            '>' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(!a || b);
            }
            '=' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a == b);
            }
            c if c.is_whitespace() => continue,
            _ => panic!("Invalid character in formula: {}", c),
        }
    }
    if stack.len() != 1 {
        panic!("Invalid formula: {}", formula);
    }
    stack.pop().unwrap()
}