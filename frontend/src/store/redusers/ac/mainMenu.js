import {switcherCondition, updateSwitcher} from "../switcher";

export const singleplayer = () => async (dispatch) => {
  dispatch(updateSwitcher(switcherCondition.createSingleplayer))
}

export const multiplayer = () => async (dispatch) => {
  dispatch(updateSwitcher(switcherCondition.multiplayer))
}