import {startLoad} from "../../store/redusers/switcher";
import {Switcher} from "./switcher";
import {connect} from "react-redux";

const mapStateToProps = (state) => {
  return {
    state: state.switcher
  }
}

const SwitcherContainer = connect(mapStateToProps, {startLoad})(Switcher)

export default SwitcherContainer