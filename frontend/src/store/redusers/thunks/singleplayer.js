import {authAPI} from "../../../api/api";
import {switcherCondition, updateSwitcher} from "../switcher";
import {createConnection, updateGame} from "../game";

export const onSubmit = (formData) => async (dispatch) => {
  if (formData.gamename === "") {
    return
  }
  let response = await authAPI.createGame(formData.gamename, formData.password, {
    gamer1: 1,
    level1: 3
  }).catch(() => 1)
  if (response !== 1) {
    await authAPI.loginGame(formData.gamename, formData.password)
    dispatch(updateGame(formData.gamename))
    dispatch(createConnection(formData.gamename))
    dispatch(updateSwitcher(switcherCondition.gameScreen))
  }
}