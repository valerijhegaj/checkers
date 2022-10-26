import {MainMenu} from "./mainMenu";
import {singleplayer, multiplayer} from "../../store/redusers/ac/mainMenu";
import {connect} from "react-redux";

const MainMenuContainer = connect(() => {
  return {state: undefined}
}, {singleplayer, multiplayer})(MainMenu)

export default MainMenuContainer