import LogInGameCreator from "./logInGameCreator";
import {onSubmit} from "../../../store/redusers/thunks/singleplayer";
import {connect} from "react-redux";

const mapStateToProps = (state) => {
  return {
    state: state.undefined
  }
}

const CreateSingleplayerContainer =
  connect(mapStateToProps, {onSubmit})(LogInGameCreator("Create"))

export default CreateSingleplayerContainer