use crate::action::Action;
use crate::prelude::{ActionSeed, Output, Runner};
use crate::runner::{BoxedRunner, CancellationToken};

pub trait Remake<I1, O1, O2, ActionOrSeed> {
    fn remake<F, R>(self, f: F) -> ActionOrSeed
        where
            F: FnOnce(BoxedRunner, Output<O1>, CancellationToken, Output<O2>) -> R + 'static,
            R: Runner + 'static;
}

impl<I1, O1, O2> Remake<I1, O1, O2, ActionSeed<I1, O2>> for ActionSeed<I1, O1>
    where
        I1: 'static,
        O1: 'static,
        O2: 'static
{
    #[inline]
    fn remake<F, R>(self, f: F) -> ActionSeed<I1, O2>
        where
            F: FnOnce(BoxedRunner, Output<O1>, CancellationToken, Output<O2>) -> R + 'static,
            R: Runner + 'static,
    {
        ActionSeed::new(|input, token, output| {
            let o1 = Output::default();
            let runner = self.create_runner(input, token.clone(), o1.clone());
            f(runner, o1, token, output)
        })
    }
}

impl<I1, O1, O2> Remake<I1, O1, O2, Action<I1, O2>> for Action<I1, O1>
    where
        I1: 'static,
        O1: 'static,
        O2: 'static
{
    #[inline]
    fn remake<F, R>(self, f: F) -> Action<I1, O2>
        where
            F: FnOnce(BoxedRunner, Output<O1>, CancellationToken, Output<O2>) -> R + 'static,
            R: Runner + 'static,
    {
        self.1.remake(f).with(self.0)
    }
}