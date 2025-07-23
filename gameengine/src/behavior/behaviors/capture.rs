use super::*;

#[derive(Clone)]
pub struct CaptureRuleBuilder {
    pub allow_landing_on_captured: bool,
    pub allow_multi_capture: bool,
    pub allow_pass_trough_captured: bool,
    pub allow_non_maximal_multi_capture: bool,
}

impl CaptureRule for CaptureRuleBuilder {
    #[inline(always)]
    fn allow_landing_on_captured(&self) -> bool {
        self.allow_landing_on_captured
    }

    #[inline(always)]
    fn allow_multi_capture(&self) -> bool {
        self.allow_multi_capture
    }

    #[inline(always)]
    fn allow_non_maximal_multi_capture(&self) -> bool {
        self.allow_non_maximal_multi_capture
    }

    #[inline(always)]
    fn allow_pass_trough_captured(&self) -> bool {
        self.allow_pass_trough_captured
    }
}
