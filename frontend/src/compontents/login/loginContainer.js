import Login from "./login";
import {onClick, update, back} from "../../store/redusers/login";
import {connect} from "react-redux";

const mapStateToProps = (state) => {
  return {
    state: state.login
  }
}

const LoginContainer =
  connect(mapStateToProps,
    {update, onClick, back})(Login)

export default LoginContainer