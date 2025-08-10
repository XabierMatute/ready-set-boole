/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/10 16:58:42 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 18:21:48 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod ex00 {
    pub mod adder;
    #[cfg(test)]
    mod test;
}

pub mod ex01 {
    pub mod multiplier;
    #[cfg(test)]
    mod test;
}

pub mod ex02 {
    pub mod gray_code;
    #[cfg(test)]
    mod test;
}

pub mod ex03 {
    pub mod eval_formula;
    #[cfg(test)]
    mod test;
}

pub mod ex04 {
    pub mod truth_table;
    #[cfg(test)]
    mod test;
}


pub mod ex05 {
    pub mod nnf;
    #[cfg(test)]
    mod test;
}

pub mod ex06 {
    pub mod cnf;
    #[cfg(test)]
    mod test;
}

pub mod ex07 {
    pub mod sat;
    #[cfg(test)]
    mod test;
}

pub mod ex08 {
    pub mod powerset;
    #[cfg(test)]
    mod test;
}

pub mod ex09 {
    pub mod set_evaluation;
    #[cfg(test)]
    mod test;
}

pub mod ex10 {
    pub mod curve;
    #[cfg(test)]
    mod test;
}

pub mod ex11 {
    pub mod inverse_function;
    #[cfg(test)]
    mod test;
}

pub mod extra {
    pub mod formula;
}

