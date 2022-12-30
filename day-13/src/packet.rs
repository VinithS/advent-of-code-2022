use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Int(l0), Self::List(_)) => Packet::List(vec![Packet::Int(*l0)]).eq(other),
            (Self::List(_), Self::Int(r0)) => self.eq(&Packet::List(vec![Packet::Int(*r0)])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => {
                // println!("Comparing ints {} - {} => {:?}", lhs, rhs, lhs.cmp(rhs));
                lhs.partial_cmp(rhs)
            }
            (Self::List(lhs), Self::List(rhs)) => {
                let min_len = std::cmp::min(lhs.len(), rhs.len());

                for i in 0..min_len {
                    let cmp = lhs[i].partial_cmp(&rhs[i]);
                    if let Some(ord) = cmp {
                        return Some(ord);
                    }
                }
                lhs.len().partial_cmp(&rhs.len())

                // println!("Comparing lists {:?} vs. {:?}", lhs, rhs);
                // let mut prev: Ordering = Ordering::Greater;

                // for (idx, p_left) in lhs.iter().enumerate() {
                //     match rhs.get(idx) {
                //         Some(p_right) => {
                //             println!("Loop: [{}] - {:?} - {:?}", idx, p_left, p_right);
                //             let ord = p_left.partial_cmp(p_right).unwrap();

                //             match ord {
                //                 Ordering::Less | Ordering::Equal => {
                //                     println!("Less or Equal. Continue.");
                //                     prev = ord;
                //                     continue;
                //                 }
                //                 Ordering::Greater => {
                //                     println!("List greater due to int cmp.");
                //                     return Some(Ordering::Greater);
                //                 }
                //             }
                //         }
                //         None => {
                //             println!("No RHS left. Prev was {:?}", prev);
                //             match prev {
                //                 Ordering::Less | Ordering::Equal => return Some(Ordering::Less),
                //                 Ordering::Greater => return Some(prev),
                //             }
                //         }
                //     };
                // }

                // // check after we iterate through LHS
                // if rhs.len() < lhs.len() {
                //     return Some(Ordering::Greater);
                // } else {
                //     Some(Ordering::Less)
                // }
            }
            (Self::Int(lhs), Self::List(rhs)) => {
                println!("Comparing int/list {:?} - {:?}", lhs, rhs);

                Packet::List(vec![Packet::Int(*lhs)]).partial_cmp(&other)
            }
            (Self::List(lhs), Self::Int(rhs)) => {
                println!("Comparing list/int {:?} - {:?}", lhs, rhs);

                self.partial_cmp(&Packet::List(vec![Packet::Int(*rhs)]))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ppp;

    use super::*;

    #[test]
    fn test_basic_comparison() {
        let p1 = Packet::List(vec![
            Packet::Int(1),
            Packet::Int(1),
            Packet::Int(3),
            Packet::Int(1),
            Packet::Int(1),
        ]);

        let p2 = Packet::List(vec![
            Packet::Int(1),
            Packet::Int(1),
            Packet::Int(5),
            Packet::Int(1),
            Packet::Int(1),
        ]);

        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    }

    #[test]
    fn test_int_vs_list() {
        let p1 = Packet::List(vec![
            Packet::List(vec![Packet::Int(1)]),
            Packet::List(vec![Packet::Int(2), Packet::Int(3), Packet::Int(4)]),
        ]);

        let p2 = Packet::List(vec![Packet::List(vec![Packet::Int(1)]), Packet::Int(4)]);

        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    }

    #[test]
    fn test_list_vs_int() {
        let (_, packets) = ppp("[9]\n[[8,7,6]]").unwrap();

        // assert!(packets.0.lt(&packets.1));
        assert_eq!(
            packets.0.partial_cmp(&packets.1).unwrap(),
            Ordering::Greater
        );
    }

    #[test]
    fn test_input_case() {
        let (_, packets) =
            ppp("[[2,[6,1,0]],[],[]]\n[[[9],[[1,4],[1,10,9,2,6],6,2],4,[[10,9,3,8],6]],[[10,[]],[[1,8],6,6,7,0],[[1,9],0,4,[3],[8,8,8,10]]],[],[8,6,3,[5]],[[10],[[6,4,2,0],6,1,8,0],0,[[9],[3,4,0,5],[10]]]]").unwrap();

        assert_eq!(packets.0.partial_cmp(&packets.1), Some(Ordering::Greater));
    }

    #[test]
    fn test_empty_vs_full() {
        let (_, packets) =
            ppp("[[],[]]\n[[1,1,1,[[8,3],6,[5,0],[0,5,7],[3,8]]],[[[7,7,7],[7,3],4,3]],[10,5]]")
                .unwrap();

        assert_eq!(packets.0.partial_cmp(&packets.1), Some(Ordering::Less));
    }
}
