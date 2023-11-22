use twsearch::scramble::{random_scramble_for_event, Event};
use cubing::alg::Alg;
use twsearch::_internal::PuzzleError;

#[swift_bridge::bridge]
mod ffi {
    // Export opaque Rust types, functions and methods for Swift to use.
    extern "Rust" {
        type Event;
        type Alg;

        type PuzzleError;

        //fn random_scramble_for_event(event: Event) -> Result<Alg, PuzzleError>;
        //#[swift_name = "randomScrambleForEvent"]
        fn randomScrambleFor(event: Event) -> Option<Alg> {
            return random_scramble_for_event(event).ok();
        }
        fn random_scramble_for_event(event: Event) -> Result<Alg, PuzzleError>;

        //#[swift_bridge(init)]
        //fn new(config: AppConfig) -> RustApp;

        //fn get_user(&self, lookup: UserLookup) -> Option<&User>;
    }
}
