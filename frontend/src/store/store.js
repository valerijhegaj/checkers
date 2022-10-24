import {login} from "./redusers/login";
import {createUser} from "./redusers/createUser";
import {switcher} from "./redusers/switcher";
import {startGame} from "./redusers/startGame";
import {joinGame} from "./redusers/joinGame";
import {game} from "./redusers/game";
import {applyMiddleware, combineReducers, createStore} from "redux";
import thunk from "redux-thunk"

let reducers = combineReducers({login, createUser, switcher, startGame, joinGame, game})

let store = createStore(reducers, applyMiddleware(thunk))

export default store
window.store = store