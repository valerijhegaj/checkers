import {authAPI} from "../../api/api";

const ActionTypes = {
  Switch: "switch display",
}

export const switcherCondition = {
  login: 0,
  createUser: 1,
  mainMenu: 2,
  startLoading: 3,
  startMenu: 4,
  startScreen: 5,
  joinScreen: 6,
  gameScreen: 7
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