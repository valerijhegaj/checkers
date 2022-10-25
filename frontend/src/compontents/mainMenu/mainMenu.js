import s from "../common/menu/Menu.module.css"

export const MainMenu = (props) => {
  const buttonStyle = `${s.button} ${s.button_text}`
  return (
    <div className={s.body}>
      <div className={`${s.header}`}>Checkers</div>
      <button className={buttonStyle}>
        Singleplayer
      </button>
      <button className={buttonStyle} onClick={props.start}>
        Multiplayer
      </button>
      <button className={buttonStyle}>
        Logout
      </button>
    </div>
  )
}