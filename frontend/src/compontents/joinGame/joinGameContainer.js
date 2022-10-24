import JoinGame from "./joinGame";
import {update, onClick} from "../../store/redusers/joinGame"
import {connect} from "react-redux";

const mapPropsToState = (state) => {
  return {
    state: state.joinGame
  }
}

const JoinGameContainer =
  connect(mapPropsToState, {update, onClick})(JoinGame)

export default JoinGameContainer