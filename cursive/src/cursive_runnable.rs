use crate::{backend, backends, Cursive};
use cursive_core::CursiveRunner;

/// A runnable wrapper around `Cursive`, bundling the backend initializer.
///
/// This struct embeds both `Cursive` and a backend-initializer
/// (`FnMut() -> Result<dyn Backend>`), to provide a simple `.run()` method.
///
/// This lets you pick the backend when creating the Cursive root, rather than
/// when running it.
///
/// It implements `DerefMut<Target=Cursive>`, so you can use it just like a
/// regular `Cursive` object.
pub struct CursiveRunnable {
    siv: Cursive,
    backend_init: Box<
        dyn FnMut() -> Result<
            Box<dyn backend::Backend>,
            Box<dyn std::error::Error>,
        >,
    >,
}

impl std::ops::Deref for CursiveRunnable {
    type Target = Cursive;

    fn deref(&self) -> &Cursive {
        &self.siv
    }
}

impl std::ops::DerefMut for CursiveRunnable {
    fn deref_mut(&mut self) -> &mut Cursive {
        &mut self.siv
    }
}

fn boxed(e: impl std::error::Error + 'static) -> Box<dyn std::error::Error> {
    Box::new(e)
}

impl CursiveRunnable {
    /// Creates a new Cursive wrapper, using the given backend.
    pub fn new<E, F>(mut backend_init: F) -> Self
    where
        E: std::error::Error + 'static,
        F: FnMut() -> Result<Box<dyn backend::Backend>, E> + 'static,
    {
        let siv = Cursive::new();
        let backend_init = Box::new(move || backend_init().map_err(boxed));
        Self { siv, backend_init }
    }

    /// Runs the event loop with the registered backend initializer.
    ///
    /// # Panics
    ///
    /// If the backend initialization fails.
    pub fn run(&mut self) {
        self.try_run().unwrap();
    }

    /// dummy text
    pub fn step(&mut self) {
        let mut runner = self.siv.runner((self.backend_init)().unwrap());

        runner.refresh();
        runner.step();
    }

    /// Runs the event loop with the registered backend initializer.
    pub fn try_run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.siv.try_run_with(&mut self.backend_init)
    }

    /// Creates a new Cursive wrapper using the dummy backend.
    ///
    /// Nothing will actually be output when calling `.run()`.
    pub fn dummy() -> Self {
        Self::new::<std::convert::Infallible, _>(|| {
            Ok(cursive_core::backend::Dummy::init())
        })
    }

    /// Creates a new Cursive wrapper using the ncurses backend.
    ///
    /// _Requires the `ncurses-backend` feature._
    #[cfg(feature = "ncurses-backend")]
    pub fn ncurses() -> Self {
        Self::new(backends::curses::n::Backend::init)
    }

    /// Creates a new Cursive wrapper using the panncurses backend.
    ///
    /// _Requires the `panncurses-backend` feature._
    #[cfg(feature = "pancurses-backend")]
    pub fn pancurses() -> Self {
        Self::new(backends::curses::pan::Backend::init)
    }

    /// Creates a new Cursive wrapper using the termion backend.
    ///
    /// _Requires the `termion-backend` feature._
    #[cfg(feature = "termion-backend")]
    pub fn termion() -> Self {
        Self::new(backends::termion::Backend::init)
    }

    /// Creates a new Cursive wrapper using the crossterm backend.
    ///
    /// _Requires the `crossterm-backend` feature._
    #[cfg(feature = "crossterm-backend")]
    pub fn crossterm() -> Self {
        Self::new(backends::crossterm::Backend::init)
    }

    /// Creates a new Cursive wrapper using the bear-lib-terminal backend.
    ///
    /// _Requires the `blt-backend` feature._
    #[cfg(feature = "blt-backend")]
    pub fn blt() -> Self {
        Self::new::<std::convert::Infallible, _>(|| {
            Ok(backends::blt::Backend::init())
        })
    }

    /// Creates a new Cursive wrapper using one of the available backends.
    ///
    /// Picks the first backend enabled from the list:
    /// * BearLibTerminal
    /// * Termion
    /// * Crossterm
    /// * Pancurses
    /// * Ncurses
    /// * Dummy
    pub fn default() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "blt-backend")] {
                Self::blt()
            } else if #[cfg(feature = "termion-backend")] {
                Self::termion()
            } else if #[cfg(feature = "crossterm-backend")] {
                Self::crossterm()
            } else if #[cfg(feature = "pancurses-backend")] {
                Self::pancurses()
            } else if #[cfg(feature = "ncurses-backend")] {
                Self::ncurses()
            } else {
                log::warn!("No built-it backend, falling back to Cursive::dummy().");
                Self::dummy()
            }
        }
    }
}
