use crate::board::Board;
use crate::movegen::MoveGen;
use crate::types::{Move, PieceType, INFINITY, MATE_VALUE};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const MAX_PLY: usize = 128;
const DEFAULT_TT_SIZE: usize = 4 * 1024 * 1024; // ~128MB

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
}

pub struct Search {
    pub nodes: u64,
    pub killers: [[Option<Move>; 2]; MAX_PLY],
    pub history: [[i32; 64]; 64],
    pub countermoves: [[Option<Move>; 64]; 64],
    pub tt: Arc<Mutex<Vec<Option<TTEntry>>>>,
    pub start_time: Option<Instant>,
    pub time_limit: Option<Duration>,
    pub stop_search: bool,
    pub tt_size: usize,
    pub stop_flag: Arc<AtomicBool>,
    pub game_history: [u64; 1024],
    pub game_history_count: usize,
}

impl Search {
    pub fn new() -> Self {
        Search {
            nodes: 0,
            killers: [[None; 2]; MAX_PLY],
            history: [[0; 64]; 64],
            countermoves: [[None; 64]; 64],
            tt: Arc::new(Mutex::new(vec![None; DEFAULT_TT_SIZE])),
            start_time: None,
            time_limit: None,
            stop_search: false,
            tt_size: DEFAULT_TT_SIZE,
            stop_flag: Arc::new(AtomicBool::new(false)),
            game_history: [0; 1024],
            game_history_count: 0,
        }
    }

    pub fn resize_tt(&mut self, mb_size: usize) {
        let entry_size = std::mem::size_of::<Option<TTEntry>>();
        let new_size = (mb_size * 1024 * 1024) / entry_size;
        self.tt_size = new_size;
        let mut tt = self.tt.lock().unwrap();
        *tt = vec![None; new_size];
    }

    pub fn clear_tt(&mut self) {
        {
            let mut tt = self.tt.lock().unwrap();
            for entry in tt.iter_mut() {
                *entry = None;
            }
        }
        self.clear_history();
        self.clear_killers();
        self.countermoves = [[None; 64]; 64];
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
        if self.stop_search || self.stop_flag.load(Ordering::Relaxed) {
            return true;
        }
        if self.nodes % 2048 == 0 {
            if let (Some(start), Some(limit)) = (self.start_time, self.time_limit) {
                if start.elapsed() >= limit {
                    self.stop_flag.store(true, Ordering::Relaxed);
                    return true;
                }
            }
        }
        false
    }

    fn is_repetition(&self, hash: u64) -> bool {
        for i in 0..self.game_history_count.saturating_sub(1) {
            if self.game_history[i] == hash {
                return true;
            }
        }
        false
    }

    fn adjust_mate_score_to_tt(&self, score: i32, ply: usize) -> i32 {
        if score >= MATE_VALUE - MAX_PLY as i32 {
            score + ply as i32
        } else if score <= -MATE_VALUE + MAX_PLY as i32 {
            score - ply as i32
        } else {
            score
        }
    }

    fn adjust_mate_score_from_tt(&self, score: i32, ply: usize) -> i32 {
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
        self.stop_flag.store(false, Ordering::Relaxed);
        self.start_time = Some(Instant::now());
        self.time_limit = time_limit;

        self.game_history[0] = board.hash;
        self.game_history_count = 1;

        let mut best_move = None;
        let mut last_best_move = None;
        let mut aspiration_window = 35;
        let mut last_score = 0;

        for d in 1..=max_depth {
            if self.should_stop() {
                break;
            }

            let mut alpha = -INFINITY;
            let mut beta = INFINITY;

            if d > 4 {
                alpha = last_score - aspiration_window;
                beta = last_score + aspiration_window;
            }

            loop {
                let (m_opt, score) = self.root_search(board, d, alpha, beta);
                if self.should_stop() && d > 1 {
                    break;
                }

                if let Some(m) = m_opt {
                    if score <= alpha {
                        alpha = (alpha - aspiration_window).max(-INFINITY);
                        aspiration_window = (aspiration_window as f64 * 1.6) as i32;
                    } else if score >= beta {
                        beta = (beta + aspiration_window).min(INFINITY);
                        aspiration_window = (aspiration_window as f64 * 1.6) as i32;
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
                        aspiration_window = (aspiration_window as f64 * 0.8).max(35.0) as i32;
                        break;
                    }
                } else {
                    break;
                }
                if aspiration_window > 2500 {
                    alpha = -INFINITY;
                    beta = INFINITY;
                }
            }
        }
        last_best_move.or(best_move)
    }

    fn root_search(
        &mut self,
        board: &Board,
        depth: u8,
        mut alpha: i32,
        beta: i32,
    ) -> (Option<Move>, i32) {
        let mut best_move = None;
        let mut best_score = -INFINITY;

        let mut moves = MoveGen::generate(board);
        let tt_move = self.probe_tt(board.hash).and_then(|e| e.m);
        self.order_moves(board, &mut moves, 0, tt_move, None);

        for (i, m) in moves.iter().enumerate() {
            if !board.is_legal(*m) {
                continue;
            }
            let next_board = board.make_move(*m);

            let mut score;
            if i == 0 {
                self.game_history[self.game_history_count] = next_board.hash;
                self.game_history_count += 1;
                score = -self.alpha_beta(
                    &next_board,
                    depth - 1,
                    -beta,
                    -alpha,
                    1,
                    true,
                    true,
                    Some(*m),
                );
                self.game_history_count -= 1;
            } else {
                self.game_history[self.game_history_count] = next_board.hash;
                self.game_history_count += 1;
                score = -self.alpha_beta(
                    &next_board,
                    depth - 1,
                    -alpha - 1,
                    -alpha,
                    1,
                    true,
                    true,
                    Some(*m),
                );
                self.game_history_count -= 1;
                if score > alpha && score < beta {
                    self.game_history[self.game_history_count] = next_board.hash;
                    self.game_history_count += 1;
                    score = -self.alpha_beta(
                        &next_board,
                        depth - 1,
                        -beta,
                        -alpha,
                        1,
                        true,
                        true,
                        Some(*m),
                    );
                    self.game_history_count -= 1;
                }
            }

            if self.should_stop() {
                break;
            }

            if score > best_score {
                best_score = score;
                best_move = Some(*m);
            }
            if score > alpha {
                alpha = score;
            }
        }

        if !self.stop_flag.load(Ordering::Relaxed) && best_move.is_some() {
            self.store_tt(board.hash, best_move, best_score, depth, TTFlag::Exact);
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
        allow_singular: bool,
        last_move: Option<Move>,
    ) -> i32 {
        if self.should_stop() {
            return 0;
        }
        self.nodes += 1;

        if ply >= MAX_PLY {
            return crate::eval::evaluate(board);
        }
        if self.is_repetition(board.hash) || board.halfmove_clock >= 100 {
            return 0;
        }

        let tt_entry = self.probe_tt(board.hash);
        let mut tt_move = None;
        if let Some(e) = tt_entry {
            tt_move = e.m;
            if e.depth >= depth {
                let adjusted_score = self.adjust_mate_score_from_tt(e.score, ply);
                match e.flag {
                    TTFlag::Exact => return adjusted_score,
                    TTFlag::LowerBound if adjusted_score >= beta => return adjusted_score,
                    TTFlag::UpperBound if adjusted_score <= alpha => return adjusted_score,
                    _ => {}
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

        // RFP (Static Null Move Pruning)
        if !in_check && depth <= 3 && ply > 0 && eval - (120 * depth as i32) >= beta {
            return eval - (120 * depth as i32);
        }

        // Null Move Pruning
        if allow_null
            && !in_check
            && depth >= 3
            && eval >= beta
            && board.occupied().count_ones() > 4
        {
            let mut null_board = *board;
            null_board.side_to_move = board.side_to_move.opponent();
            null_board.hash ^= crate::zobrist::ZOBRIST.side;
            if let Some(sq) = null_board.ep_square {
                null_board.hash ^= crate::zobrist::ZOBRIST.ep[sq as usize];
            }
            null_board.ep_square = None;

            let r = 3 + depth / 4;
            let score = -self.alpha_beta(
                &null_board,
                depth.saturating_sub(1 + r),
                -beta,
                -beta + 1,
                ply + 1,
                false,
                false,
                None,
            );
            if score >= beta {
                return beta;
            }
        }

        // Singular Extension
        let mut extension = 0;

        // IID (Internal Iterative Deepening)
        if depth >= 6 && tt_move.is_none() {
            let d = if depth > 8 { depth - 4 } else { depth - 2 };
            self.alpha_beta(board, d, alpha, beta, ply, false, false, last_move);
            if let Some(e) = self.probe_tt(board.hash) {
                tt_move = e.m;
            }
        }

        if allow_singular && depth >= 6 && tt_move.is_some() {
            if let Some(e) = tt_entry {
                if e.depth >= depth - 3 && e.flag != TTFlag::UpperBound {
                    let tt_score = self.adjust_mate_score_from_tt(e.score, ply);
                    let margin = 2 * depth as i32;
                    let singular_beta = tt_score - margin;
                    let score = self.alpha_beta(
                        board,
                        (depth - 1) / 2,
                        singular_beta - 1,
                        singular_beta,
                        ply,
                        false,
                        false,
                        last_move,
                    );
                    if score < singular_beta {
                        extension = 1;
                    }
                }
            }
        }

        let mut moves = MoveGen::generate(board);
        self.order_moves(board, &mut moves, ply, tt_move, last_move);

        let mut best_score = -INFINITY;
        let mut best_move = None;
        let mut moves_searched = 0;
        let mut legal_moves_found = 0;

        for m in moves {
            let next_board = board.make_move(m);

            // Check legality: did we leave our king in check?
            let us = board.side_to_move;
            let king_bit =
                next_board.by_type[PieceType::King as usize] & next_board.by_color[us as usize];
            if king_bit == 0 {
                continue;
            }
            let king_sq = king_bit.trailing_zeros() as u8;
            if next_board.is_square_attacked(king_sq, us.opponent()) {
                continue;
            }

            legal_moves_found += 1;
            let gives_check = next_board.is_in_check();

            // Late Move Pruning
            if !in_check
                && depth <= 3
                && !m.is_capture()
                && !m.is_promotion()
                && moves_searched >= (8 + depth * depth) as usize
                && !gives_check
            {
                continue;
            }

            // Futility Pruning
            if !in_check
                && depth <= 2
                && !m.is_capture()
                && !m.is_promotion()
                && !gives_check
                && eval + 250 * depth as i32 <= alpha
            {
                continue;
            }

            let mut score;
            let d = depth + extension;

            // Check for passed pawn push
            let is_pawn = (board.by_type[PieceType::Pawn as usize] & (1u64 << m.from())) != 0;
            let is_passed_pawn_push = if is_pawn {
                let us = board.side_to_move;
                let enemy_pawns = board.by_type[PieceType::Pawn as usize]
                    & board.by_color[us.opponent() as usize];
                crate::eval::is_passed_pawn(m.to(), us, enemy_pawns)
            } else {
                false
            };

            if moves_searched == 0 {
                self.game_history[self.game_history_count] = next_board.hash;
                self.game_history_count += 1;
                score = -self.alpha_beta(
                    &next_board,
                    d - 1,
                    -beta,
                    -alpha,
                    ply + 1,
                    true,
                    true,
                    Some(m),
                );
                self.game_history_count -= 1;
            } else {
                let reduction = if d >= 3
                    && moves_searched >= 3
                    && !m.is_capture()
                    && !m.is_promotion()
                    && !in_check
                    && !gives_check
                    && !is_passed_pawn_push
                {
                    let mut r = 1;
                    if moves_searched >= 8 {
                        r += 1;
                    }
                    if d >= 6 {
                        r += 1;
                    }
                    r
                } else {
                    0
                };

                self.game_history[self.game_history_count] = next_board.hash;
                self.game_history_count += 1;
                score = -self.alpha_beta(
                    &next_board,
                    d.saturating_sub(1 + reduction),
                    -alpha - 1,
                    -alpha,
                    ply + 1,
                    true,
                    true,
                    Some(m),
                );
                self.game_history_count -= 1;

                if score > alpha && reduction > 0 {
                    self.game_history[self.game_history_count] = next_board.hash;
                    self.game_history_count += 1;
                    score = -self.alpha_beta(
                        &next_board,
                        d - 1,
                        -alpha - 1,
                        -alpha,
                        ply + 1,
                        true,
                        true,
                        Some(m),
                    );
                    self.game_history_count -= 1;
                }
                if score > alpha && score < beta {
                    self.game_history[self.game_history_count] = next_board.hash;
                    self.game_history_count += 1;
                    score = -self.alpha_beta(
                        &next_board,
                        d - 1,
                        -beta,
                        -alpha,
                        ply + 1,
                        true,
                        true,
                        Some(m),
                    );
                    self.game_history_count -= 1;
                }
            }

            moves_searched += 1;
            if self.should_stop() {
                return 0;
            }

            if score >= beta {
                if !m.is_capture() {
                    self.store_killer(m, ply);
                    self.update_history(m, depth);
                    if let Some(lm) = last_move {
                        self.countermoves[lm.from() as usize][lm.to() as usize] = Some(m);
                    }
                }
                self.store_tt(
                    board.hash,
                    Some(m),
                    self.adjust_mate_score_to_tt(score, ply),
                    depth,
                    TTFlag::LowerBound,
                );
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

        if legal_moves_found == 0 {
            return if in_check {
                -MATE_VALUE + ply as i32
            } else {
                0
            };
        }

        if best_score == -INFINITY {
            return alpha;
        }

        let flag = if best_score <= alpha {
            TTFlag::UpperBound
        } else {
            TTFlag::Exact
        };
        self.store_tt(
            board.hash,
            best_move,
            self.adjust_mate_score_to_tt(best_score, ply),
            depth,
            flag,
        );
        best_score
    }

    fn quiescence(&mut self, board: &Board, mut alpha: i32, beta: i32, ply: usize) -> i32 {
        if self.should_stop() {
            return 0;
        }
        self.nodes += 1;
        if ply >= MAX_PLY {
            return crate::eval::evaluate(board);
        }

        let stand_pat = crate::eval::evaluate(board);
        if stand_pat >= beta {
            return beta;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let in_check = board.is_in_check();
        let mut moves = MoveGen::generate(board);
        if !in_check {
            moves.retain(|m| m.is_capture() || m.is_promotion());
        }
        self.order_moves(board, &mut moves, ply, None, None);

        for m in moves {
            if !board.is_legal(m) {
                continue;
            }
            if !in_check && stand_pat < alpha - 900 && !m.is_promotion() {
                continue;
            }
            // Relaxed SEE pruning from 0 to -200 to catch tactical sacrifices
            if board.see_value(m) < -200 {
                continue;
            }

            let next_board = board.make_move(m);
            let score = -self.quiescence(&next_board, -beta, -alpha, ply + 1);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }
        alpha
    }

    fn order_moves(
        &self,
        board: &Board,
        moves: &mut Vec<Move>,
        ply: usize,
        tt_move: Option<Move>,
        last_move: Option<Move>,
    ) {
        let countermove =
            last_move.and_then(|lm| self.countermoves[lm.from() as usize][lm.to() as usize]);

        moves.sort_by_cached_key(|m| {
            if Some(*m) == tt_move {
                return std::cmp::Reverse(5000000);
            }
            let mut score = 0;
            if m.is_capture() {
                let victim = board
                    .get_piece_at(m.to())
                    .map(|(p, _)| p)
                    .unwrap_or(PieceType::Pawn);
                let attacker = board
                    .get_piece_at(m.from())
                    .map(|(p, _)| p)
                    .unwrap_or(PieceType::Pawn);
                score = 4000000 + (victim as i32 * 100) - attacker as i32;
                score += board.see_value(*m);
            } else if m.is_promotion() {
                score = 3500000 + m.promoted_piece().map(|p| p as i32).unwrap_or(0);
            } else {
                if ply < MAX_PLY {
                    if Some(*m) == self.killers[ply][0] {
                        score = 3000000;
                    } else if Some(*m) == self.killers[ply][1] {
                        score = 2900000;
                    }
                }
                if Some(*m) == countermove {
                    score = 2800000;
                }
                if score == 0 {
                    score = self.history[m.from() as usize][m.to() as usize];
                }
            }
            std::cmp::Reverse(score)
        });
    }

    fn store_killer(&mut self, m: Move, ply: usize) {
        if ply < MAX_PLY && self.killers[ply][0] != Some(m) {
            self.killers[ply][1] = self.killers[ply][0];
            self.killers[ply][0] = Some(m);
        }
    }

    fn update_history(&mut self, m: Move, depth: u8) {
        let bonus = (depth as i32) * (depth as i32);
        let current = self.history[m.from() as usize][m.to() as usize];
        self.history[m.from() as usize][m.to() as usize] = (current + bonus).min(2000000);
        if self.history[m.from() as usize][m.to() as usize] >= 2000000 {
            for i in 0..64 {
                for j in 0..64 {
                    self.history[i][j] /= 2;
                }
            }
        }
    }

    fn probe_tt(&self, hash: u64) -> Option<TTEntry> {
        let tt = self.tt.lock().unwrap();
        let entry = tt[hash as usize % self.tt_size];
        if let Some(e) = entry {
            if e.key == hash {
                return Some(e);
            }
        }
        None
    }

    fn store_tt(&mut self, hash: u64, m: Option<Move>, score: i32, depth: u8, flag: TTFlag) {
        let idx = hash as usize % self.tt_size;
        let mut tt = self.tt.lock().unwrap();
        if let Some(existing) = tt[idx] {
            if existing.key == hash && existing.depth > depth {
                return;
            }
        }
        tt[idx] = Some(TTEntry {
            key: hash,
            m,
            score,
            depth,
            flag,
        });
    }
}
