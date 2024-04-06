//! Provides extension methods.

use std::future::Future;

use bevy::prelude::{Commands, EntityWorldMut, World};

use crate::prelude::Reactor;
use crate::reactor::Initialized;
use crate::task::ReactiveTask;
use crate::world_ptr::WorldPtr;

/// Provides a way to create and initialize [`Reactor`] in the ecs systems.
///
/// This trait is implemented in [`World`] and [`Commands`].
///
/// [`Reactor`]: crate::prelude::Reactor
/// [`World`]: bevy::prelude::World
/// [`Commands`]: bevy::prelude::Commands
pub trait ScheduleReactor<'w, Fun, Fut, Out> {
    /// Create and initialize [`Reactor`](crate::reactor::Reactor).
    fn spawn_initialized_reactor(&'w mut self, f: Fun) -> Out;
}

impl<'w, Fun, Fut> ScheduleReactor<'w, Fun, Fut, EntityWorldMut<'w>> for World
    where
        Fun: FnOnce(ReactiveTask) -> Fut + 'static,
        Fut: Future + 'static
{
    fn spawn_initialized_reactor(&'w mut self, f: Fun) -> EntityWorldMut<'w> {
        let mut flurx = Reactor::schedule(f);
        flurx.scheduler.run_sync(WorldPtr::new(self));
        self.spawn((
            Initialized,
            flurx
        ))
    }
}

impl<'w, 'b, F, Fut> ScheduleReactor<'w, F, Fut, ()> for Commands<'w, 'b>
    where
        F: FnOnce(ReactiveTask) -> Fut + Send + 'static,
        Fut: Future + 'static
{
    #[inline]
    fn spawn_initialized_reactor(&'w mut self, f: F) {
        self.add(|world: &mut World| {
            world.spawn_initialized_reactor(f);
        });
    }
}