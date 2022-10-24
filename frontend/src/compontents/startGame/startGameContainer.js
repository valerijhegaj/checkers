import {StartGame} from "./startGame";
import {update, createFor2, createFor1} from "../../store/redusers/startGame";
import {connect} from "react-redux";

const mapStateToProps = (state) => {
  return {
    state: state.startGame
  }
}

const StartGameContainer = connect(mapStateToProps, {update, createFor2, createFor1})(StartGame)

export default StartGameContainer