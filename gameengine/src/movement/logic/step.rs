use super::*;

#[derive(Clone)]
pub struct OneStep {}

impl StepCountStrategy for OneStep {
    #[inline(always)]
    fn get_standard(&self) -> Steps {
        STEPS_ONE
    }

    #[inline(always)]
    fn get_before_capture(&self) -> Steps {
        STEPS_ONE
    }

    #[inline(always)]
    fn get_after_capture(&self) -> Steps {
        STEPS_ONE
    }
}

#[derive(Clone)]
pub struct MaxStep {}

impl StepCountStrategy for MaxStep {
    #[inline(always)]
    fn get_standard(&self) -> Steps {
        STEPS_INFINITY
    }

    #[inline(always)]
    fn get_before_capture(&self) -> Steps {
        STEPS_INFINITY
    }

    #[inline(always)]
    fn get_after_capture(&self) -> Steps {
        STEPS_INFINITY
    }
}
