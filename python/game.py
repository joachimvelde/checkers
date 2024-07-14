import pygame
import time
import random
import copy
import math
from bot import *

# Board constants
WIDTH = 800
HEIGHT = 800
ROWS = 8
COLS = 8
TILE_WIDTH = WIDTH / COLS
TILE_HEIGHT = HEIGHT / ROWS
PIECE_RADIUS = 30
KING_MARK_RADIUS = 10

PIECE_VALUE = 3
KING_VALUE = 2
CENTER_REWARD = 5

"""
The board is represented as a two-dimensional array with pieces represented as characters,
and empty tiles as None.

Moves are represented as a tuple containing two tuples on the form: ((from), (to)).

Black will try to maximise the state value, while red will want to minimise it.
"""

# --- Game functions ---
def init_board():
    board = [[None, 'B', None, 'B', None, 'B', None, 'B'],
             ['B', None, 'B', None, 'B', None, 'B', None],
             [None, 'B', None, 'B', None, 'B', None, 'B'],
             [None, None, None, None, None, None, None, None],
             [None, None, None, None, None, None, None, None],
             ['R', None, 'R', None, 'R', None, 'R', None],
             [None, 'R', None, 'R', None, 'R', None, 'R'],
             ['R', None, 'R', None, 'R', None, 'R', None]]
    return board

def within_bounds(position):
    return position[0] >= 0 and position[0] < ROWS and position[1] >= 0 and position[1] < COLS

def get_moves(board, position):
    assert position[0] >= 0 and position[0] < ROWS and position[1] >= 0 and position[1] < COLS
    assert board[position[0]][position[1]] is not None

    dir = ((None, None), (None, None)) # Avoid editor warnings
    player = board[position[0]][position[1]]
    if player == 'R': dir = ((-1, -1), (-1, 1)) # Red can move up left or up right
    if player == 'B': dir = (( 1, -1), ( 1, 1)) # Black can move down left or down right
    if player == "RK" or player == "BK": dir = ((1, 1), (1, -1), (-1, 1), (-1, -1)) # Kings can do both

    moves = []

    for dr, dc in dir:
        dest = (position[0] + dr, position[1] + dc)

        # Check bounds
        if not within_bounds(dest): continue

        # Empty moves then capturing moves
        if board[dest[0]][dest[1]] is None:
            moves.append((position, dest))
        elif board[dest[0]][dest[1]] != player:
            dest = (dest[0] + dr, dest[1] + dc)
            # Check for out of bounds
            if within_bounds(dest) and board[dest[0]][dest[1]] is None:
                moves.append((position, dest))

    return moves

def get_all_moves(board, player):
    moves = []
    for r in range(ROWS):
        for c in range(COLS):
            if board[r][c] == player or board[r][c] == player + 'K':
                moves.extend(get_moves(board, (r, c)))
    return moves

def is_valid_move(board, player, move):
    return move in get_all_moves(board, player)

def apply_move(board, move):
    src = move[0]
    dst = move[1]
    board[dst[0]][dst[1]] = board[src[0]][src[1]]
    board[src[0]][src[1]] = None

    # Handle captures
    if abs(dst[0] - src[0]) > 1:
        board[(src[0] + dst[0]) // 2][(src[1] + dst[1]) // 2] = None

    if dst[0] == 0 and board[dst[0]][dst[1]] == 'R': board[dst[0]][dst[1]] = "RK"
    if dst[0] == 7 and board[dst[0]][dst[1]] == 'B': board[dst[0]][dst[1]] = "BK"

def is_game_over(board):
    return get_winner(board) != None

def get_winner(board):
    blacks = 0
    reds = 0

    for r in range(ROWS):
        for c in range(COLS):
            if board[r][c] == 'B' or board[r][c] == "BK": blacks += 1
            if board[r][c] == 'R' or board[r][c] == "RK": reds   += 1

    if blacks == 0: return 'R'
    if reds   == 0: return 'B'
    return None



# - Drawing functions -
def draw_tiles(surface):
    for r in range(ROWS):
        for c in range(COLS):
            rect = pygame.Rect(c * TILE_WIDTH, r * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT)
            colour = pygame.Color(255, 255, 255) if (r + c) % 2 == 0 else pygame.Color(0, 0, 0)
            pygame.draw.rect(surface, colour, rect)

def draw_pieces(board, surface):
    for r in range(ROWS):
        for c in range(COLS):
            if board[r][c] is None: continue

            colour = pygame.Color(255, 0, 120) if board[r][c][0] == 'B' else pygame.Color(255, 0, 0)
            center = (TILE_WIDTH / 2 + c * TILE_WIDTH, TILE_HEIGHT / 2 + r * TILE_HEIGHT)
            pygame.draw.circle(surface, colour, center, PIECE_RADIUS)

            if board[r][c] == 'BK' or board[r][c] == 'RK':
                pygame.draw.circle(surface, pygame.Color(255, 255, 255), center, KING_MARK_RADIUS)

def draw_board(board, surface):
    draw_tiles(surface)
    draw_pieces(board, surface)



# --- Minimax functions ---
def state_value(board):
    value = 0
    for r in range(ROWS):
        for c in range(COLS):
            if board[r][c] == 'B': value += PIECE_VALUE
            if board[r][c] == 'R': value -= PIECE_VALUE
            if board[r][c] == 'BK': value += KING_VALUE
            if board[r][c] == 'RK': value -= KING_VALUE

            # Center control reward
            if (board[r][c] == 'B' or board[r][c] == 'BK') and (r > 2 and r < 6 and c > 2 and c < 6):
                value += CENTER_REWARD
            if (board[r][c] == 'R' or board[r][c] == 'RK') and (r > 2 and r < 6 and c > 2 and c < 6):
                value -= CENTER_REWARD

    return value

# Returns a new board (deep copy) with applied move
def result(board, move):
    new_board = copy.deepcopy(board)
    apply_move(new_board, move)
    return new_board

def minimax(board, player_colour, depth):
    if is_game_over(board) or depth == 0:
        return state_value(board), None

    best_move = None

    if player_colour == 'B':
        value = -math.inf
        for move in get_all_moves(board, player_colour):
            next_value, _ = minimax(result(board, move), 'R', depth-1)
            if next_value > value:
                best_move = move
                value = next_value
        return value, best_move

    if player_colour == 'R':
        value = math.inf
        for move in get_all_moves(board, player_colour):
            next_value, _ = minimax(result(board, move), 'B', depth-1)
            if next_value < value:
                best_move = move
                value = next_value
        return value, best_move


class App:
    def __init__(self):
        self.running = True
        pygame.init()
        self.screen = pygame.display.set_mode((WIDTH, HEIGHT))
        clock = pygame.time.Clock()

        self.board = init_board()
        self.black = Bot('B')
        self.red = Bot('R')
        self.player_turn = self.black # Black starts

        while self.running:
            self.update()
            self.draw()
            clock.tick(60) # 60 fps

    def update(self):
        # Handle inputs
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.running = False

        # Handle each turn
        if is_game_over(self.board):
            print(get_winner(self.board), "wins!")
            time.sleep(1)
            self.board = init_board()
        else:
            move = self.player_turn.get_move(self.board)
            apply_move(self.board, move)
            self.player_turn = self.red if self.player_turn.colour == 'B' else self.black

    def draw(self):
        draw_board(self.board, self.screen)
        pygame.display.flip()

def main():
    App()

if __name__ == "__main__":
    main()

