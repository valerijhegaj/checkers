import LogInGameCreator from "./logInGameCreator";
import {onSubmit} from "../../../store/redusers/thunks/joinGame";
import {connect} from "react-redux";

const mapStateToProps = (state) => {
  return {
    state: state.undefined
  }
}

const JoinContainer =
  connect(mapStateToProps, {onSubmit})(LogInGameCreator("Join"))

export default JoinContainer