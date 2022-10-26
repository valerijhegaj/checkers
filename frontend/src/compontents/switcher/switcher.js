import LoginContainer from "../startMenu/login/loginContainer";
import {switcherCondition} from "../../store/redusers/switcher";
import StartMenuContainer from "../startMenu/startMenuContainer";
import MainMenuContainer from "../mainMenu/mainMenuContainer";
import GameContainer from "../game/gameContainer";
import CreateUserContainer from "../startMenu/createUser/createUserContainer";
import CreateSingleplayerContainer
  from "../mainMenu/createGame/createSingleplayerContainer";
import MultiplayerContainer
  from "../mainMenu/multiplayer/multiplayerContainer";
import CreateMultiplayerContainer
  from "../mainMenu/createGame/createMultiplayerContainer";
import JoinContainer from "../mainMenu/createGame/joinContainer";

export const Switcher = (props) => {
  switch (props.state.condition) {
    case switcherCondition.startLoading:
      props.startLoad()
      return <div>loading</div>

    case switcherCondition.startMenu:
      return <StartMenuContainer />
    case switcherCondition.login:
      return <LoginContainer />
    case switcherCondition.createUser:
      return <CreateUserContainer />

    case switcherCondition.mainMenu:
      return <MainMenuContainer />

    case switcherCondition.createSingleplayer:
      return <CreateSingleplayerContainer />

    case switcherCondition.multiplayer:
      return <MultiplayerContainer />
    case switcherCondition.createMultiplayer:
      return <CreateMultiplayerContainer />
    case switcherCondition.join:
      return <JoinContainer />

    case switcherCondition.gameScreen:
      return <GameContainer />

    default:
      return <h1>crashed :(</h1>
  }
}
