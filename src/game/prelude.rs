pub use super::game::{CursorDirection, Game};

#[macro_export]
macro_rules! register_game_keydown_listener {
    ($fb:expr, $key:expr, $game:expr, $method:ident $(, $arg:expr)*) => {
        $fb.register_keydown_listener(
            $key,
            Box::new({
                let tmp_ptr = &mut $game as *mut Game;
                move || unsafe {
                    (*tmp_ptr).$method($($arg),*);
                }
            }),
        );
    };
}
