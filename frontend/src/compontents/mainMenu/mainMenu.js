import s from "../common/menu/Menu.module.css"

export const MainMenu = (props) => {
  const buttonStyle = `${s.button} ${s.button_text}`
  return (
    <div className={s.body}>
      <div className={`${s.header}`}>Checkers</div>
      <button className={buttonStyle} onClick={props.singleplayer}>
        Singleplayer
      </button>
      <button className={buttonStyle} onClick={props.multiplayer}>
        Multiplayer
      </button>
      <button className={buttonStyle} onClick={props.onContinue}>
        Continue
      </button>
      <button className={buttonStyle}
              onClick={() => alert("you won't logout :)")}>
        Logout
      </button>
    </div>
  )
}