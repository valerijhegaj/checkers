import {MainMenu} from "./mainMenu";
import {join, start} from "../../store/redusers/mainMenu";
import {connect} from "react-redux";

const MainMenuContainer = connect(() => {return{state: undefined}}, {join, start})(MainMenu)

export default MainMenuContainer