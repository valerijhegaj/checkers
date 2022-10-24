import {StartMenu} from "./startMenu";
import {login, register} from "../../store/redusers/startMenu";
import {connect} from "react-redux";

const StartMenuContainer = connect(()=>{return {state: undefined}}, {login,register})(StartMenu)

export default StartMenuContainer