#include <stdlib.h>
#include <stdio.h>

#include "raylib.h"
#include "raymath.h"

const float WIDTH = 800;
const float HEIGHT = 800;

const size_t ROWS = 8;
const size_t COLS = 8;
const float TILE_WIDTH = WIDTH / COLS;
const float TILE_HEIGHT = HEIGHT / ROWS;

const float PIECE_RADIUS = 30.0;
const float KING_RADIUS = 10.0;
const float OUTLINE_THICCNESS = 5.0;

typedef enum {
    NONE,
    PAWN,
    KING
} Type;

typedef enum {
    P1 = 1,
    P2 = 2
} Player;

typedef struct {
    Type type;
    Player player;
    Vector2 pos;
} Piece;

// Only used for moves
typedef struct {
    size_t row;
    size_t col;
} Position;

typedef struct {
    Position from;
    Position to;
} Move;

Piece *selected_piece = NULL;

void init_board(Piece board[ROWS][COLS])
{
    for (size_t r = 0; r < ROWS; r++) {
        for (size_t c = 0; c < COLS; c++) {
            if (r < 3 & (r + c) % 2 == 1) {
                board[r][c].type = PAWN;
                board[r][c].player = P2;
            } else if (r > 4 & (r + c) % 2 != 0) {
                board[r][c].type = PAWN;
                board[r][c].player = P1;
            } else {
                board[r][c].type = NONE;
                board[r][c].player = 0;
            }
            board[r][c].pos = (Vector2) { .x = TILE_WIDTH / 2.0 + (float) c * TILE_WIDTH, .y = TILE_HEIGHT / 2.0 + (float) r * TILE_HEIGHT };
        }
    }
}

void copy_board(Piece dst[ROWS][COLS], Piece src[ROWS][COLS])
{
    printf("[IMPLEMENT ME]\n");
}

void get_moves(Piece board[ROWS][COLS], size_t row, size_t col, Move moves[], int *n_moves)
{
    Piece *p = &board[row][col];

    // TODO: Fix bug that causes nan for pieces on the edge of the board (size_t moment) Just change the boundary checks
    if (p->type == PAWN) {
        if (p->player == P1) {
            if (row - 1 > 0 && col - 1 > 0 && board[row-1][col-1].type == NONE) {
                moves[(*n_moves)++] = (Move) {
                    (Position) { .row = row - 1 },
                    (Position) { .col = col - 1 }
                };
            }
            if (row - 1 > 0 && col + 1 < COLS && board[row-1][col+1].type == NONE) {
                moves[(*n_moves)++] = (Move) {
                    (Position) { .row = row - 1 },
                    (Position) { .col = col + 1 }
                };
            }
        }

        if (p->player == P2) {
        }
    }
}

void update(Piece board[ROWS][COLS])
{
    if (IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) {
        Vector2 mouse = GetMousePosition();
        for (size_t r = 0; r < ROWS; r++) {
            for (size_t c = 0; c < COLS; c++) {
                if (board[r][c].type != NONE && board[r][c].player == P1 && CheckCollisionPointCircle(mouse, board[r][c].pos, PIECE_RADIUS)) {
                    selected_piece = &board[r][c];

                    int n_moves = 0;
                    Move moves[2];
                    get_moves(board, r, c, moves, &n_moves);
                    for (size_t i = 0; i < n_moves; i++) {
                        printf("from (%f, %f) to (%f, %f)\n", moves[i].from, moves[i].to);
                    }
                }
            }
        }
    }
}

void draw_tiles()
{
    for (size_t r = 0; r < ROWS; r++) {
        for (size_t c = 0; c < COLS; c++) {
            Rectangle tile = { .x = c * TILE_WIDTH, .y = r * TILE_HEIGHT, .width = TILE_WIDTH, .height = TILE_HEIGHT };
            Color colour = ((r + c) % 2 == 0) ? WHITE : BLACK;
            DrawRectangleRec(tile, colour);
        }
    }
}

void draw_pieces(Piece board[ROWS][COLS])
{
    for (size_t r = 0; r < ROWS; r++) {
        for (size_t c = 0; c < COLS; c++) {
            Piece *p = &board[r][c];
            if (p->type != NONE) {
                Vector2 pos = { .x = TILE_WIDTH / 2.0 + (float) c * TILE_WIDTH, .y = TILE_HEIGHT / 2.0 + (float) r * TILE_HEIGHT };
                // Mark the selected piece with an outline
                if (p == selected_piece) {
                    DrawCircleV(pos, PIECE_RADIUS + OUTLINE_THICCNESS, GREEN);
                }

                Color colour = (p->player == P1) ? RED : GRAY;
                DrawCircleV(pos, PIECE_RADIUS, colour);
                if (p->type == KING) {
                    DrawCircleV(pos, KING_RADIUS, WHITE);
                }

            }
        }
    }
}

void draw(Piece board[ROWS][COLS])
{
    BeginDrawing();
        draw_tiles();
        draw_pieces(board);
    EndDrawing();
}

int main(int argc, char *argv[])
{
    Piece board[ROWS][COLS];
    init_board(board);

    InitWindow(WIDTH, HEIGHT, "Checkers");
    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        update(board);
        draw(board);
    }

    CloseWindow();

    return 0;
}
