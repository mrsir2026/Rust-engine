mod bitboard;
mod board;
mod eval;
mod movegen;
mod search;
mod tables;
mod types;
mod uci;
mod zobrist;

fn main() {
    uci::start_uci();
}
