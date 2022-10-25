import Login from "./login";
import {onSubmit} from "../../store/redusers/login";
import {connect} from "react-redux";

const mapStateToProps = () => {
  return {
    state: undefined
  }
}

const LoginContainer =
  connect(mapStateToProps, {onSubmit})(Login)

export default LoginContainer