import {authAPI} from "../../api/api";
import {switcherCondition, updateSwitcher} from "./switcher";

export const onSubmit = (formData) => async (dispatch) => {
  if (formData.username === "") {
    return
  }
  const maxAge = 30 * 24 * 60 * 60 // 30 days
  let response = await authAPI.login(formData.username, formData.password, maxAge).catch(() => 1)
  if (response !== 1) {
    dispatch(updateSwitcher(switcherCondition.mainMenu))
  }
}
