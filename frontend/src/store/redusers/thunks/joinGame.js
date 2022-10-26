import {authAPI} from "../../../api/api";
import {switcherCondition, updateSwitcher} from "../switcher";
import {createConnection, updateGame} from "../game";

export const onSubmit = (formData) => async (dispatch) => {
  if (formData.gamename === "") {
    return
  }
  const response = await authAPI.loginGame(formData.gamename, formData.password).catch(() => 1)
  if (response !== 1) {
    dispatch(updateGame(formData.gamename))
    dispatch(createConnection(formData.gamename))
    dispatch(updateSwitcher(switcherCondition.gameScreen))
  }
}