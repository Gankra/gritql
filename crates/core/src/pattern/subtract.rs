use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{binding::Constant, context::QueryContext};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Subtract<Q: QueryContext> {
    pub lhs: Pattern<Q>,
    pub rhs: Pattern<Q>,
}

impl<Q: QueryContext> Subtract<Q> {
    pub fn new(lhs: Pattern<Q>, rhs: Pattern<Q>) -> Self {
        Self { lhs, rhs }
    }

    pub(crate) fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let res = self.evaluate(state, context, logs)?;
        Ok(ResolvedPattern::Constant(Constant::Float(res)))
    }

    fn evaluate<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<f64> {
        let lhs = self.lhs.float(state, context, logs)?;
        let rhs = self.rhs.float(state, context, logs)?;
        let res = lhs - rhs;
        Ok(res)
    }
}

impl<Q: QueryContext> PatternName for Subtract<Q> {
    fn name(&self) -> &'static str {
        "SUBTRACT"
    }
}

impl<Q: QueryContext> Matcher<Q> for Subtract<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding_text = binding.text(&state.files)?;
        let binding_int = binding_text.parse::<f64>()?;
        let target = self.evaluate(state, context, logs)?;
        Ok(binding_int == target)
    }
}
