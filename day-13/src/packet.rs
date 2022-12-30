use std::cmp::Ordering;

#[derive(Debug, Clone, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl Packet {
    pub fn decode_packet(n: u32) -> Packet {
        Packet::List(vec![Packet::List(vec![Packet::Int(n)])])
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(lhs), Self::List(rhs)) => lhs == rhs,
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::Int(lhs), Self::List(rhs)) => &vec![Packet::Int(*lhs)] == rhs,
            (Self::List(lhs), Self::Int(rhs)) => lhs == &vec![Packet::Int(*rhs)],
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(lhs), Self::List(rhs)) => lhs.cmp(rhs),
            (Self::Int(lhs), Self::Int(rhs)) => lhs.cmp(rhs),
            (Self::Int(lhs), Self::List(rhs)) => vec![Packet::Int(*lhs)].cmp(rhs),
            (Self::List(lhs), Self::Int(rhs)) => lhs.cmp(&vec![Packet::Int(*rhs)]),
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
        let packets = &packets[0];

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
        let packets = &packets[0];

        assert_eq!(packets.0.partial_cmp(&packets.1), Some(Ordering::Less));
    }

    #[test]
    fn test_empty_vs_full() {
        let (_, packets) =
            ppp("[[],[]]\n[[1,1,1,[[8,3],6,[5,0],[0,5,7],[3,8]]],[[[7,7,7],[7,3],4,3]],[10,5]]")
                .unwrap();

        let packets = &packets[0];

        assert_eq!(packets.0.partial_cmp(&packets.1), Some(Ordering::Less));
    }
}
