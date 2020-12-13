//
// Created by gustavo on 16/11/2020.
//

#ifndef CHESS_POSITIONS_H
#define CHESS_POSITIONS_H

enum class CoordinatesX{
    A, B, C, D, E, F, G, H
};

enum class CoordinatesY{
    One, Two, Three, Four, Five, Six, Seven, Eight
};

class Position {
    CoordinatesX x;
    CoordinatesY y;
};

#endif //CHESS_POSITIONS_H
