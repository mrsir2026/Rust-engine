use crate::board::Board;
use crate::movegen::MoveGen;
use crate::types::{Move, PieceType, INFINITY, MATE_VALUE};
use std::time::{Duration, Instant};

const MAX_PLY: usize = 64;
const DEFAULT_TT_SIZE: usize = 1024 * 1024; // ~32MB with 16 bytes per entry

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub key: u64,
    pub m: Option<Move>,
    pub score: i32,
    pub depth: u8,
    pub flag: TTFlag,
    pub age: u8,
}

pub struct Search {
    pub nodes: u64,
    pub killers: [[Option<Move>; 2]; MAX_PLY],
    pub history: [[i32; 64]; 64],
    pub tt: Vec<Option<TTEntry>>,
    pub start_time: Option<Instant>,
    pub time_limit: Option<Duration>,
    pub stop_search: bool,
    pub tt_size: usize,
    pub game_ply: u8,
}

impl Search {
    pub fn new() -> Self {
        Search {
            nodes: 0,
            killers: [[None; 2]; MAX_PLY],
            history: [[0; 64]; 64],
            tt: vec![None; DEFAULT_TT_SIZE],
            start_time: None,
            time_limit: None,
            stop_search: false,
            tt_size: DEFAULT_TT_SIZE,
            game_ply: 0,
        }
    }

    pub fn resize_tt(&mut self, mb_size: usize) {
        let entry_size = std::mem::size_of::<Option<TTEntry>>();
        let new_size = (mb_size * 1024 * 1024) / entry_size;
        self.tt_size = new_size;
        self.tt = vec![None; new_size];
    }

    pub fn clear_tt(&mut self) {
        for entry in self.tt.iter_mut() {
            *entry = None;
        }
        self.game_ply = 0;
        self.clear_history();
        self.clear_killers();
    }

    pub fn clear_history(&mut self) {
        for i in 0..64 {
            for j in 0..64 {
                self.history[i][j] = 0;
            }
        }
    }

    pub fn clear_killers(&mut self) {
        for i in 0..MAX_PLY {
            self.killers[i] = [None; 2];
        }
    }

    fn should_stop(&self) -> bool {
        if self.stop_search {
            return true;
        }
        if let (Some(start), Some(limit)) = (self.start_time, self.time_limit) {
            if start.elapsed() >= limit {
                return true;
            }
        }
        false
    }

    fn adjust_mate_score(&self, score: i32, ply: usize) -> i32 {
        if score >= MATE_VALUE - MAX_PLY as i32 {
            score - ply as i32
        } else if score <= -MATE_VALUE + MAX_PLY as i32 {
            score + ply as i32
        } else {
            score
        }
    }

    pub fn go(
        &mut self,
        board: &Board,
        max_depth: u8,
        time_limit: Option<Duration>,
    ) -> Option<Move> {
        self.nodes = 0;
        self.stop_search = false;
        self.start_time = Some(Instant::now());
        self.time_limit = time_limit;
        self.killers = [[None; 2]; MAX_PLY];

        let mut best_move = None;
        let mut last_best_move = None;
        let mut aspiration_window = 50;
        let mut last_score = 0;

        for d in 1..=max_depth {
            if self.should_stop() {
                break;
            }

            let mut alpha = -INFINITY;
            let mut beta = INFINITY;

            if d > 5 {
                alpha = last_score - aspiration_window;
                beta = last_score + aspiration_window;
            }

            loop {
                let (m, score) = self.root_search(board, d, alpha, beta);

                if self.stop_search {
                    break;
                }

                if score <= alpha {
                    aspiration_window *= 2;
                    alpha = score - aspiration_window;
                    beta = score + aspiration_window;
                } else if score >= beta {
                    aspiration_window *= 2;
                    beta = score + aspiration_window;
                    alpha = score - aspiration_window;
                } else {
                    best_move = Some(m);
                    last_best_move = Some(m);
                    last_score = score;

                    let score_str = if score >= MATE_VALUE - MAX_PLY as i32 {
                        format!("mate {}", (MATE_VALUE - score + 1) / 2)
                    } else if score <= -MATE_VALUE + MAX_PLY as i32 {
                        format!("mate {}", -(-MATE_VALUE - score + 1) / 2)
                    } else {
                        format!("cp {}", score)
                    };

                    let elapsed = self
                        .start_time
                        .map(|s| s.elapsed().as_millis() as u64)
                        .unwrap_or(0);
                    let nps = if elapsed > 0 {
                        (self.nodes as u128 * 1000 / elapsed as u128) as u64
                    } else {
                        0
                    };

                    println!(
                        "info depth {} score {} nodes {} time {} nps {} pv {}",
                        d,
                        score_str,
                        self.nodes,
                        elapsed,
                        nps,
                        m.to_string()
                    );

                    aspiration_window = 50;
                    break;
                }

                if aspiration_window > 5000 {
                    alpha = -INFINITY;
                    beta = INFINITY;
                }
            }
        }

        last_best_move.or(best_move)
    }

    fn root_search(&mut self, board: &Board, depth: u8, mut alpha: i32, beta: i32) -> (Move, i32) {
        let mut best_move = Move::new(0, 0, Move::QUIET);
        let mut best_score = -INFINITY;

        if self.should_stop() {
            return (best_move, 0);
        }

        let mut moves = MoveGen::generate(board);
        let tt_move = self.probe_tt(board.hash).and_then(|e| e.m);
        self.order_moves(board, &mut moves, 0, tt_move);

        let mut legal_moves_found = 0;
        for (i, m) in moves.iter().enumerate() {
            if self.should_stop() {
                break;
            }

            if !board.is_legal(*m) {
                continue;
            }
            legal_moves_found += 1;

            let next_board = board.make_move(*m);
            let score = if i == 0 {
                -self.alpha_beta(&next_board, depth - 1, -beta, -alpha, 1, true)
            } else {
                let mut score =
                    -self.alpha_beta(&next_board, depth - 1, -alpha - 1, -alpha, 1, true);
                if score > alpha && score < beta {
                    score = -self.alpha_beta(&next_board, depth - 1, -beta, -alpha, 1, true);
                }
                score
            };

            if self.should_stop() {
                break;
            }

            if score > best_score {
                best_score = score;
                best_move = *m;
            }
            if score > alpha {
                alpha = score;
            }
        }

        if !self.should_stop() && legal_moves_found == 0 {
            if board.is_in_check() {
                return (best_move, -MATE_VALUE + 1);
            } else {
                return (best_move, 0);
            }
        }

        if !self.should_stop() && legal_moves_found > 0 {
            self.store_tt(
                board.hash,
                Some(best_move),
                best_score,
                depth,
                TTFlag::Exact,
            );
        }

        (best_move, best_score)
    }

    fn alpha_beta(
        &mut self,
        board: &Board,
        mut depth: u8,
        mut alpha: i32,
        beta: i32,
        ply: usize,
        allow_null: bool,
    ) -> i32 {
        self.nodes += 1;

        if self.should_stop() {
            return 0;
        }

        if ply >= MAX_PLY {
            return crate::eval::evaluate(board);
        }

        let tt_entry = self.probe_tt(board.hash);
        if let Some(e) = tt_entry {
            if e.depth >= depth {
                let adjusted_score = self.adjust_mate_score(e.score, ply);
                match e.flag {
                    TTFlag::Exact => return adjusted_score,
                    TTFlag::LowerBound => {
                        if adjusted_score >= beta {
                            return adjusted_score;
                        }
                    }
                    TTFlag::UpperBound => {
                        if adjusted_score <= alpha {
                            return adjusted_score;
                        }
                    }
                }
            }
        }

        let in_check = board.is_in_check();
        if in_check {
            depth += 1;
        }
        if depth == 0 {
            return self.quiescence(board, alpha, beta, ply);
        }

        let eval = crate::eval::evaluate(board);

        // Reverse Futility Pruning
        if !in_check && depth <= 3 && ply > 0 {
            let margin = 120 * depth as i32;
            if eval - margin >= beta {
                return eval - margin;
            }
        }

        // Futility Pruning
        let futility_margin = [0, 150, 300, 450];
        let can_futility = !in_check && depth <= 3 && eval + futility_margin[depth as usize] <= alpha;

        // Null Move Pruning
        if allow_null && !in_check && depth >= 3 && ply > 0 && eval >= beta {
            let mut null_board = *board;
            null_board.side_to_move = board.side_to_move.opponent();
            null_board.hash ^= crate::zobrist::ZOBRIST.side;
            if let Some(sq) = null_board.ep_square {
                null_board.hash ^= crate::zobrist::ZOBRIST.ep[sq as usize];
            }
            null_board.ep_square = None;

            let r = 2 + depth / 6;
            let score =
                -self.alpha_beta(&null_board, depth.saturating_sub(1 + r), -beta, -beta + 1, ply + 1, false);
            if score >= beta {
                return beta;
            }
        }

        // Internal Iterative Deepening (IID)
        let mut tt_move = tt_entry.and_then(|e| e.m);
        if depth >= 4 && tt_move.is_none() {
            let reduced_depth = depth - 2;
            self.alpha_beta(board, reduced_depth, alpha, beta, ply, false);
            if let Some(e) = self.probe_tt(board.hash) {
                tt_move = e.m;
            }
        }

        let mut moves = MoveGen::generate(board);
        self.order_moves(board, &mut moves, ply, tt_move);

        let mut best_score = -INFINITY;
        let mut best_move = None;
        let mut moves_searched = 0;
        let mut legal_moves_found = 0;

        for m in moves {
            if self.nodes % 2048 == 0 && self.should_stop() {
                break;
            }

            if !board.is_legal(m) {
                continue;
            }
            legal_moves_found += 1;

            // Futility Pruning
            if can_futility && moves_searched > 0 && !m.is_capture() && !m.is_promotion() {
                continue;
            }

            let next_board = board.make_move(m);
            // SEE Pruning
            if depth <= 4 && !in_check && moves_searched > 0 && m.is_capture() {
                if board.see_value(m) < 0 {
                    moves_searched += 1;
                    continue;
                }
            }

            let mut score;
            if moves_searched == 0 {
                score = -self.alpha_beta(&next_board, depth - 1, -beta, -alpha, ply + 1, true);
            } else {
                // LMR
                let reduction = if depth >= 3 && moves_searched >= 4 && !m.is_capture() && !m.is_promotion() && !in_check {
                    let mut r = (moves_searched as f64).ln() * (depth as f64).ln() / 2.0;
                    if r < 1.0 { r = 1.0; }
                    r as u8
                } else {
                    0
                };

                score = -self.alpha_beta(&next_board, depth.saturating_sub(1 + reduction), -alpha - 1, -alpha, ply + 1, true);
                
                if score > alpha && reduction > 0 {
                    score = -self.alpha_beta(&next_board, depth - 1, -alpha - 1, -alpha, ply + 1, true);
                }
                
                if score > alpha && score < beta {
                    score = -self.alpha_beta(&next_board, depth - 1, -beta, -alpha, ply + 1, true);
                }
            }

            moves_searched += 1;

            if self.should_stop() {
                break;
            }

            if score >= beta {
                if !m.is_capture() {
                    self.store_killer(m, ply);
                    self.update_history(m, depth);
                }
                self.store_tt(board.hash, Some(m), score, depth, TTFlag::LowerBound);
                return score;
            }
            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
            if score > alpha {
                alpha = score;
            }
        }

        if !self.should_stop() && legal_moves_found == 0 {
            return if in_check {
                -MATE_VALUE + ply as i32
            } else {
                0
            };
        }

        if !self.should_stop() {
            let flag = if best_score <= alpha {
                TTFlag::UpperBound
            } else {
                TTFlag::Exact
            };

            let score_to_store = if best_score >= MATE_VALUE - MAX_PLY as i32 {
                best_score + ply as i32
            } else if best_score <= -MATE_VALUE + MAX_PLY as i32 {
                best_score - ply as i32
            } else {
                best_score
            };

            self.store_tt(board.hash, best_move, score_to_store, depth, flag);
        }

        best_score
    }

    fn quiescence(&mut self, board: &Board, mut alpha: i32, beta: i32, ply: usize) -> i32 {
        self.nodes += 1;

        if self.should_stop() || ply >= MAX_PLY {
            return crate::eval::evaluate(board);
        }

        let stand_pat = crate::eval::evaluate(board);
        if stand_pat >= beta { return beta; }
        if stand_pat > alpha { alpha = stand_pat; }

        let in_check = board.is_in_check();
        let mut moves = MoveGen::generate(board);
        if !in_check {
            moves.retain(|m| m.is_capture() || m.is_promotion());
        }

        self.order_moves(board, &mut moves, ply, None);

        let mut legal_moves_found = 0;
        for m in moves {
            if !board.is_legal(m) {
                continue;
            }
            legal_moves_found += 1;

            if !in_check && board.see_value(m) < 0 { continue; }
            
            let next_board = board.make_move(m);
            let score = -self.quiescence(&next_board, -beta, -alpha, ply + 1);

            if score >= beta { return beta; }
            if score > alpha { alpha = score; }
        }

        if in_check && legal_moves_found == 0 {
            return -MATE_VALUE + ply as i32;
        }

        alpha
    }

    fn order_moves(&self, board: &Board, moves: &mut Vec<Move>, ply: usize, tt_move: Option<Move>) {
        moves.sort_by_cached_key(|m| {
            if Some(*m) == tt_move {
                return std::cmp::Reverse(2000000);
            }
            
            let mut score = 0;
            if m.is_capture() {
                let victim = board.get_piece_at(m.to()).map(|(p, _)| p).unwrap_or(PieceType::Pawn);
                let attacker = board.get_piece_at(m.from()).map(|(p, _)| p).unwrap_or(PieceType::Pawn);
                
                let see = board.see_value(*m);
                if see >= 0 {
                    score = 1000000 + (victim as i32 * 10) - attacker as i32 + see;
                } else {
                    score = 100000 + see;
                }
            } else if m.is_promotion() {
                score = 900000 + m.promoted_piece().map(|p| p as i32).unwrap_or(0);
            } else {
                if ply < MAX_PLY {
                    if Some(*m) == self.killers[ply][0] {
                        score = 800000;
                    } else if Some(*m) == self.killers[ply][1] {
                        score = 700000;
                    }
                }
                if score == 0 {
                    score = self.history[m.from() as usize][m.to() as usize];
                }
            }
            std::cmp::Reverse(score)
        });
    }

    fn store_killer(&mut self, m: Move, ply: usize) {
        if ply >= MAX_PLY {
            return;
        }
        if self.killers[ply][0] != Some(m) {
            self.killers[ply][1] = self.killers[ply][0];
            self.killers[ply][0] = Some(m);
        }
    }

    fn update_history(&mut self, m: Move, depth: u8) {
        let bonus = (depth as i32) * (depth as i32);
        self.history[m.from() as usize][m.to() as usize] += bonus;
        if self.history[m.from() as usize][m.to() as usize] > 1000000 {
            for i in 0..64 {
                for j in 0..64 {
                    self.history[i][j] /= 2;
                }
            }
        }
    }

    fn probe_tt(&self, hash: u64) -> Option<TTEntry> {
        let entry = self.tt[hash as usize % self.tt_size];
        if let Some(e) = entry {
            if e.key == hash {
                return Some(e);
            }
        }
        None
    }

    fn store_tt(&mut self, hash: u64, m: Option<Move>, score: i32, depth: u8, flag: TTFlag) {
        let idx = hash as usize % self.tt_size;

        let should_store = if let Some(existing) = self.tt[idx] {
            existing.key != hash || depth >= existing.depth || existing.age != self.game_ply % 64
        } else {
            true
        };

        if should_store {
            self.tt[idx] = Some(TTEntry {
                key: hash,
                m,
                score,
                depth,
                flag,
                age: self.game_ply % 64,
            });
        }
    }
}
