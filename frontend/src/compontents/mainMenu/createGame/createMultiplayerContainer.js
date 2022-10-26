import LogInGameCreator from "./logInGameCreator";
import {onSubmit} from "../../../store/redusers/thunks/mutliplayer";
import {connect} from "react-redux";

const mapStateToProps = (state) => {
  return {
    state: state.undefined
  }
}

const CreateMultiplayerContainer =
  connect(mapStateToProps, {onSubmit})(LogInGameCreator("Create"))

export default CreateMultiplayerContainer