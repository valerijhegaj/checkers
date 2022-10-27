import menuStyle from "../../common/menu/Menu.module.css";
import {Field, reduxForm} from "redux-form";
import formStyle from "../../common/form/Form.module.css";

export const LogInGameForm = (props) => {
  const input = `${menuStyle.button_text} ${formStyle.input}`
  const button = `${menuStyle.button_text} ${menuStyle.button}`

  return (
    <form onSubmit={props.handleSubmit} className={menuStyle.body}>
      <div className={menuStyle.header}>Checkers</div>
      <Field placeholder={"gamename"} component={"input"}
             name={"gamename"} className={input}/>
      <Field placeholder={"password"} component={"input"}
             name={"password"} className={input}/>
      <button type={"submit"} className={button}>
        {props.SubmitName}
      </button>
      <div className={formStyle.text}>
        {props.error}
      </div>
    </form>
  )
}

const LogInGameFormRedux = reduxForm({
  form: "create game"
})(LogInGameForm)


const LogInGameCreator = (SubmitName) => (props) => {
  return (
    <LogInGameFormRedux {...props} SubmitName={SubmitName}
                        messeage={"wrong"}/>
  )
}

export default LogInGameCreator

