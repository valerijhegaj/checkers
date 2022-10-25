import {switcherCondition, updateSwitcher} from "./switcher";
import {authAPI} from "../../api/api";

export const onSubmit = (formData) => async (dispatch) => {
  if (formData.username === "") {
    return
  }
  await authAPI.register(formData.username, formData.password).catch(() => 1)
  const maxAge = 30 * 24 * 60 * 60 // 30 days
  let response = await authAPI.login(formData.username, formData.password, maxAge).catch(() => 1)
  if (response !== 1) {
    dispatch(updateSwitcher(switcherCondition.mainMenu))
  }
}