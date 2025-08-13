/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:19:22 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 19:36:15 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![cfg(test)]

mod tests {
    use crate::ex07::sat::sat;

    #[test]
    fn test_sat_basic_true() {
        assert!(sat("A"));
        assert!(sat("A!"));
        assert!(sat("AB&"));
        assert!(sat("AB|"));
        assert!(sat("AB^"));
        assert!(sat("AB>"));
        assert!(sat("AB="));
    }

    #[test]
    fn test_sat_basic_false() {
        assert!(!sat("0"));
        assert!(!sat("1!"));
        assert!(!sat("00&"));
        assert!(!sat("10&"));
        assert!(!sat("A!A&"));        
        assert!(!sat("01&"));
        assert!(!sat("AA!&"));        
        assert!(!sat("00|"));
        assert!(!sat("00^"));
        assert!(!sat("11^"));
        assert!(!sat("AA^"));
        assert!(!sat("A!A!^"));
        assert!(!sat("10>"));
        assert!(!sat("10="));
        assert!(!sat("AA!="));
        assert!(!sat("01="));
        assert!(!sat("A!A="));
    }

    #[test]
    fn test_sat_subject() {
        assert!(sat("AB|"));
        assert!(sat("AB&"));
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
    }

    #[test]
    fn test_sat_extra_trivial() {
        assert!(sat("A!B&"));
        assert!(sat("A!B|"));
        assert!(sat("A!B^"));
        assert!(sat("A!B>"));
        assert!(sat("A!B="));
        assert!(sat("A!B!&"));
        assert!(sat("A!B!|"));
        assert!(sat("A!B!^"));
        assert!(sat("A!B!>"));
        assert!(sat("A!B!="));
        assert!(sat("A!B!|C&"));
        assert!(sat("A!B!|C|D&"));
        assert!(sat("A!B!|C|D|E&"));
        
    }

    //TODO: añade tests 

}