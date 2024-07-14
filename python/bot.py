from game import *

MINIMAX_DEPTH = 2

class Bot:
    def __init__(self, colour):
        self.colour = colour

    def get_move(self, board):
        value, move = minimax(board, self.colour, MINIMAX_DEPTH)
        time.sleep(0.1)
        print(move, value)
        return move

