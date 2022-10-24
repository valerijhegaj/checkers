import s from "../common/menu/Menu.module.css"

export const MainMenu = (props) => {
  return (
    <div className={s.body}>
      <div className={`${s.header}`}>Checkers</div>
      <button className={`${s.button} ${s.button_text}`} onClick={() => {props.start()}}>start</button>
      <button className={`${s.button} ${s.button_text}`} onClick={() => {props.join()}}>join</button>
    </div>
  )
}