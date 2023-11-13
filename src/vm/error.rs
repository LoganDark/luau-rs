use std::mem::MaybeUninit;

use luau_sys::luau::lua_Status;

use crate::vm::value::LuauValue;
use crate::vm::value::string::LString;
use crate::vm::value::thread::Thread;

pub unsafe fn protect<T>(writer: impl FnOnce(*mut T) -> lua_Status) -> Result<T, lua_Status> {
	let mut result = MaybeUninit::<T>::uninit();
	let status = writer(result.as_mut_ptr());

	if status == lua_Status::LUA_OK {
		Ok(result.assume_init())
	} else {
		Err(status)
	}
}

pub enum LStatus<'a, T> {
	Ok(T),
	Yield,
	Break,
	Err(LError<'a>)
}

impl<'a, T> LStatus<'a, T> {
	pub unsafe fn protect(thread: &'a Thread<'a>, proper: bool, writer: impl FnOnce(*mut T) -> lua_Status) -> Self {
		match protect(writer) {
			Ok(value) => Self::Ok(value),
			Err(lua_Status::LUA_YIELD) => Self::Yield,
			Err(lua_Status::LUA_BREAK) => Self::Break,
			Err(error) => Self::Err(LError::capture(thread, proper, error))
		}
	}
}

#[derive(Debug)]
pub enum LError<'a> {
	Runtime(LuauValue<'a, LString<'a>>),
	Syntax(LuauValue<'a, LString<'a>>),
	OutOfMemory,
	DoubleError,
	StackOverflow,
	StackUnderflow
}

impl<'a> LError<'a> {
	pub unsafe fn capture(thread: &'a Thread<'a>, proper: bool, status: lua_Status) -> Self {
		let message = if proper || matches!(status, lua_Status::LUA_ERRMEM | lua_Status::LUA_ERRERR) {
			let Some(Ok(message)) = LuauValue::pop(thread) else { return Self::DoubleError };
			let Some(Ok(message)) = message.get_string(thread) else { return Self::DoubleError };
			Some(message)
		} else {
			None
		};

		match status {
			lua_Status::LUA_ERRRUN => message.map(Self::Runtime).unwrap_or(Self::DoubleError),
			lua_Status::LUA_ERRSYNTAX => message.map(Self::Syntax).unwrap_or(Self::DoubleError),
			lua_Status::LUA_ERRMEM => Self::OutOfMemory,
			lua_Status::LUA_ERRERR => Self::DoubleError,
			_ => Self::DoubleError
		}
	}

	pub unsafe fn protect<T>(thread: &'a Thread<'a>, proper: bool, writer: impl FnOnce(*mut T) -> lua_Status) -> LResult<'a, T> {
		protect(writer).map_err(|status| Self::capture(thread, proper, status))
	}
}

pub type LResult<'a, T> = Result<T, LError<'a>>;
