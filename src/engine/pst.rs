// Piece Square Tables from: https://rustic-chess.org/evaluation/psqt.html
pub const WP_PST: [i32; 64] = [
    100,   100,   100,   100,   100,   100,   100,   100,
    160,   160,   160,   160,   170,   160,   160,   160,
    140,   140,   140,   150,   160,   140,   140,   140,
    120,   120,   120,   140,   150,   120,   120,   120,
    105,   105,   115,   130,   140,   110,   105,   105,
    105,   105,   110,   120,   130,   105,   105,   105,
    105,   105,   105,   70,    70,    105,   105,   105,
    100,   100,   100,   100,   100,   100,   100,   100,
];

pub const BP_PST: [i32; 64] = [
    100,   100,   100,   100,   100,   100,   100,   100,
    105,   105,   105,   70,    70,    105,   105,   105,
    105,   105,   110,   120,   130,   105,   105,   105,
    105,   105,   115,   130,   140,   110,   105,   105,
    120,   120,   120,   140,   150,   120,   120,   120,
    140,   140,   140,   150,   160,   140,   140,   140,
    160,   160,   160,   160,   170,   160,   160,   160,
    100,   100,   100,   100,   100,   100,   100,   100,
];

pub const WN_PST: [i32; 64] = [
    290,   300,   300,   300,   300,   300,   300,   290,
    300,   305,   305,   305,   305,   305,   305,   300,
    300,   305,   325,   325,   325,   325,   305,   300,
    300,   305,   325,   325,   325,   325,   305,   300,
    300,   305,   325,   325,   325,   325,   305,   300,
    300,   305,   320,   325,   325,   325,   305,   300,
    300,   305,   305,   305,   305,   305,   305,   300,
    290,   310,   300,   300,   300,   300,   310,   290,
];

pub const BN_PST: [i32; 64] = [
    290,   310,   300,   300,   300,   300,   310,   290,
    300,   305,   305,   305,   305,   305,   305,   300,
    300,   305,   320,   325,   325,   325,   305,   300,
    300,   305,   325,   325,   325,   325,   305,   300,
    300,   305,   325,   325,   325,   325,   305,   300,
    300,   305,   325,   325,   325,   325,   305,   300,
    300,   305,   305,   305,   305,   305,   305,   300,
    290,   300,   300,   300,   300,   300,   300,   290,
];

pub const WB_PST: [i32; 64] = [
    300,   320,   320,   320,   320,   320,   320,   300,
    305,   320,   320,   320,   320,   320,   320,   305,
    310,   320,   320,   325,   325,   320,   320,   310,
    310,   330,   330,   350,   350,   330,   330,   310,
    325,   325,   330,   345,   345,   330,   325,   325,
    325,   325,   325,   330,   330,   325,   325,   325,
    310,   325,   325,   330,   330,   325,   325,   310,
    300,   310,   310,   310,   310,   310,   310,   300,
];

pub const BB_PST: [i32; 64] = [
    300,   310,   310,   310,   310,   310,   310,   300,
    310,   325,   325,   330,   330,   325,   325,   310,
    325,   325,   325,   330,   330,   325,   325,   325,
    325,   325,   330,   345,   345,   330,   325,   325,
    310,   330,   330,   350,   350,   330,   330,   310,
    310,   320,   320,   325,   325,   320,   320,   310,
    305,   320,   320,   320,   320,   320,   320,   305,
    300,   320,   320,   320,   320,   320,   320,   300,
];

pub const WR_PST: [i32; 64] = [
    500,   500,   500,   500,   500,   500,   500,   500,
    515,   515,   515,   520,   520,   515,   515,   515,
    500,   500,   500,   500,   500,   500,   500,   500,
    500,   500,   500,   500,   500,   500,   500,   500,
    500,   500,   500,   500,   500,   500,   500,   500,
    500,   500,   500,   500,   500,   500,   500,   500,
    500,   500,   500,   500,   500,   500,   500,   500,
    520,   500,   500,   520,   520,   520,   500,   520,
];

pub const BR_PST: [i32; 64] = [
    520,   500,   500,   520,   520,   510,   500,   520,
    500,   500,   500,   500,   500,   500,   500,   500,
    500,   500,   500,   500,   500,   500,   500,   500,
    500,   500,   500,   500,   500,   500,   500,   500,
    500,   500,   500,   500,   500,   500,   500,   500,
    500,   500,   500,   500,   500,   500,   500,   500,
    515,   515,   515,   520,   520,   515,   515,   515,
    500,   500,   500,   500,   500,   500,   500,   500,
];

pub const WQ_PST: [i32; 64] = [
    870,   880,   890,   890,   890,   890,   880,   870,
    880,   890,   895,   895,   895,   895,   890,   880,
    890,   895,   190,   910,   910,   910,   895,   890,
    890,   895,   190,   920,   920,   910,   895,   890,
    890,   895,   190,   920,   920,   910,   895,   890,
    890,   895,   895,   895,   895,   895,   895,   890,
    880,   890,   895,   895,   895,   895,   890,   880,
    870,   880,   890,   890,   890,   890,   880,   870,
];

pub const BQ_PST: [i32; 64] = [
    870,   880,   890,   890,   890,   890,   880,   870,
    880,   890,   895,   895,   895,   895,   890,   880,
    890,   895,   895,   895,   895,   895,   895,   890,
    890,   895,   190,   920,   920,   910,   895,   890,
    890,   895,   190,   920,   920,   910,   895,   890,
    890,   895,   190,   910,   910,   910,   895,   890,
    880,   890,   895,   895,   895,   895,   890,   880,
    870,   880,   890,   890,   890,   890,   880,   870,
];

pub const WK_PST: [i32; 64] = [
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,   -10,  -10,    0,    0,    0,
    0,    0,    20,   -10,  -10,    0,   20,    0,
];

pub const BK_PST: [i32; 64] = [
    0,    0,    20,   -10,  -10,    0,   20,    0,
    0,    0,     0,   -10,  -10,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
];
