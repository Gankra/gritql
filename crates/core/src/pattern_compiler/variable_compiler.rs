use super::{compiler::NodeCompilationContext, node_compiler::NodeCompiler};
use crate::pattern::variable::{register_variable, Variable};
use anyhow::Result;
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct VariableCompiler;

impl NodeCompiler for VariableCompiler {
    type TargetPattern = Variable;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let name = node.text().trim();
        register_variable(
            name,
            context.compilation.file,
            node.range(),
            context.vars,
            context.global_vars,
            context.vars_array,
            context.scope_index,
        )
    }
}