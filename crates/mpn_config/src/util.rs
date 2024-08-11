use std::sync::LazyLock;

use terminal_size::terminal_size;

pub static TERMINAL_COLUMNS: LazyLock<usize> = LazyLock::new(|| {
	if cfg!(test) {
		return 80;
	}
	if let Some(size) = terminal_size() {
		return size.0 .0 as usize;
	}
	80
});
