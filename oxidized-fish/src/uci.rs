use crate::board::Board;
use crate::search::Search;
use crate::types::Move;
use std::io::{self, BufRead};
use std::time::Duration;

pub fn start_uci() {
    let stdin = io::stdin();
    let mut searcher = Search::new();
    let mut board = Board::new();

    println!("id name Oxidized Fish 0.4-Improved");
    println!("id author Gemini");
    println!("option name Hash type spin default 32 min 1 max 1024");
    println!("uciok");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let args: Vec<&str> = line.split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        match args[0] {
            "uci" => {
                println!("id name Oxidized Fish 0.4-Improved");
                println!("id author Gemini");
                println!("option name Hash type spin default 32 min 1 max 1024");
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "ucinewgame" => {
                board = Board::new();
                searcher.clear_tt();
            }
            "setoption" => {
                if args.len() >= 5 && args[2] == "Hash" {
                    if let Ok(size) = args[4].parse::<usize>() {
                        searcher.resize_tt(size);
                    }
                }
            }
            "position" => {
                let mut moves_start = 0;
                if args.len() > 1 && args[1] == "startpos" {
                    board = Board::new();
                    moves_start = 2;
                } else if args.len() > 1 && args[1] == "fen" {
                    let mut fen_parts = Vec::new();
                    let mut i = 2;
                    while i < args.len() && args[i] != "moves" {
                        fen_parts.push(args[i]);
                        i += 1;
                    }
                    board = Board::from_fen(&fen_parts.join(" "));
                    moves_start = i;
                }

                if moves_start < args.len() && args[moves_start] == "moves" {
                    for m_str in &args[moves_start + 1..] {
                        if let Some(mv) = parse_move(&board, m_str) {
                            board = board.make_move(mv);
                        }
                    }
                }
            }
            "go" => {
                let mut depth = 64;
                let mut wtime: Option<u64> = None;
                let mut btime: Option<u64> = None;
                let mut winc: u64 = 0;
                let mut binc: u64 = 0;
                let mut movetime: Option<u64> = None;

                let mut i = 1;
                while i < args.len() {
                    match args[i] {
                        "depth" => {
                            if i + 1 < args.len() {
                                depth = args[i + 1].parse().unwrap_or(64);
                                i += 2;
                                continue;
                            }
                        }
                        "wtime" => {
                            if i + 1 < args.len() {
                                wtime = args[i + 1].parse().ok();
                                i += 2;
                                continue;
                            }
                        }
                        "btime" => {
                            if i + 1 < args.len() {
                                btime = args[i + 1].parse().ok();
                                i += 2;
                                continue;
                            }
                        }
                        "winc" => {
                            if i + 1 < args.len() {
                                winc = args[i + 1].parse().unwrap_or(0);
                                i += 2;
                                continue;
                            }
                        }
                        "binc" => {
                            if i + 1 < args.len() {
                                binc = args[i + 1].parse().unwrap_or(0);
                                i += 2;
                                continue;
                            }
                        }
                        "movetime" => {
                            if i + 1 < args.len() {
                                movetime = args[i + 1].parse().ok();
                                i += 2;
                                continue;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }

                let time_limit =
                    calculate_time_limit(board.side_to_move, wtime, btime, winc, binc, movetime);

                if let Some(m) = searcher.go(&board, depth, time_limit) {
                    println!("bestmove {}", m.to_string());
                } else {
                    println!("bestmove 0000");
                }
            }
            "quit" => break,
            _ => {}
        }
    }
}

fn calculate_time_limit(
    side_to_move: crate::types::Color,
    wtime: Option<u64>,
    btime: Option<u64>,
    winc: u64,
    binc: u64,
    movetime: Option<u64>,
) -> Option<Duration> {
    if let Some(mt) = movetime {
        return Some(Duration::from_millis(mt));
    }

    let time = match side_to_move {
        crate::types::Color::White => wtime,
        crate::types::Color::Black => btime,
    };

    let inc = match side_to_move {
        crate::types::Color::White => winc,
        crate::types::Color::Black => binc,
    };

    if let Some(t) = time {
        let time_ms = t as i64;
        let inc_ms = inc as i64;

        let target = if time_ms < 2000 {
            (time_ms / 40).max(50)
        } else if time_ms < 10000 {
            (time_ms / 30 + inc_ms / 2).max(100)
        } else {
            (time_ms / 25 + inc_ms * 3 / 4).max(200)
        };

        let max_time = (time_ms / 2).max(100);
        let allocated = target.min(max_time) as u64;

        return Some(Duration::from_millis(allocated));
    }

    None
}

fn parse_move(board: &Board, m_str: &str) -> Option<Move> {
    let legal_moves = crate::movegen::MoveGen::generate(board);
    for m in legal_moves {
        if m.to_string() == m_str {
            if board.is_legal(m) {
                return Some(m);
            }
        }
    }
    None
}
