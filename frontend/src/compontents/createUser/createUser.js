import {Field, reduxForm} from "redux-form";
import menuStyle from "../common/menu/Menu.module.css"
import formStyle from "../common/form/Form.module.css"

const CreateUserForm = (props) => {
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
        Register
      </button>
    </form>
  )
}

const CreateUserFormRedux = reduxForm({
  form: "createUser"
})(CreateUserForm)

const CreateUser = (props) => {
  return (
    <CreateUserFormRedux onSubmit={props.onSubmit}/>
  )
}

export default CreateUser