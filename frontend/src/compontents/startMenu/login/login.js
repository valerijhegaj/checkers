import {Field, reduxForm} from "redux-form";
import menuStyle from "../../common/menu/Menu.module.css";
import formStyle from "../../common/form/Form.module.css";

const LoginForm = (props) => {
  const input =  `${menuStyle.button_text} ${formStyle.input}`
  const button = `${menuStyle.button_text} ${menuStyle.button}`

  return (
    <form onSubmit={props.handleSubmit} className={menuStyle.body}>
      <div className={menuStyle.header}>Checkers</div>
      <Field placeholder={"username"} component={"input"}
             name={"username"} type={"username"} className={input}/>
      <Field placeholder={"password"} component={"input"}
             name={"password"} type={"password"} className={input}/>
      <button type={"submit"} className={button}>
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