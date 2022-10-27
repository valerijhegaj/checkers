import {authAPI} from "../../../api/api";
import {switcherCondition, updateSwitcher} from "../switcher";
import {SubmissionError} from "redux-form";

export const onSubmit = (formData) => async (dispatch) => {
  if (formData.username === "" || formData.username === undefined) {
    throw new SubmissionError({_error: "Username can't be empty"})
  }
  const maxAge = 30 * 24 * 60 * 60 // 30 days
  const response = await authAPI.login(formData.username, formData.password, maxAge)
    .catch(() => 1)
  if (response === 1) {
    throw new SubmissionError({_error: "Username or password wrong"})
  }
  dispatch(updateSwitcher(switcherCondition.mainMenu))
}
