import pygame
import time
import random

# Board constants
WIDTH = 800
HEIGHT = 800
ROWS = 8
COLS = 8
TILE_WIDTH = WIDTH / COLS
TILE_HEIGHT = HEIGHT / ROWS
PIECE_RADIUS = 30
KING_MARK_RADIUS = 10

"""
The board is represented as a two-dimensional array with pieces represented as characters,
and empty tiles as None.

Moves are represented as a tuple containing two tuples on the form: ((from), (to)).
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
            if board[r][c] == player:
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

    # TODO: Implement kings
    if dst[0] == 0 and board[dst[0]][dst[1]] == 'R': board[dst[0]][dst[1]] = "RK"
    if dst[0] == 7 and board[dst[0]][dst[1]] == 'B': board[dst[0]][dst[1]] = "BK"

def is_game_over(board):
    return get_winner(board) != None

def get_winner(board):
    blacks = 0
    reds = 0

    for r in range(ROWS):
        for c in range(COLS):
            if board[r][c] == 'B': blacks += 1
            if board[r][c] == 'R': reds   += 1

    if blacks == 0: return 'R'
    if reds   == 0: return 'B'
    return None

def get_player_move(board, player):
    moves = get_all_moves(board, player)
    time.sleep(0.1)
    if len(moves) > 0:
        return moves[random.randint(0, len(moves) - 1)]
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



# --- Q-learning functions ---
def init_q():
    pass # Init the Q-table

# Chooses an action based on an epsilon-greedy policy
def choose_action(Q, state, valid_moves, epsilon):
    pass

def update_Q(Q, state, action, reward, next_state, alpha, gamma):
    pass

def get_reward(board, player):
    pass

class App:
    def __init__(self):
        self.running = True
        pygame.init()
        self.screen = pygame.display.set_mode((WIDTH, HEIGHT))
        clock = pygame.time.Clock()

        self.board = init_board()
        self.player_turn = 'B' # Black starts

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
            self.board = init_board()
        else:
            move = get_player_move(self.board, self.player_turn)
            if is_valid_move(self.board, self.player_turn, move):
                apply_move(self.board, move)
            self.player_turn = 'R' if self.player_turn == 'B' else 'B'

    def draw(self):
        draw_board(self.board, self.screen)
        pygame.display.flip()

def main():
    App()

if __name__ == "__main__":
    main()

