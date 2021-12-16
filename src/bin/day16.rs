use aoc2021::lines_as_vec;

use bit_vec::BitVec;
use log::trace;

fn main() {
    env_logger::init();

    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u64 {
    let mut input = decode_line(&lines_as_vec("input/day16.txt")[0]);
    let decoded = old_decode_packet(&mut input).unwrap();
    sum_pkt_versions(decoded)
}

fn part2() -> u64 {
    let mut input = decode_line(&lines_as_vec("input/day16.txt")[0]);
    let decoded = Packet::try_from(&mut input).unwrap();
    decoded.data.value()
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Sum(Vec<Packet>),     // 0
    Product(Vec<Packet>), // 1
    Min(Vec<Packet>),     // 2
    Max(Vec<Packet>),     // 3
    Value(u64),           // 4
    GT(Box<[Packet; 2]>), // 5
    LT(Box<[Packet; 2]>), // 6
    Eq(Box<[Packet; 2]>), // 7
}

impl PacketType {
    fn value(&self) -> u64 {
        match self {
            PacketType::Sum(v) => v.iter().fold(0, |acc, p| acc + p.data.value()),
            PacketType::Product(v) => v.iter().fold(1, |acc, p| acc * p.data.value()),
            PacketType::Min(v) => v.iter().fold(u64::MAX, |acc, p| acc.min(p.data.value())),
            PacketType::Max(v) => v.iter().fold(0, |acc, p| acc.max(p.data.value())),
            PacketType::Value(v) => *v,
            PacketType::GT(v) => {
                if v[0].data.value() > v[1].data.value() {
                    1
                } else {
                    0
                }
            }
            PacketType::LT(v) => {
                if v[0].data.value() < v[1].data.value() {
                    1
                } else {
                    0
                }
            }
            PacketType::Eq(v) => {
                if v[0].data.value() == v[1].data.value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,      // 3 bits
    data: PacketType, // 3 bits
}

fn decode_line(line: &str) -> BitVec {
    BitVec::from_bytes(
        line.chars()
            .map(|c| u8::try_from(c.to_digit(16).unwrap()).expect("too big"))
            .collect::<Vec<u8>>()
            .as_slice()
            .chunks_exact(2)
            .flat_map(<&[u8; 2]>::try_from)
            .map(|[h, l]| h << 4 | l)
            .collect::<Vec<u8>>()
            .as_slice(),
    )
}

#[derive(Clone, Copy)]
enum PktLen {
    Bits(usize),
    Count(usize),
}

trait DecodeEx {
    fn get_length(&mut self) -> Result<PktLen, ()>;
    fn get_subpackets(&mut self, pktlen: PktLen) -> Vec<Packet>;
    fn take_bits(&mut self, at: usize) -> Result<Box<Self>, ()>;
    fn take_bits_as_bytes(&mut self, at: usize) -> Result<Vec<u8>, ()>;
}

impl DecodeEx for BitVec {
    fn get_length(&mut self) -> Result<PktLen, ()> {
        trace!("getting len");
        let length_type_id = *self.take_bits_as_bytes(1)?.get(0).ok_or(())?;
        Ok(if length_type_id == 0 {
            let len_b = self.take_bits_as_bytes(15)?;
            let mut out = vec![0u8; 8 - len_b.len()];
            out.extend(len_b.into_iter());
            let bit_len = usize::from_be_bytes(<[u8; 8]>::try_from(out).unwrap());
            trace!("bit len is {}", bit_len);
            PktLen::Bits(bit_len)
        } else {
            let len_c = self.take_bits_as_bytes(11)?;
            let mut out = vec![0u8; 8 - len_c.len()];
            out.extend(len_c.into_iter());
            let packet_count = usize::from_be_bytes(<[u8; 8]>::try_from(out).unwrap());
            trace!("packet count is {}", packet_count);
            PktLen::Count(packet_count)
        })
    }

    fn get_subpackets(&mut self, pktlen: PktLen) -> Vec<Packet> {
        let mut subpackets = Vec::new();
        match pktlen {
            PktLen::Bits(nb) => {
                let mut subbits = self.take_bits(nb).unwrap();
                while let Ok(pkt) = Packet::try_from(&mut *subbits) {
                    subpackets.push(pkt);
                }
            }
            PktLen::Count(mut nc) => {
                while let Ok(pkt) = Packet::try_from(&mut *self) {
                    subpackets.push(pkt);
                    nc -= 1;
                    if nc == 0 {
                        break;
                    }
                }
            }
        }
        subpackets
    }

    fn take_bits(&mut self, at: usize) -> Result<Box<Self>, ()> {
        if at > self.len() {
            return Err(());
        }

        trace!("start {:?}", self);
        let tail = self.split_off(at);
        trace!("head {:?} tail {:?}", self, tail);
        let result = self.clone();
        *self = tail;
        Ok(Box::new(result))

    }

    fn take_bits_as_bytes(&mut self, at: usize) -> Result<Vec<u8>, ()> {
        if at > self.len() {
            return Err(());
        }

        let head = self.take_bits(at)?;
        let result = BitVec::from_elem(8 - (at % 8), false) // fill with leading bits to pad left
            .into_iter()
            .chain(head.into_iter())
            .collect::<BitVec>()
            .to_bytes();
        trace!("result {:?}", result);
        Ok(result)
    }
}

impl TryFrom<&mut BitVec> for Packet {
    type Error = ();

    fn try_from(raw: &mut BitVec) -> Result<Self, Self::Error> {
        trace!("\nGETTING PACKET");
        let version = *raw.take_bits_as_bytes(3)?.get(0).ok_or(())?;
        trace!("version {}", version);
        let type_flg = *raw.take_bits_as_bytes(3)?.get(0).ok_or(())?;
        trace!("type flag is {}", type_flg);

        let data = if type_flg == 0x04 {

                let mut literal: u64 = 0;
                loop {
                    let moar = *raw.take_bits_as_bytes(1)?.get(0).ok_or(())?;
                    let value = *raw.take_bits_as_bytes(4)?.get(0).ok_or(())?;
                    literal = (literal << 4) | (value as u64);
                    if moar == 0 {
                        break;
                    }
                }
                trace!("pkt LITERAL {}", literal);
                PacketType::Value(literal)

        } else {

            let pktlen = raw.get_length()?;
            let subp = match pktlen {
                PktLen::Bits(_nb) => {
                    raw.get_subpackets(pktlen)
                }
                PktLen::Count(cp) => {
                    let subp = raw.get_subpackets(pktlen);
                    if subp.len() != cp {
                        panic!("packet count mismatch {} != {}\n{:?}", subp.len(), cp, subp);
                    }
                    subp
                }
            };

            match type_flg {
                0x0 => {
                    trace!("pkt SUB {:?}", subp);
                    PacketType::Sum(subp)
                }
                0x1 => {
                    trace!("pkt MUL {:?}", subp);
                    PacketType::Product(subp)
                }
                0x2 => {
                    trace!("pkt MIN {:?}", subp);
                    PacketType::Min(subp)
                }
                0x3 => {
                    trace!("pkt MAX {:?}", subp);
                    PacketType::Max(subp)
                }
                0x4 => unreachable!(),
                0x5 => {
                    trace!("pkt GT {:?}", subp);
                    if subp.len() != 2 {
                        panic!("bad gt subp {:?}", subp);
                    }
                    PacketType::GT(Box::new(<[Packet; 2]>::try_from(subp).unwrap()))
                }
                0x6 => {
                    trace!("pkt LT {:?}", subp);
                    if subp.len() != 2 {
                        panic!("bad lt subp {:?}", subp);
                    }
                    PacketType::LT(Box::new(<[Packet; 2]>::try_from(subp).unwrap()))
                }
                0x7 => {
                    trace!("pkt EQ {:?}", subp);
                    if subp.len() != 2 {
                        panic!("bad eq subp {:?}", subp);
                    }
                    PacketType::Eq(Box::new(<[Packet; 2]>::try_from(subp).unwrap()))
                }
                _ => unreachable!(),
            }
        };
        let done = Packet { version, data };
        trace!("DONE {:?}\n", done);
        Ok(done)
    }
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = Packet::try_from(&mut decode_line("C200B40A82")).unwrap();
        assert_eq!(3, input.data.value());
    }

    #[test]
    fn test_case_2() {
        let input = Packet::try_from(&mut decode_line("04005AC33890")).unwrap();
        assert_eq!(54, input.data.value());
    }

    #[test]
    fn test_case_3() {
        let input = Packet::try_from(&mut decode_line("880086C3E88112")).unwrap();
        assert_eq!(7, input.data.value());
    }

    #[test]
    fn test_case_4() {
        let input = Packet::try_from(&mut decode_line("CE00C43D881120")).unwrap();
        assert_eq!(9, input.data.value());
    }

    #[test]
    fn test_case_5() {
        let input = Packet::try_from(&mut decode_line("D8005AC2A8F0")).unwrap();
        assert_eq!(1, input.data.value());
    }

    #[test]
    fn test_case_6() {
        let input = Packet::try_from(&mut decode_line("F600BC2D8F")).unwrap();
        assert_eq!(0, input.data.value());
    }

    #[test]
    fn test_case_7() {
        let input = Packet::try_from(&mut decode_line("9C005AC2F8F0")).unwrap();
        assert_eq!(0, input.data.value());
    }

    #[test]
    fn test_case_8() {
        let input = Packet::try_from(&mut decode_line("9C0141080250320F1802104A08")).unwrap();
        assert_eq!(1, input.data.value());
    }
}

/*****************
 * part 1 stuff
 *****************/

fn old_decode_packet(raw: &mut BitVec) -> Result<OldPacket, ()> {
    let version = *raw.take_bits_as_bytes(3)?.get(0).ok_or(())?;
    let data = match raw.take_bits_as_bytes(3)?.get(0).ok_or(())? {
        0x4 => {
            let mut literal: u64 = 0;
            loop {
                let moar = *raw.take_bits_as_bytes(1)?.get(0).ok_or(())?;
                let value = *raw.take_bits_as_bytes(4)?.get(0).ok_or(())?;
                literal = (literal << 4) | (value as u64);
                if moar == 0 {
                    break;
                }
            }
            OldPacketType::Literal(literal)
        }
        _ => {
            raw.get_length()?; // ignore
            let mut subpackets = Vec::new();
            while let Ok(pkt) = old_decode_packet(&mut *raw) {
                subpackets.push(pkt);
            }
            OldPacketType::Op(subpackets)
        }
    };
    Ok(OldPacket { version, data })
}

fn sum_pkt_versions(p: OldPacket) -> u64 {
    p.version as u64
        + match p.data {
            OldPacketType::Literal(_) => 0,
            OldPacketType::Op(v) => v.into_iter().fold(0, |acc, p| acc + sum_pkt_versions(p)),
        }
}

#[derive(Debug, PartialEq, Eq)]
enum OldPacketType {
    Literal(u64), // 4
    Op(Vec<OldPacket>),
}

#[derive(Debug, PartialEq, Eq)]
struct OldPacket {
    version: u8,         // 3 bits
    data: OldPacketType, // 3 bits
}

#[cfg(test)]
mod part1_day16_tests {

    use super::*;

    #[test]
    fn test_sum_version() {
        let mut input = decode_line("8A004A801A8002F478");
        let d = old_decode_packet(&mut input).unwrap();
        assert_eq!(16, sum_pkt_versions(d));

        let mut input = decode_line("620080001611562C8802118E34");
        let d = old_decode_packet(&mut input).unwrap();
        assert_eq!(12, sum_pkt_versions(d));

        let mut input = decode_line("C0015000016115A2E0802F182340");
        let d = old_decode_packet(&mut input).unwrap();
        assert_eq!(23, sum_pkt_versions(d));

        let mut input = decode_line("A0016C880162017C3686B18A3D4780");
        let d = old_decode_packet(&mut input).unwrap();
        assert_eq!(31, sum_pkt_versions(d));
    }

    #[test]
    fn test_decoder() {
        let mut input = decode_line("D2FE28");

        let d = old_decode_packet(&mut input);
        assert_eq!(
            Ok(OldPacket {
                version: 6,
                data: OldPacketType::Literal(2021),
            }),
            d
        );

        let mut input = decode_line("38006F45291200");

        let d = old_decode_packet(&mut input);
        assert_eq!(
            Ok(OldPacket {
                version: 1,
                data: OldPacketType::Op(vec![
                    OldPacket {
                        version: 6,
                        data: OldPacketType::Literal(10),
                    },
                    OldPacket {
                        version: 2,
                        data: OldPacketType::Literal(20),
                    }
                ])
            }),
            d
        );

        let mut input = decode_line("EE00D40C823060");

        let d = old_decode_packet(&mut input);

        assert_eq!(
            Ok(OldPacket {
                version: 7,
                data: OldPacketType::Op(vec![
                    OldPacket {
                        version: 2,
                        data: OldPacketType::Literal(1),
                    },
                    OldPacket {
                        version: 4,
                        data: OldPacketType::Literal(2),
                    },
                    OldPacket {
                        version: 1,
                        data: OldPacketType::Literal(3),
                    },
                ])
            }),
            d
        );

        let mut input = decode_line("8A004A801A8002F478");
        let d = old_decode_packet(&mut input);

        assert_eq!(
            Ok(OldPacket {
                version: 4,
                data: OldPacketType::Op(vec![OldPacket {
                    version: 1,
                    data: OldPacketType::Op(vec![OldPacket {
                        version: 5,
                        data: OldPacketType::Op(vec![OldPacket {
                            version: 6,
                            data: OldPacketType::Literal(15)
                        }])
                    }])
                }])
            }),
            d
        );
    }

    #[test]
    fn test_take_bits_as_bytes() {
        let mut i = BitVec::from_bytes(&[0b11110000]);
        let j = i.take_bits_as_bytes(3);
        assert_eq!(Ok(vec![0b111]), j);
        let mut r = BitVec::from_elem(5, false);
        r.set(0, true);
        assert_eq!(r, i);

        let mut i = BitVec::from_elem(3, false);
        assert_eq!(Err(()), i.take_bits_as_bytes(5));
    }

    #[test]
    fn test_decode_line() {
        let input = decode_line("D2FE28");

        assert_eq!(BitVec::from_bytes(&[0xD2, 0xFE, 0x28]), input);
    }

    #[test]
    fn test_case() {}
}
