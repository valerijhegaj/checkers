import {Game} from "./game";
import {
  onClickEmpty,
  onClickFigure
} from "../../store/redusers/game";
import {connect} from "react-redux";

const mapStateToProps = (state) => {
  return {
    state: state.game
  }
}

const GameContainer = connect(mapStateToProps, {onClickFigure, onClickEmpty})(Game)

export default GameContainer