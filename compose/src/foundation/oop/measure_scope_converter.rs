use crate::foundation::measure_scope::MeasureScope;

pub(crate) trait MeasureScopeConverter {
    fn as_measure_scope(&self) -> &dyn MeasureScope;
    fn as_measure_scope_mut(&mut self) -> &mut dyn MeasureScope;
}