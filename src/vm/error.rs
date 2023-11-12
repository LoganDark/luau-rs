use bstr::BString;

pub enum LuauStatus<T> {
	Ok(T),
	Yield,
	Break,
	Err(LuauError)
}

pub enum LuauError {
	Runtime(BString),
	OutOfMemory,
	Nested
}
