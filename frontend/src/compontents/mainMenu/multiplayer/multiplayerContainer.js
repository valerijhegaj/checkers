import {connect} from "react-redux";
import {Multiplayer} from "./multiplayer";
import {
  create,
  join
} from "../../../store/redusers/thunks/mutliplayer";

const MultiplayerContainer = connect(() => {
  return {state: undefined}
}, {join, create})(Multiplayer)

export default MultiplayerContainer