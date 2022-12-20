//! Traits and generic `struct`s for evaluation

pub mod float_slice;
pub mod grad;
pub mod interval;
pub mod point;
pub mod tape;

mod choice;
mod vars;

// Re-export a few things
pub use choice::Choice;
pub use float_slice::FloatSliceEval;
pub use grad::GradEval;
pub use interval::IntervalEval;
pub use point::PointEval;
pub use tape::Tape;
pub use vars::Vars;

use float_slice::FloatSliceEvalT;
use grad::GradEvalT;
use interval::IntervalEvalT;
use point::PointEvalT;

/// Represents a "family" of evaluators (JIT, interpreter, etc)
pub trait Family: Clone {
    /// Register limit for this evaluator family.
    const REG_LIMIT: u8;

    type IntervalEval: IntervalEvalT<Self>;
    type FloatSliceEval: FloatSliceEvalT<Self>;
    type PointEval: PointEvalT<Self>;
    type GradEval: GradEvalT<Self>;

    /// Recommended tile sizes for 3D rendering
    fn tile_sizes_3d() -> &'static [usize];

    /// Recommended tile sizes for 2D rendering
    fn tile_sizes_2d() -> &'static [usize];
}

/// Helper trait used to add evaluator constructions to anything implementing
/// [`Family`](Family).
pub trait Eval<F: Family> {
    fn new_point_evaluator(tape: Tape<F>) -> point::PointEval<F>;
    fn new_interval_evaluator(tape: Tape<F>) -> interval::IntervalEval<F>;
    fn new_interval_evaluator_with_storage(
        tape: Tape<F>,
        storage: interval::IntervalEvalStorage<F>,
    ) -> interval::IntervalEval<F>;
    fn new_float_slice_evaluator(
        tape: Tape<F>,
    ) -> float_slice::FloatSliceEval<F>;

    fn new_float_slice_evaluator_with_storage(
        tape: Tape<F>,
        storage: float_slice::FloatSliceEvalStorage<F>,
    ) -> float_slice::FloatSliceEval<F>;

    fn new_grad_evaluator(tape: Tape<F>) -> grad::GradEval<F>;

    fn new_grad_evaluator_with_storage(
        tape: Tape<F>,
        storage: grad::GradEvalStorage<F>,
    ) -> grad::GradEval<F>;
}

impl<F: Family> Eval<F> for F {
    /// Builds a point evaluator from the given `Tape`
    fn new_point_evaluator(tape: Tape<F>) -> point::PointEval<F> {
        point::PointEval::new(tape)
    }

    /// Builds an interval evaluator from the given `Tape`
    fn new_interval_evaluator(tape: Tape<F>) -> interval::IntervalEval<F> {
        interval::IntervalEval::new(tape)
    }

    /// Builds an interval evaluator from the given `Tape`, reusing storage
    fn new_interval_evaluator_with_storage(
        tape: Tape<F>,
        storage: interval::IntervalEvalStorage<F>,
    ) -> interval::IntervalEval<F> {
        interval::IntervalEval::new_with_storage(tape, storage)
    }

    /// Builds a float evaluator from the given `Tape`
    fn new_float_slice_evaluator(
        tape: Tape<F>,
    ) -> float_slice::FloatSliceEval<F> {
        float_slice::FloatSliceEval::new(tape)
    }

    /// Builds a float slice evaluator from the given `Tape`, reusing storage
    fn new_float_slice_evaluator_with_storage(
        tape: Tape<F>,
        storage: float_slice::FloatSliceEvalStorage<F>,
    ) -> float_slice::FloatSliceEval<F> {
        float_slice::FloatSliceEval::new_with_storage(tape, storage)
    }

    /// Builds a grad slice evaluator from the given `Tape`
    fn new_grad_evaluator(tape: Tape<F>) -> grad::GradEval<F> {
        grad::GradEval::new(tape)
    }

    /// Builds a float slice evaluator from the given `Tape`, reusing storage
    fn new_grad_evaluator_with_storage(
        tape: Tape<F>,
        storage: grad::GradEvalStorage<F>,
    ) -> grad::GradEval<F> {
        grad::GradEval::new_with_storage(tape, storage)
    }
}
