// Copyright 2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Macros for declaring and implementing the runtime APIs.

// these are part of the public API, so need to be re-exported
pub use runtime_version::{ApiId, RuntimeVersion};

/// Declare the given API traits.
///
/// # Example:
///
/// ```nocompile
/// decl_runtime_apis!{
///     pub trait Test<Event> ExtraClientSide<ClientArg> {
///         fn test<AccountId>(event: Event) -> AccountId;
///     }
/// }
/// ```
///
/// Will result in the following declaration:
///
/// ```nocompile
/// mod runtime {
///     pub trait Test {
///         fn test(event: Event) -> AccountId;
///     }
/// }
///
/// pub trait Test<Block: BlockT, Event> {
///     type Error;
///     fn test(&self, at: &BlockId<Block>, event: Event) -> Result<AccountId, Self::Error>;
/// }
/// ```
///
/// The declarations generated in the `runtime` module will be used by `impl_runtime_apis!` for implementing
/// the traits for a runtime. The other declarations should be used for implementing the interface
/// in the client.
#[macro_export]
macro_rules! decl_runtime_apis {
	(
		$(
			$( #[$attr:meta] )*
			pub trait $name:ident $(< $( $generic_param:ident $( : $generic_bound:path )* ),* >)* {
				$(
					$( #[$fn_attr:meta] )*
					fn $fn_name:ident ( $( $param_name:ident : $param_type:ty ),* )
					$( -> $return_ty:ty)*;
				)*
			}
		)*
	) => {
		$(
			decl_runtime_apis!(
				@GENERATE_RETURN_TYPES
				$( #[$attr] )*
				pub trait $name < Block: $crate::runtime_api::BlockT $( $(, $generic_param $( : $generic_bound )* )* )* > {
					$(
						$( #[$fn_attr] )*
						fn $fn_name ( $( $param_name : &$param_type, )* ) $( -> $return_ty )*;
					)*
				};
				{};
				$( $( $return_ty )*; )*
			);
		)*
		decl_runtime_apis! {
			@GENERATE_RUNTIME_TRAITS
			$(
				$( #[$attr] )*
				pub trait $name < Block: $crate::runtime_api::BlockT $( $(, $generic_param $( : $generic_bound )* )* )* > {
					$(
						$( #[$fn_attr] )*
						fn $fn_name ( $( $param_name : $param_type ),* ) $( -> $return_ty )*;
					)*
				}
			)*
		}
	};
	(@GENERATE_RETURN_TYPES
		$( #[$attr:meta] )*
		pub trait $name:ident $(< $( $generic_param_orig:ident $( : $generic_bound_orig:path )* ),* >)* {
			$(
				$( #[$fn_attr:meta] )*
				fn $fn_name:ident ( $( $param_name:ident : $param_type:ty, )* ) $( -> $return_ty:ty)*;
			)*
		};
		{ $( $result_return_ty:ty; )* };
		$return_ty_current:ty;
		$( $( $return_ty_rest:ty )*; )*
	) => {
		decl_runtime_apis!(
			@GENERATE_RETURN_TYPES
			$( #[$attr] )*
			pub trait $name $(< $( $generic_param_orig $( : $generic_bound_orig )* ),* >)* {
				$(
					$( #[$fn_attr] )*
					fn $fn_name ( $( $param_name : $param_type, )* ) $( -> $return_ty )*;
				)*
			};
			{ $( $result_return_ty; )* $crate::error::Result<$return_ty_current>; };
			$( $( $return_ty_rest )*; )*
		);
	};
	(@GENERATE_RETURN_TYPES
		$( #[$attr:meta] )*
		pub trait $name:ident $(< $( $generic_param_orig:ident $( : $generic_bound_orig:path )* ),* >)* {
			$(
				$( #[$fn_attr:meta] )*
				fn $fn_name:ident ( $( $param_name:ident : $param_type:ty, )* ) $( -> $return_ty:ty)*;
			)*
		};
		{ $( $result_return_ty:ty; )* };
		;
		$( $( $return_ty_rest:ty )*; )*
	) => {
		decl_runtime_apis!(
			@GENERATE_RETURN_TYPES
			$( #[$attr] )*
			pub trait $name $(< $( $generic_param_orig $( : $generic_bound_orig )* ),* >)* {
				$(
					$( #[$fn_attr] )*
					fn $fn_name ( $( $param_name : $param_type, )* ) $( -> $return_ty )*;
				)*
			};
			{ $( $result_return_ty; )* $crate::error::Result<()>; };
			$( $( $return_ty_rest )*; )*
		);
	};
	(@GENERATE_RETURN_TYPES
		$( #[$attr:meta] )*
		pub trait $name:ident $(< $( $generic_param_orig:ident $( : $generic_bound_orig:path )* ),* >)* {
			$(
				$( #[$fn_attr:meta] )*
				fn $fn_name:ident ( $( $param_name:ident : $param_type:ty, )* ) $( -> $return_ty:ty)*;
			)*
		};
		{ $( $result_return_ty:ty; )* };
	) => {
		decl_runtime_apis!(
			@GENERATE_CLIENT_TRAITS
			$( #[$attr] )*
			pub trait $name $(< $( $generic_param_orig $( : $generic_bound_orig )* ),* >)* {
				$(
					$( #[$fn_attr] )*
					fn $fn_name ( $( $param_name : $param_type, )* ) $( -> $return_ty )*;
				)*
			};
			{ $( $result_return_ty; )* };
		);
	};
	(@GENERATE_CLIENT_TRAITS
		$( #[$attr:meta] )*
		pub trait $name:ident $(< $( $generic_param:ident $( : $generic_bound:path )* ),* >)* {
			$(
				$( #[$fn_attr:meta] )*
				fn $fn_name:ident ( $( $param_name:ident : $param_type:ty, )* ) $( -> $return_ty:ty)*;
			)*
		};
		{ $( $result_return_ty:ty; )* };
	) => {
		$( #[$attr] )*
		#[cfg(feature = "std")]
		pub trait $name $( < $( $generic_param $( : $generic_bound )* ),* > )* : $crate::runtime_api::Core<Block> {
			$(
				$( #[$fn_attr] )*
				fn $fn_name (
					 &self,
					 at: &$crate::runtime_api::BlockId<Block>
					 $(, $param_name: $param_type )*
				) -> $result_return_ty;
			)*
		}
	};
	(@GENERATE_RUNTIME_TRAITS
		$(
			$( #[$attr:meta] )*
			pub trait $name:ident < $( $generic_param:ident $( : $generic_bound:path )* ),* > {
				$(
					$( #[$fn_attr:meta] )*
					fn $fn_name:ident ( $( $param_name:ident : $param_type:ty ),* ) $( -> $return_ty:ty )*;
				)*
			}
		)*
	) => {
		/// The API traits to implement on the runtime side.
		pub mod runtime {
			use super::*;

			$(
				$( #[$attr] )*
				pub trait $name < $( $generic_param $( : $generic_bound )* ),* > {
					$(
						$( #[$fn_attr] )*
						fn $fn_name ( $( $param_name: $param_type ),* ) $( -> $return_ty )*;
					)*
				}
			)*
		}
	};
}
