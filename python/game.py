import pygame

# Board constants
WIDTH = 800
HEIGHT = 800
ROWS = 8
COLS = 8
TILE_WIDTH = WIDTH / COLS
TILE_HEIGHT = HEIGHT / ROWS

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


def valid_move(board, move):
    pass

def apply_move(board, move):
    pass

def get_valid_moves(board, player):
    pass

def game_over(board):
    pass

def get_winner(board):
    pass

# - Drawing functions -
def draw_tiles(surface):
    for r in range(ROWS):
        for c in range(COLS):
            rect = pygame.Rect(c * TILE_WIDTH, r * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT)
            colour = pygame.Color(255, 255, 255) if (r + c) % 2 == 0 else pygame.Color(0, 0, 0)
            pygame.draw.rect(surface, colour, rect)

def draw_pieces(board, surface):
    pass

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

        while self.running:
            self.update()
            self.draw()
            clock.tick(60) # 60 fps

    def update(self):
        # Handle inputs
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.running = False

    def draw(self):
        draw_board(self.board, self.screen)
        pygame.display.flip()

def main():
    App()

if __name__ == "__main__":
    main()

