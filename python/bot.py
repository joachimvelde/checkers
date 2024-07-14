class Bot:
    def __init__(self, colour, Q=None, epsilon=0.1, alpha=0.1, gamma=0.9):
        self.colour = colour
        self.Q = Q
        self.epsilon = epsilon
        self.alpha = alpha
        self.gamma = gamma

    def select_move(self, board):
        pass

    def learn(self, state, action, reward, next_state):
        pass
