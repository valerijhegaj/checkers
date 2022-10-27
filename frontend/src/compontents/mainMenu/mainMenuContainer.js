import {MainMenu} from "./mainMenu";
import {singleplayer, multiplayer, onContinue} from "../../store/redusers/ac/mainMenu";
import {connect} from "react-redux";

const MainMenuContainer = connect(() => {
  return {state: undefined}
}, {singleplayer, multiplayer, onContinue})(MainMenu)

export default MainMenuContainer