import {switcherCondition, updateSwitcher} from "../switcher";

export const singleplayer = () => {
  return updateSwitcher(switcherCondition.createSingleplayer)
}

export const multiplayer = () =>  {
  return  updateSwitcher(switcherCondition.multiplayer)
}

export const onContinue = () => {
  return updateSwitcher(switcherCondition.onContinue)
}