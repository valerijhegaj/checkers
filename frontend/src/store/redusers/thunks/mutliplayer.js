import {authAPI} from "../../../api/api";
import {switcherCondition, updateSwitcher} from "../switcher";
import {createConnection, updateGame} from "../game";

export const onSubmit = (formData) => async (dispatch) => {
  if (formData.gamename === "") {
    return
  }
  let response = await authAPI.createGame(formData.gamename, formData.password, {}).catch(() => 1)
  if (response !== 1) {
    await authAPI.loginGame(formData.gamename, formData.password)
    dispatch(updateGame(formData.gamename))
    dispatch(createConnection(formData.gamename))
    dispatch(updateSwitcher(switcherCondition.gameScreen))
  }
}

export const create = () => async  (dispatch) => {
  dispatch(updateSwitcher(switcherCondition.createMultiplayer))
}

export const join = () => async (dispatch) => {
  dispatch(updateSwitcher(switcherCondition.join))
}