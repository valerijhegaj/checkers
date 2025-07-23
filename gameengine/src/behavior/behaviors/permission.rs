use super::*;

#[derive(Clone)]
pub struct MoveOwnedCaptureOther {}

impl MovePermissionPolicy for MoveOwnedCaptureOther {
    #[inline(always)]
    fn can_move(&self, player: &Player, piece: &Piece) -> bool {
        piece.is_owner(player)
    }

    #[inline(always)]
    fn can_capture(&self, _: &Player, captor: &Piece, captured: &Piece) -> bool {
        *captor.get_player() != *captured.get_player()
    }
}
