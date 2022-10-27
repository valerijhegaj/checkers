import {switcherCondition, updateSwitcher} from "../switcher";
import {authAPI} from "../../../api/api";
import {SubmissionError} from "redux-form";

export const onSubmit = (formData) => async (dispatch) => {
  if (formData.username === "" || formData.username === undefined) {
    throw new SubmissionError({_error: "Username can't be empty"})

  }
  let response = await authAPI.register(formData.username, formData.password).catch(() => 1)
  if (response === 1) {
    throw new SubmissionError({_error: "Username already exist"})
  }

  const maxAge = 30 * 24 * 60 * 60 // 30 days
  response = await authAPI.login(formData.username, formData.password, maxAge).catch(() => 1)
  if (response === 1) {
    throw new SubmissionError({_error: "Try to login, something went wrong :-("})
  }
  dispatch(updateSwitcher(switcherCondition.mainMenu))
}