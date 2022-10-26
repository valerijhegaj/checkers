import s from "../common/menu/Menu.module.css"

export const StartMenu = (props) => {
  return (
    <div className={s.body}>
      <div className={`${s.header}`}>Checkers</div>
      <button className={`${s.button} ${s.button_text}`}
              onClick={props.register}>
        Register
      </button>
      <button className={`${s.button} ${s.button_text}`}
              onClick={props.login}>
        Login
      </button>
    </div>
  )
}