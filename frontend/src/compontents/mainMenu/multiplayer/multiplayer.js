import s from "../../common/menu/Menu.module.css"

export const Multiplayer = (props) => {
  const buttonStyle = `${s.button} ${s.button_text}`
  return (
    <div className={s.body}>
      <div className={`${s.header}`}>Checkers</div>
      <button className={buttonStyle} onClick={props.create}>
        Create
      </button>
      <button className={buttonStyle} onClick={props.join}>
        Join
      </button>
    </div>
  )
}