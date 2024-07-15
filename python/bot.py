from game import *

MINIMAX_DEPTH = 5

class Bot:
    def __init__(self, colour):
        self.colour = colour

    def get_move(self, board):
        value, move = minimax(board, self.colour, MINIMAX_DEPTH)
        return move


class Player:
    def __init__(self, colour):
        self.colour = colour

    def get_move(self, board):
        pass
