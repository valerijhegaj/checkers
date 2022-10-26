import {authAPI} from "../../../api/api";
import {switcherCondition, updateSwitcher} from "../switcher";
import {createConnection, updateGame} from "../game";

export const onSubmit = (formData) => async (dispatch) => {
  if (formData.gamename === "") {
    return
  }
  authAPI.createGame(formData.gamename, formData.password, {
    gamer: [0, 1],
    level: [0, 3]
  }).then(response => {
    authAPI.loginGame(formData.gamename, formData.password)
      .then(response => {
          dispatch(updateGame(formData.gamename))
          dispatch(createConnection(formData.gamename))
          dispatch(updateSwitcher(switcherCondition.gameScreen))
        }
      )
      .catch(() => 1)
  })
    .catch(() => 1)
}