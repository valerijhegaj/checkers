import {switcherCondition, updateSwitcher} from "../switcher";

export const register = () => {
  return updateSwitcher(switcherCondition.createUser)
}

export const login = () => {
  return updateSwitcher(switcherCondition.login)
}