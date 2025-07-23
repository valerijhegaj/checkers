use crate::{
    behavior::{
        Capture, CaptureRuleBuilder, EveryDiagonal, EveryMove, MaxStep, MoveOwnedCaptureOther,
        Movement, NoPromote, OneStep, PromoteToKing, PromoteToKingInOppositeInMove,
        ToPieceOpponentDiagonally,
    },
    model::{Board, Board8x8, MoveWithEffect, Player, bm, wm},
    rules::{BoardCreator, Rule},
};

use super::CheckersRule;
struct RussianRule {}

impl BoardCreator for RussianRule {
    fn new_board(&self) -> impl Board {
        Board8x8 {
            data: [
                [wm(), None, wm(), None, wm(), None, wm(), None],
                [None, wm(), None, wm(), None, wm(), None, wm()],
                [wm(), None, wm(), None, wm(), None, wm(), None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, bm(), None, bm(), None, bm(), None, bm()],
                [bm(), None, bm(), None, bm(), None, bm(), None],
                [None, bm(), None, bm(), None, bm(), None, bm()],
            ],
        }
    }
}

impl RussianRule {
    pub fn new_rule() -> impl Rule {
        let man_movement = Movement::new(
            ToPieceOpponentDiagonally {},
            OneStep {},
            PromoteToKing {},
            MoveOwnedCaptureOther {},
        );
        let king_movement = Movement::new(
            EveryDiagonal {},
            MaxStep {},
            NoPromote {},
            MoveOwnedCaptureOther {},
        );
        let king_capture = Capture::new(
            EveryDiagonal {},
            MaxStep {},
            NoPromote {},
            MoveOwnedCaptureOther {},
            EveryMove {},
            CaptureRuleBuilder {
                allow_landing_on_captured: false,
                allow_multi_capture: true,
                allow_non_maximal_multi_capture: false,
                allow_pass_trough_captured: false,
            },
        );
        let man_capture = Capture::new(
            ToPieceOpponentDiagonally {},
            OneStep {},
            PromoteToKingInOppositeInMove {
                recursive_capture: king_capture.clone(),
            },
            MoveOwnedCaptureOther {},
            EveryMove {},
            CaptureRuleBuilder {
                allow_landing_on_captured: false,
                allow_multi_capture: true,
                allow_non_maximal_multi_capture: false,
                allow_pass_trough_captured: false,
            },
        );

        CheckersRule {
            man_movement,
            king_movement,
            man_capture,
            king_capture,
            board_creator: RussianRule {},
            first_player: Player::White,
        }
    }
}
