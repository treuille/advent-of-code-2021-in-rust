#![allow(clippy::needless_collect, clippy::manual_map)]

use std::iter;

const PUZZLE_INPUT: &str = "
220D62004EF14266BBC5AB7A824C9C1802B360760094CE7601339D8347E20020264D0804CA95C33E
006EA00085C678F31B80010B88319E1A1802D8010D4BC268927FF5EFE7B9C94D0C80281A00552549
A7F12239C0892A04C99E1803D280F3819284A801B4CCDDAE6754FC6A7D2F89538510265A3097BDF0
530057401394AEA2E33EC127EC3010060529A18B00467B7ABEE992B8DD2BA8D29253700627637679
9BCFBA4793CFF379D75CA1AA001B11DE6428402693BEBF3CC94A314A73B084A21739B98000010338
D0A004CF4DCA4DEC80488F004C0010A83D1D2278803D1722F45F94F9F98029371ED7CFDE0084953B
0AD7C633D2FF070C013B004663DA857C4523384F9F5F9495C280050B300660DC3B87040084C20883
11C8010C84F1621F080513AC910676A651664698DF62EA401934B0E6003E3396B5BBCCC9921C1803
4200FC608E9094401C8891A234080330EE31C643004380296998F2DECA6CCC796F65224B5EBBD000
3EF3D05A92CE6B1B2B18023E00BCABB4DA84BCC0480302D0056465612919584662F46F3004B40160
0042E1044D89C200CC4E8B916610B80252B6C2FCCE608860144E99CD244F3C44C983820040E59E65
4FA6A59A8498025234A471ED629B31D004A4792B54767EBDCD2272A014CC525D21835279FAD49934
EDD45802F294ECDAE4BB586207D2C510C8802AC958DA84B400804E314E31080352AA938F13F24E9A
8089804B24B53C872E0D24A92D7E0E2019C68061A901706A00720148C404CA08018A005180100039
9B00D02A004000A8C402482801E200530058AC010BA8018C00694D4FA2640243CEA7D80280008446
48D91A4001088950462BC2E600216607480522B00540010C84914E1E0002111F21143B9BFD6D9513
005A4F9FC60AB40109CBB34E5D89C02C82F34413D59EA57279A42958B51006A13E8F60094EF81E66
D0E737AE08
";

fn main() {
    let (version_sum, value) = parse_packet(&mut bit_iter(PUZZLE_INPUT));
    println!("16a: {version_sum} (843)");
    println!("16b: {value} (5390807940351)");
}

fn bit_iter(s: &str) -> impl Iterator<Item = u8> + '_ {
    let chars = s.trim().lines().flat_map(|line| line.chars());
    chars.flat_map(|c| {
        let x = u8::from_str_radix(c.to_string().as_ref(), 16).unwrap();
        (0..4).map(move |shift| (x >> shift) & 1).rev()
    })
}

fn parse_packet<BitIter>(bits: &mut BitIter) -> (u64, u64)
where
    BitIter: Iterator<Item = u8>,
{
    let version = parse_u64(bits, 3);
    let packet_type = parse_u64(bits, 3);
    if packet_type == 4 {
        (version, parse_literal(bits))
    } else {
        let versions_and_values: Box<dyn Iterator<Item = (u64, u64)>> = match bits.next().unwrap() {
            0 => {
                let n_bits = parse_u64(bits, 15);
                let bits: Vec<u8> = bits.take(n_bits as usize).collect();
                let mut bits = bits.into_iter().peekable();
                Box::new(iter::from_fn(move || match bits.peek() {
                    Some(_) => Some(parse_packet(&mut bits)),
                    None => None,
                }))
            }
            1 => {
                let n_packets = parse_u64(bits, 11) as usize;
                Box::new(iter::repeat_with(|| parse_packet(bits)).take(n_packets))
            }
            _ => unimplemented!("All bits are 0 or 1."),
        };
        let (versions, values): (Vec<u64>, Vec<u64>) = versions_and_values.unzip();
        let value: u64 = match packet_type {
            0 => values.iter().sum(),
            1 => values.iter().product(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => (values[0] > values[1]) as u64,
            6 => (values[0] < values[1]) as u64,
            7 => (values[0] == values[1]) as u64,
            _ => panic!("Unepected packet type: {packet_type}"),
        };
        (version + versions.iter().sum::<u64>(), value)
    }
}

fn parse_u64<BitIter>(bits: &mut BitIter, n_bits: usize) -> u64
where
    BitIter: Iterator<Item = u8>,
{
    let mut res = 0;
    for _ in 0..n_bits {
        res = res << 1 | (bits.next().unwrap() as u64);
    }
    res
}

fn parse_literal<BitIter>(bits: &mut BitIter) -> u64
where
    BitIter: Iterator<Item = u8>,
{
    let mut res = 0;
    while bits.next() == Some(1) {
        res = (res << 4) | parse_u64(bits, 4);
    }
    res = (res << 4) | parse_u64(bits, 4);
    res
}
