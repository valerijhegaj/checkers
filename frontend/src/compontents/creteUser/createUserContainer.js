import {onClick, update, back} from "../../store/redusers/createUser";
import CreateUser from "./createUser";
import {connect} from "react-redux";

const mapStateToProps = (state) => {
  return {
    state: state.createUser
  }
}

const CreateUserContainer = connect(mapStateToProps, {update, onClick, back})(CreateUser)

export default CreateUserContainer