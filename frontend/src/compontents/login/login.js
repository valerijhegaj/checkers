import {Field, reduxForm} from "redux-form";
import menuStyle from "../common/menu/Menu.module.css";
import formStyle from "../common/form/Form.module.css";

const LoginForm = (props) => {
  return (
    <form onSubmit={props.handleSubmit} className={menuStyle.body}>

      <div className={menuStyle.header}>Checkers</div>
      <Field placeholder={"username"} component={"input"}
             name={"username"} type={"username"}
             className={`${menuStyle.button_text} ${formStyle.input}`}/>
      <Field placeholder={"password"} component={"input"}
             name={"password"} type={"password"}
             className={`${menuStyle.button_text} ${formStyle.input}`}/>
      <button type={"submit"}
              className={`${menuStyle.button_text} ${menuStyle.button}`}>
        Login
      </button>
    </form>
  )
}

const LoginFormRedux = reduxForm({
  form: "login"
})(LoginForm)

const Login = (props) => {
  return (
    <LoginFormRedux onSubmit={props.onSubmit}/>
  )
}

export default Login