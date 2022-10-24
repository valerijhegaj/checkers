import {Field, reduxForm} from "redux-form";

const CreateUserForm = (props) => {
  return (
    <form onSubmit={props.handleSubmit}>
      <Field placeholder={"username"} component={"input"}
             name={"username"} type={"username"}/>
      <Field placeholder={"password"} component={"input"}
             name={"password"} type={"password"}/>
      <Field type={"checkbox"} component={"input"}
             name={"rememberMe"}/>remember me for 30 days
      <button type={"submit"}>register</button>

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