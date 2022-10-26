import {applyMiddleware, combineReducers, createStore} from "redux";
import thunk from "redux-thunk"
import {reducer as form} from "redux-form"

import {switcher} from "./redusers/switcher";
import {game} from "./redusers/game";

let reducers = combineReducers({
  form,
  switcher,
  game,
})

let store = createStore(reducers, applyMiddleware(thunk))

export default store
window.store = store