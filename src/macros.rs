/// Used for making const tile IDs in u8.
///
/// ```
/// use riichi::tu8;
///
/// assert_eq!(tu8!(E), 27u8);
/// ```
#[macro_export]
macro_rules! tu8 {
    (1m) => {
        0_u8
    };
    (2m) => {
        1_u8
    };
    (3m) => {
        2_u8
    };
    (4m) => {
        3_u8
    };
    (5m) => {
        4_u8
    };
    (6m) => {
        5_u8
    };
    (7m) => {
        6_u8
    };
    (8m) => {
        7_u8
    };
    (9m) => {
        8_u8
    };

    (1p) => {
        9_u8
    };
    (2p) => {
        10_u8
    };
    (3p) => {
        11_u8
    };
    (4p) => {
        12_u8
    };
    (5p) => {
        13_u8
    };
    (6p) => {
        14_u8
    };
    (7p) => {
        15_u8
    };
    (8p) => {
        16_u8
    };
    (9p) => {
        17_u8
    };

    (1s) => {
        18_u8
    };
    (2s) => {
        19_u8
    };
    (3s) => {
        20_u8
    };
    (4s) => {
        21_u8
    };
    (5s) => {
        22_u8
    };
    (6s) => {
        23_u8
    };
    (7s) => {
        24_u8
    };
    (8s) => {
        25_u8
    };
    (9s) => {
        26_u8
    };

    (E) => {
        27_u8
    };
    (S) => {
        28_u8
    };
    (W) => {
        29_u8
    };
    (N) => {
        30_u8
    };
    (P) => {
        31_u8
    };
    (F) => {
        32_u8
    };
    (C) => {
        33_u8
    };

    (5mr) => {
        34_u8
    };
    (5pr) => {
        35_u8
    };
    (5sr) => {
        36_u8
    };

    (?) => {
        37_u8
    };

    (-) => {
        50_u8
    };

    ($first:tt, $($left:tt),*) => {
        [$crate::tu8!($first), $($crate::tu8!($left)),*]
    };

    ($($_:tt)*) => {
        ::std::compile_error!("invalid tile pattern");
    }
}

/// Used for making const tile IDs in usize.
#[macro_export]
macro_rules! tuz {
    ($s:tt) => {
        $crate::tu8!($s) as usize
    };
    ($first:tt, $($left:tt),*) => {
        [$crate::tuz!($first), $($crate::tuz!($left)),*]
    };
}

#[macro_export]
macro_rules! checkstate {
    ($st:expr, play) => {
        match $st {
            InGameState::SelfPlay
            | InGameState::RightPlay
            | InGameState::AcrossPlay
            | InGameState::LeftPlay => true,
            _ => false,
        }
    };
    ($st:expr, tsumo) => {
        match $st {
            InGameState::SelfTsumo
            | InGameState::RightTsumo
            | InGameState::AcrossTsumo
            | InGameState::LeftTsumo => true,
            _ => false,
        }
    };
    ($st:expr, naki) => {
        match $st {
            InGameState::SelfNaki
            | InGameState::RightNaki
            | InGameState::AcrossNaki
            | InGameState::LeftNaki => true,
            _ => false,
        }
    };
}

#[macro_export]
macro_rules! state2id {
    ($v:expr) => {
        match $v {
            InGameState::SelfPlay | InGameState::SelfTsumo | InGameState::SelfNaki => 0,
            InGameState::RightPlay | InGameState::RightTsumo | InGameState::RightNaki => 1,
            InGameState::AcrossPlay | InGameState::AcrossTsumo | InGameState::AcrossNaki => 2,
            InGameState::LeftPlay | InGameState::LeftTsumo | InGameState::LeftNaki => 3,
            _ => panic!("unreachable!"),
        }
    };
}

#[macro_export]
macro_rules! id2loc {
    ($v:expr) => {
        match $v {
            0 => PlayerSeat::Selv,
            1 => PlayerSeat::Right,
            2 => PlayerSeat::Across,
            3 => PlayerSeat::Left,
            _ => panic!("unreachable!"),
        }
    };
}

#[macro_export]
macro_rules! nextplayer {
    ($v:expr) => {
        ($v + 1) % 4
    };
}

#[macro_export]
macro_rules! id2state {
    ($v:expr, play) => {
        PLAY_MAP[$v as usize]
    };
    ($v:expr, tsumo) => {
        TSUMO_MAP[$v as usize]
    };
    ($v:expr, naki) => {
        NAKI_MAP[$v as usize]
    };
}
