import { Chess } from 'https://cdn.jsdelivr.net/npm/chess.js@1.4.0/dist/esm/chess.js';

// ===== Chess Application State =====
const socket = io();

// Game State
let game = new Chess();
let selectedSquare = null;
let currentMoveIndex = -1;
let moveHistory = [];

// Settings
let boardTheme = 'classic';
let pieceSet = 'cburnett';
let playerColor = 'w';
let opponentType = 'engine';
let engineDepth = 4;
let isBoardFlipped = false;
let showHints = true;
let soundEnabled = true;

// Timers
let whiteTime = 0;
let blackTime = 0;
let timerInterval = null;
let isGameActive = false;

// Engine
let engineStatus = 'disconnected';
let engineEvaluation = 0;

// Edit Mode State
let isEditMode = false;
let palletDraggedPiece = null;

// ===== Piece Images =====
const pieceImages = {
    'w': { 'p': 'wP', 'n': 'wN', 'b': 'wB', 'r': 'wR', 'q': 'wQ', 'k': 'wK' },
    'b': { 'p': 'bP', 'n': 'bN', 'b': 'bB', 'r': 'bR', 'q': 'bQ', 'k': 'bK' }
};

const pieceSets = {
    'cburnett': 'https://chessboardjs.com/img/chesspieces/wikipedia/',
    'merida': 'https://raw.githubusercontent.com/oakmac/chessboardjs/master/website/img/chesspieces/wikipedia/',
    'alpha': 'https://raw.githubusercontent.com/oakmac/chessboardjs/master/website/img/chesspieces/wikipedia/',
    'chessnut': 'https://raw.githubusercontent.com/oakmac/chessboardjs/master/website/img/chesspieces/wikipedia/'
};

// ===== DOM Elements =====
const boardElement = document.getElementById('chess-board');
const statusElement = document.getElementById('status');
const moveListElement = document.getElementById('move-list');
const engineStatusElement = document.getElementById('engine-status');
const engineBadge = document.getElementById('engine-badge');
const evalScoreElement = document.getElementById('eval-score');
const evalFillElement = document.getElementById('eval-fill');
const engineLinesElement = document.getElementById('engine-lines');

// ===== Initialization =====
function init() {
    renderBoard();
    setupEventListeners();
    updateCapturedPieces();
    socket.emit('start-engine');
    loadSettings();
}

// ===== Board Rendering =====
function renderBoard() {
    boardElement.innerHTML = '';
    
    const rows = isBoardFlipped ? [0,1,2,3,4,5,6,7] : [7,6,5,4,3,2,1,0];
    const cols = isBoardFlipped ? [7,6,5,4,3,2,1,0] : [0,1,2,3,4,5,6,7];

    rows.forEach(r => {
        cols.forEach(c => {
            const squareName = String.fromCharCode(97 + c) + (r + 1);
            const square = document.createElement('div');
            square.classList.add('square');
            square.classList.add((r + c) % 2 === 0 ? 'dark' : 'light');
            square.dataset.square = squareName;
            
            const piece = game.get(squareName);
            if (piece) {
                square.classList.add('has-piece');
                const img = document.createElement('img');
                const setUrl = pieceSets[pieceSet] || pieceSets['cburnett'];
                img.src = `${setUrl}${pieceImages[piece.color][piece.type]}.png`;
                img.draggable = true;
                img.alt = `${piece.color}${piece.type}`;
                
                // Drag events
                img.addEventListener('dragstart', (e) => handleDragStart(e, squareName));
                img.addEventListener('dragend', handleDragEnd);
                
                square.appendChild(img);
            }
            
            // Drop events
            square.addEventListener('dragover', handleDragOver);
            square.addEventListener('drop', (e) => handleDrop(e, squareName));
            square.addEventListener('dragleave', handleDragLeave);
            
            // Right click to delete piece in edit mode
            square.addEventListener('contextmenu', (e) => {
                if (isEditMode) {
                    e.preventDefault();
                    game.remove(squareName);
                    renderBoard();
                }
            });
            
            square.addEventListener('click', () => handleSquareClick(squareName));
            boardElement.appendChild(square);
        });
    });

    highlightLastMove();
    updateStatus();
    updateActivePlayer();
}

// ===== Drag & Drop Handlers =====
let draggedSquare = null;

function handleDragStart(e, square) {
    const piece = game.get(square);
    if (!piece || piece.color !== game.turn()) {
        e.preventDefault();
        return;
    }
    
    draggedSquare = square;
    e.target.classList.add('dragging');
    e.dataTransfer.effectAllowed = 'move';
    
    if (showHints) {
        selectedSquare = square;
        highlightLegalMoves();
    }
}

function handleDragEnd(e) {
    e.target.classList.remove('dragging');
    draggedSquare = null;
    clearHighlights();
}

function handleDragOver(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
    e.currentTarget.classList.add('drag-over');
}

function handleDragLeave(e) {
    e.currentTarget.classList.remove('drag-over');
}

function handleDrop(e, square) {
    e.preventDefault();
    e.currentTarget.classList.remove('drag-over');
    
    // Handle pallet piece drop
    if (palletDraggedPiece) {
        const type = palletDraggedPiece[1];
        const color = palletDraggedPiece[0];
        game.put({ type, color }, square);
        renderBoard();
        palletDraggedPiece = null;
        return;
    }

    if (draggedSquare && draggedSquare !== square) {
        if (isEditMode || e.altKey) {
            // "Teleport" move (illegal)
            const piece = game.get(draggedSquare);
            game.remove(draggedSquare);
            game.put(piece, square);
            renderBoard();
        } else {
            attemptMove(draggedSquare, square);
        }
    }
}

// ===== Move Handling =====
function handleSquareClick(square) {
    if (game.isGameOver() || !isGameActive && moveHistory.length > 0) return;
    
    // If engine's turn and playing against engine, ignore
    if (opponentType === 'engine' && game.turn() !== playerColor[0]) return;

    if (selectedSquare === square) {
        selectedSquare = null;
        clearHighlights();
        return;
    }

    const move = attemptMove(selectedSquare, square);

    if (!move) {
        const piece = game.get(square);
        if (piece && piece.color === game.turn()) {
            selectedSquare = square;
            if (showHints) {
                highlightLegalMoves();
            }
        } else {
            selectedSquare = null;
            clearHighlights();
        }
    }
}

function attemptMove(from, to) {
    if (!from || !to) return null;
    
    const piece = game.get(from);
    if (!piece) return null;

    // Check for promotion
    const isPawn = piece.type === 'p';
    const isLastRank = (piece.color === 'w' && to[1] === '8') || (piece.color === 'b' && to[1] === '1');
    const promotion = (isPawn && isLastRank) ? 'q' : undefined;

    const move = game.move({
        from: from,
        to: to,
        promotion: promotion
    });

    if (move) {
        playSound('move');
        saveMove(move);
        selectedSquare = null;
        renderBoard();
        updateCapturedPieces();
        checkEngineMove();
        
        if (game.isGameOver()) {
            handleGameOver();
        }
    }
    
    return move;
}

function saveMove(move) {
    currentMoveIndex++;
    moveHistory = moveHistory.slice(0, currentMoveIndex);
    moveHistory.push({
        move: move,
        fen: game.fen()
    });
    updateMoveHistory();
}

// ===== Highlighting =====
function highlightLegalMoves() {
    clearHighlights();
    
    if (!selectedSquare) return;
    
    const squareEl = document.querySelector(`.square[data-square="${selectedSquare}"]`);
    if (squareEl) squareEl.classList.add('selected');
    
    const moves = game.moves({ square: selectedSquare, verbose: true });
    moves.forEach(move => {
        const sq = document.querySelector(`.square[data-square="${move.to}"]`);
        if (sq) {
            sq.classList.add('legal-move');
            if (sq.querySelector('img')) {
                sq.classList.add('has-piece');
            }
        }
    });
}

function highlightLastMove() {
    if (currentMoveIndex >= 0 && moveHistory[currentMoveIndex]) {
        const move = moveHistory[currentMoveIndex].move;
        const fromSquare = document.querySelector(`.square[data-square="${move.from}"]`);
        const toSquare = document.querySelector(`.square[data-square="${move.to}"]`);
        
        if (fromSquare) fromSquare.classList.add('last-move');
        if (toSquare) toSquare.classList.add('last-move');
    }
}

function clearHighlights() {
    document.querySelectorAll('.square').forEach(sq => {
        sq.classList.remove('selected', 'legal-move', 'has-piece', 'drag-over');
    });
}

// ===== UI Updates =====
function updateStatus() {
    let status = '';
    let iconClass = 'fa-circle';
    let iconColor = '';
    
    const turn = game.turn() === 'w' ? 'White' : 'Black';

    if (game.isCheckmate()) {
        status = `${turn} is in checkmate`;
        iconClass = 'fa-chess-king';
    } else if (game.isDraw()) {
        status = 'Game ended in a draw';
        iconClass = 'fa-handshake';
    } else if (game.isStalemate()) {
        status = 'Stalemate';
        iconClass = 'fa-hand-paper';
    } else if (game.isThreefoldRepetition()) {
        status = 'Draw by repetition';
        iconClass = 'fa-redo';
    } else if (game.isInsufficientMaterial()) {
        status = 'Draw by insufficient material';
        iconClass = 'fa-chess-board';
    } else {
        status = `${turn} to move`;
        if (game.isCheck()) {
            status += ' (Check!)';
            iconClass = 'fa-exclamation-circle';
            iconColor = 'check';
        }
    }

    statusElement.innerHTML = `
        <span class="status-icon ${iconColor}"><i class="fas ${iconClass}"></i></span>
        <span class="status-text">${status}</span>
    `;
}

function updateActivePlayer() {
    const whiteCard = document.querySelector('.player-card.opponent');
    const blackCard = document.querySelector('.player-card.player');
    
    // This is simplified - you'd need to properly track which card is which
    if (game.turn() === 'w') {
        // White's turn
    } else {
        // Black's turn
    }
}

function updateMoveHistory() {
    if (moveHistory.length === 0) {
        moveListElement.innerHTML = '<div class="move-placeholder">No moves yet. Start a game!</div>';
        return;
    }
    
    moveListElement.innerHTML = '';
    
    moveHistory.forEach((item, index) => {
        const move = item.move;
        const moveNumber = Math.floor(index / 2) + 1;
        
        // Create move number if it's white's move
        if (index % 2 === 0) {
            const numEl = document.createElement('div');
            numEl.className = 'move-number';
            numEl.textContent = `${moveNumber}.`;
            moveListElement.appendChild(numEl);
        }
        
        const moveEl = document.createElement('div');
        moveEl.className = 'move-item';
        if (index === currentMoveIndex) {
            moveEl.classList.add('current');
        }
        moveEl.textContent = move.san;
        moveEl.onclick = () => goToMove(index);
        moveListElement.appendChild(moveEl);
    });
    
    // Scroll to current move
    const currentEl = moveListElement.querySelector('.move-item.current');
    if (currentEl) {
        currentEl.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    }
}

function updateCapturedPieces() {
    // This would track captured pieces by comparing initial board state
    // Simplified implementation - you'd need to track captures during gameplay
    const capturedByWhite = document.getElementById('captured-by-white');
    const capturedByBlack = document.getElementById('captured-by-black');
    
    // Placeholder - implement actual capture tracking
    if (capturedByWhite) {
        capturedByWhite.innerHTML = '';
    }
}

function updateEngineStatus(status, text) {
    const badge = document.getElementById('engine-badge');
    const dot = badge.querySelector('.status-dot');
    const textEl = badge.querySelector('.status-text');
    
    dot.className = `status-dot ${status}`;
    textEl.textContent = text;
    
    engineStatus = status;
}

function updateEvaluation(score) {
    engineEvaluation = score;
    evalScoreElement.textContent = score > 0 ? `+${score}` : score;
    
    // Update eval bar
    const maxScore = 10;
    const normalizedScore = Math.max(-maxScore, Math.min(maxScore, score));
    const percentage = 50 + (normalizedScore / maxScore) * 50;
    evalFillElement.style.height = `${percentage}%`;
}

// ===== Game Control =====
function startNewGame() {
    game.reset();
    moveHistory = [];
    currentMoveIndex = -1;
    selectedSquare = null;
    
    const side = document.querySelector('input[name="side"]:checked').value;
    playerColor = side === 'white' ? 'w' : 'b';
    isBoardFlipped = side === 'black';
    opponentType = document.querySelector('input[name="opponent"]:checked').value;
    
    // Reset timers
    const timeControl = document.getElementById('time-control').value;
    if (timeControl !== 'unlimited') {
        const baseTime = parseInt(timeControl.split('+')[0]) * 60;
        whiteTime = baseTime;
        blackTime = baseTime;
    }
    
    isGameActive = true;
    renderBoard();
    updateMoveHistory();
    updateCapturedPieces();
    
    if (playerColor === 'b') {
        checkEngineMove();
    }
    
    // Close any modals
    document.getElementById('game-over-modal').classList.remove('active');
}

function handleGameOver() {
    isGameActive = false;
    clearInterval(timerInterval);
    
    const modal = document.getElementById('game-over-modal');
    const icon = document.getElementById('result-icon');
    const title = document.getElementById('result-title');
    const message = document.getElementById('result-message');
    
    let result = '';
    let resultMessage = '';
    
    if (game.isCheckmate()) {
        const winner = game.turn() === 'w' ? 'Black' : 'White';
        result = winner === (playerColor === 'w' ? 'White' : 'Black') ? 'win' : 'loss';
        resultMessage = 'by checkmate';
    } else if (game.isDraw()) {
        result = 'draw';
        resultMessage = 'Draw';
    } else if (game.isStalemate()) {
        result = 'draw';
        resultMessage = 'Stalemate';
    }

    
    icon.className = `game-result-icon ${result}`;
    icon.innerHTML = result === 'win' ? '<i class="fas fa-trophy"></i>' : 
                     result === 'loss' ? '<i class="fas fa-times"></i>' : 
                     '<i class="fas fa-equals"></i>';
    
    title.textContent = result === 'win' ? 'Victory!' : 
                        result === 'loss' ? 'Defeat' : 
                        'Draw';
    message.textContent = resultMessage;
    
    modal.classList.add('active');
    playSound('game-end');
}

function flipBoard() {
    isBoardFlipped = !isBoardFlipped;
    renderBoard();
}

function undoMove() {
    if (currentMoveIndex < 0) return;

    let movesToUndo = 1;
    // If playing engine and it's player's turn, take back engine move + player move
    if (opponentType === 'engine' && game.turn() === playerColor[0] && currentMoveIndex >= 1) {
        // Check if the move we are undoing is indeed an engine move
        if (moveHistory[currentMoveIndex].move.color !== playerColor[0]) {
            movesToUndo = 2;
        }
    }

    // Truncate history and update index
    const targetIndex = Math.max(-1, currentMoveIndex - movesToUndo);
    moveHistory = moveHistory.slice(0, targetIndex + 1);
    currentMoveIndex = targetIndex;

    // Reconstruct game state
    if (currentMoveIndex === -1) {
        game.reset();
    } else {
        game.load(moveHistory[currentMoveIndex].fen);
    }
    
    selectedSquare = null;
    clearHighlights();
    renderBoard();
    updateMoveHistory();
    updateCapturedPieces();
    updateEvaluation(0);
    
    // If we undo while engine is thinking, it might still return a move.
    // The check in engine-response will ignore it.
}

function goToMove(index) {
    if (index < -1 || index >= moveHistory.length) return;
    
    currentMoveIndex = index;
    if (index === -1) {
        game.reset();
    } else {
        game.load(moveHistory[index].fen);
    }
    
    renderBoard();
    updateMoveHistory();
}

// ===== Engine Communication =====
function checkEngineMove() {
    if (!game.isGameOver() && opponentType === 'engine' && game.turn() !== playerColor[0]) {
        updateEngineStatus('thinking', 'Engine thinking...');
        const fen = game.fen();
        socket.emit('engine-command', `position fen ${fen}`);
        socket.emit('engine-command', `go depth ${engineDepth}`);
    }
}

// Socket events
socket.on('engine-response', (data) => {
    if (data.startsWith('bestmove')) {
        updateEngineStatus('active', 'Engine Ready');
        
        // Safety: If user undid a move while engine was thinking, 
        // it might now be player's turn. Ignore the engine's move.
        if (opponentType === 'engine' && game.turn() === playerColor[0]) return;

        const moveStr = data.split(' ')[1];
        if (moveStr && moveStr !== '0000' && moveStr !== '(none)') {
            const from = moveStr.substring(0, 2);
            const to = moveStr.substring(2, 4);
            const promo = moveStr.substring(4, 5);
            
            const moveOptions = { from, to };
            if (promo) {
                moveOptions.promotion = promo;
            } else {
                // Only add 'q' as fallback if it's actually a promotion rank for a pawn
                const piece = game.get(from);
                if (piece && piece.type === 'p' && (to[1] === '8' || to[1] === '1')) {
                    moveOptions.promotion = 'q';
                }
            }
            
            try {
                const move = game.move(moveOptions);
                if (move) {
                    saveMove(move);
                    renderBoard();
                    updateCapturedPieces();
                    playSound('move');
                    
                    if (game.isGameOver()) {
                        handleGameOver();
                    }
                }
            } catch (e) {
                console.error('Invalid move from engine:', moveOptions, e);
                updateEngineStatus('error', 'Engine Error: Invalid Move');
            }
        }
    } else if (data.startsWith('info')) {
        // Parse engine info for evaluation
        const scoreMatch = data.match(/score cp (-?\d+)/);
        if (scoreMatch) {
            const score = parseInt(scoreMatch[1]) / 100;
            updateEvaluation(score);
        }
    } else if (data === 'readyok') {
        updateEngineStatus('active', 'Engine Ready');
    }
});

// ===== Settings =====
function loadSettings() {
    const saved = localStorage.getItem('chessSettings');
    if (saved) {
        const settings = JSON.parse(saved);
        boardTheme = settings.boardTheme || 'classic';
        pieceSet = settings.pieceSet || 'cburnett';
        engineDepth = settings.engineDepth || 4;
        showHints = settings.showHints !== false;
        soundEnabled = settings.soundEnabled !== false;
        
        // Apply settings - check if elements exist first
        const pieceSetEl = document.getElementById('piece-set');
        if (pieceSetEl) pieceSetEl.value = pieceSet;
        
        const engineDepthEl = document.getElementById('engine-depth');
        if (engineDepthEl) engineDepthEl.value = engineDepth;
        
        const depthValEl = document.getElementById('depth-val');
        if (depthValEl) depthValEl.textContent = engineDepth;
        
        const showHintsEl = document.getElementById('show-hints');
        if (showHintsEl) showHintsEl.checked = showHints;
        
        const soundEnabledEl = document.getElementById('sound-enabled');
        if (soundEnabledEl) soundEnabledEl.checked = soundEnabled;
        
        // Update theme buttons
        document.querySelectorAll('.theme-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.theme === boardTheme);
        });
        
        applyTheme(boardTheme);
    }
}

function saveSettings() {
    const settings = {
        boardTheme,
        pieceSet,
        engineDepth,
        showHints,
        soundEnabled
    };
    localStorage.setItem('chessSettings', JSON.stringify(settings));
}

function applyTheme(theme) {
    document.body.className = `theme-${theme}`;
    boardTheme = theme;
}

// ===== Audio =====
function playSound(type) {
    if (!soundEnabled) return;
    
    // Simple beep sounds using Web Audio API
    const audioContext = new (window.AudioContext || window.webkitAudioContext)();
    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();
    
    oscillator.connect(gainNode);
    gainNode.connect(audioContext.destination);
    
    switch (type) {
        case 'move':
            oscillator.frequency.value = 400;
            gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.1);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 0.1);
            break;
        case 'game-end':
            oscillator.frequency.value = 600;
            gainNode.gain.setValueAtTime(0.2, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.5);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 0.5);
            break;
    }
}

// ===== Event Listeners =====
function setupEventListeners() {
    // Game controls
    document.getElementById('new-game-btn').addEventListener('click', startNewGame);
    document.getElementById('play-again-btn').addEventListener('click', startNewGame);
    document.getElementById('flip-board-btn').addEventListener('click', flipBoard);
    document.getElementById('undo-btn').addEventListener('click', undoMove);
    
    const undoBtnSidebar = document.getElementById('undo-btn-sidebar');
    if (undoBtnSidebar) undoBtnSidebar.addEventListener('click', undoMove);
    
    // Move navigation
    document.getElementById('move-first').addEventListener('click', () => goToMove(-1));
    document.getElementById('move-prev').addEventListener('click', () => goToMove(currentMoveIndex - 1));
    document.getElementById('move-next').addEventListener('click', () => goToMove(currentMoveIndex + 1));
    document.getElementById('move-last').addEventListener('click', () => goToMove(moveHistory.length - 1));
    
    // Settings modal
    const settingsBtn = document.getElementById('settings-btn');
    const settingsModal = document.getElementById('settings-modal');
    const closeModal = document.querySelector('.close-modal');
    const saveSettingsBtn = document.getElementById('save-settings');
    const resetSettingsBtn = document.getElementById('reset-settings');
    
    settingsBtn.addEventListener('click', () => {
        settingsModal.classList.add('active');
    });
    
    closeModal.addEventListener('click', () => {
        settingsModal.classList.remove('active');
    });
    
    settingsModal.addEventListener('click', (e) => {
        if (e.target === settingsModal) {
            settingsModal.classList.remove('active');
        }
    });
    
    // Theme buttons
    document.querySelectorAll('.theme-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.theme-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            applyTheme(btn.dataset.theme);
        });
    });
    
    // Piece set
    document.getElementById('piece-set').addEventListener('change', (e) => {
        pieceSet = e.target.value;
        renderBoard();
    });
    
    // Engine depth
    document.getElementById('engine-depth').addEventListener('input', (e) => {
        engineDepth = parseInt(e.target.value);
        document.getElementById('depth-val').textContent = engineDepth;
    });
    
    // Checkboxes
    document.getElementById('show-hints').addEventListener('change', (e) => {
        showHints = e.target.checked;
    });
    
    document.getElementById('sound-enabled').addEventListener('change', (e) => {
        soundEnabled = e.target.checked;
    });
    
    // Save settings
    saveSettingsBtn.addEventListener('click', () => {
        saveSettings();
        settingsModal.classList.remove('active');
    });

    // Game sync
    const forceMoveBtn = document.getElementById('force-move-btn');
    const manualMoveInput = document.getElementById('manual-move-input');
    const loadFenBtn = document.getElementById('load-fen-btn');
    const fenInput = document.getElementById('fen-input');

    forceMoveBtn.addEventListener('click', () => {
        const moveStr = manualMoveInput.value.trim();
        if (!moveStr) return;

        // If it's the engine's turn, we assume the user wants to REPLACE the engine's 
        // intended move with this manual one.
        if (opponentType === 'engine' && game.turn() !== playerColor[0]) {
            // Check if we should undo first (if engine already made a move in moveHistory)
            if (moveHistory.length > 0 && moveHistory[currentMoveIndex].move.color !== playerColor[0]) {
                game.undo();
                currentMoveIndex--;
            }
        }

        const move = game.move(moveStr);
        if (move) {
            manualMoveInput.value = '';
            saveMove(move);
            renderBoard();
            playSound('move');
            
            // After manual move, if it's now engine's turn again, it will think
            checkEngineMove();
        } else {
            manualMoveInput.classList.add('error');
            setTimeout(() => manualMoveInput.classList.remove('error'), 500);
        }
    });

    manualMoveInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') forceMoveBtn.click();
    });

    loadFenBtn.addEventListener('click', () => {
        const fen = fenInput.value.trim();
        if (game.load(fen)) {
            currentMoveIndex = -1;
            moveHistory = [{ move: { san: 'Position Load' }, fen: fen }];
            currentMoveIndex = 0;
            renderBoard();
            updateMoveHistory();
            fenInput.value = '';
        } else {
            fenInput.classList.add('error');
            setTimeout(() => fenInput.classList.remove('error'), 500);
        }
    });

    // Edit Mode Toggle
    const toggleEditBtn = document.getElementById('toggle-edit-mode');
    const editTools = document.getElementById('edit-tools');
    const toggleTurnBtn = document.getElementById('toggle-turn-btn');

    toggleEditBtn.addEventListener('click', () => {
        isEditMode = !isEditMode;
        toggleEditBtn.classList.toggle('active', isEditMode);
        editTools.classList.toggle('hidden', !isEditMode);
        document.body.classList.toggle('edit-mode-active', isEditMode);
    });

    toggleTurnBtn.addEventListener('click', () => {
        const fen = game.fen();
        const parts = fen.split(' ');
        parts[1] = parts[1] === 'w' ? 'b' : 'w';
        game.load(parts.join(' '));
        renderBoard();
        checkEngineMove();
    });

    // Pallet piece dragging
    document.querySelectorAll('.pallet-piece').forEach(img => {
        img.addEventListener('dragstart', (e) => {
            palletDraggedPiece = img.dataset.piece;
            e.dataTransfer.setData('text/plain', palletDraggedPiece);
        });
        img.addEventListener('dragend', () => {
            palletDraggedPiece = null;
        });
    });
    
    // Reset settings
    resetSettingsBtn.addEventListener('click', () => {
        boardTheme = 'classic';
        pieceSet = 'cburnett';
        engineDepth = 4;
        showHints = true;
        soundEnabled = true;
        
        const pieceSetEl = document.getElementById('piece-set');
        if (pieceSetEl) pieceSetEl.value = 'cburnett';
        
        const engineDepthEl = document.getElementById('engine-depth');
        if (engineDepthEl) engineDepthEl.value = 4;
        
        const depthValEl = document.getElementById('depth-val');
        if (depthValEl) depthValEl.textContent = '4';
        
        const showHintsEl = document.getElementById('show-hints');
        if (showHintsEl) showHintsEl.checked = true;
        
        const soundEnabledEl = document.getElementById('sound-enabled');
        if (soundEnabledEl) soundEnabledEl.checked = true;
        
        document.querySelectorAll('.theme-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.theme === 'classic');
        });
        
        applyTheme('classic');
        renderBoard();
        saveSettings();
    });
    
    // Keyboard shortcuts
    document.addEventListener('keydown', (e) => {
        if (e.target.tagName === 'INPUT' || e.target.tagName === 'SELECT') return;
        
        switch (e.key) {
            case 'ArrowLeft':
                goToMove(currentMoveIndex - 1);
                break;
            case 'ArrowRight':
                goToMove(currentMoveIndex + 1);
                break;
            case 'z':
                if (e.ctrlKey || e.metaKey) {
                    e.preventDefault();
                    undoMove();
                }
                break;
            case 'f':
                flipBoard();
                break;
            case 'n':
                if (e.ctrlKey || e.metaKey) {
                    e.preventDefault();
                    startNewGame();
                }
                break;
        }
    });
}

// Start the app
init();
