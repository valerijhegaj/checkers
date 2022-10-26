import {connect} from "react-redux";
import {onSubmit} from "../../../store/redusers/thunks/createUser";
import CreateUser from "./createUser";

const CreateUserContainer = connect(() => {
  return {state: undefined}
}, {onSubmit})(CreateUser)

export default CreateUserContainer