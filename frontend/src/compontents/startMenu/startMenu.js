import s from "../common/menu/Menu.module.css"

export const StartMenu = (props) => {
  return (
    <div className={s.body}>
      <div className={`${s.header}`}>Checkers</div>
      <button className={`${s.button} ${s.button_text}`} onClick={() => props.register()}>register</button>
      <button className={`${s.button} ${s.button_text}`} onClick={() => props.login()}>login</button>
    </div>
  )
}