import {authAPI} from "../../../api/api";
import {switcherCondition, updateSwitcher} from "../switcher";
import {createConnection, updateGame} from "../game";
import {SubmissionError} from "redux-form";

export const onSubmit = (formData) => async (dispatch) => {
  if (formData.gamename === "" || formData.gamename === undefined) {
    throw new SubmissionError({_error: "Gamename can't be empty"})
  }
  const response = await authAPI.loginGame(formData.gamename, formData.password).catch(() => 1)
  if (response === 1) {
    throw new SubmissionError({_error: "Gamename or password wrong"})
  }
  dispatch(updateGame(formData.gamename))
  dispatch(createConnection(formData.gamename))
  dispatch(updateSwitcher(switcherCondition.gameScreen))
}