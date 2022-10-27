import {authAPI} from "../../api/api";

const ActionTypes = {
  Switch: "switch display",
}

export const switcherCondition = {
  startLoading: 0,
  startMenu: 1,
  login: 11,
  createUser: 12,

  mainMenu: 2,

  createSingleplayer: 21,

  multiplayer: 22,
  createMultiplayer: 221,
  join: 222,

  onContinue: 23,

  gameScreen: 3,
}
const initialState = {
  condition: switcherCondition.startLoading
}

export const switcher = (state = initialState, action) => {
  switch (action.type) {
    case ActionTypes.Switch:
      return {condition: action.condition}
    default:
      return state
  }
}

export const updateSwitcher = (condition) => {
  return {type: ActionTypes.Switch, condition: condition}
}

export const startLoad = () => async (dispatch) => {
  let response = await authAPI.checkAuth().catch(
    () => {
      dispatch(updateSwitcher(switcherCondition.startMenu))
      return 1
    })
  if (response !== 1) {
    dispatch(updateSwitcher(switcherCondition.mainMenu))
  }
}