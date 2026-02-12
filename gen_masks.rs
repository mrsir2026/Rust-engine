
fn generate_slider_mask(sq: usize, is_rook: bool) -> u64 {
    let mut mask = 0u64;
    let r = (sq / 8) as i16;
    let f = (sq % 8) as i16;
    let directions: &[(i16, i16)] = if is_rook {
        &[(1, 0), (-1, 0), (0, 1), (0, -1)]
    } else {
        &[(1, 1), (1, -1), (-1, 1), (-1, -1)]
    };

    for &(dr, df) in directions {
        let mut nr = r + dr;
        let mut nf = f + df;
        while nr > 0 && nr < 7 && nf > 0 && nf < 7 {
            mask |= 1u64 << (nr * 8 + nf);
            nr += dr;
            nf += df;
        }
    }
    mask
}

fn main() {
    println!("Bishop Masks:");
    for sq in 0..64 {
        println!("sq {}: 0x{:016x}", sq, generate_slider_mask(sq, false));
    }
    println!("
Rook Masks:");
    for sq in 0..64 {
        println!("sq {}: 0x{:016x}", sq, generate_slider_mask(sq, true));
    }
}
