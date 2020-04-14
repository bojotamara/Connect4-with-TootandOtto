pub struct Connect4GameBoard {
    pub rows: u8,
    pub columns: u8,
    pub tokens: [[i8; 7];6]
}

pub struct TootOttoGameBoard {
    pub rows: u8,
    pub columns: u8,
    pub tokens: [[i8; 6];4],
    pub disc_map: [[char; 6];4]
}
