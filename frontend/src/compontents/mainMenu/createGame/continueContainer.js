import LogInGameCreator from "./logInGameCreator";
import {onSubmit} from "../../../store/redusers/thunks/joinGame";
import {connect} from "react-redux";

const mapStateToProps = (state) => {
  return {
    state: state.undefined
  }
}

const ContinueContainer =
  connect(mapStateToProps, {onSubmit})(LogInGameCreator("Continue"))

export default ContinueContainer