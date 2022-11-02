// Copyright John Nunley, 2022.
//
// This software is distributed under the Boost Software License Version 1.0 and the Apache
// 2.0 License, at your option. See the `LICENSE-BOOST` and `LICENSE-APACHE` files in the
// root of this repository for the full text of the licenses.
//
// --------------------------------------------------------------------------------------------
//
//  Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE-BOOST or copy at
//        https://www.boost.org/LICENSE_1_0.txt)
//
// --------------------------------------------------------------------------------------------
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A macro for creating safe, fast and no-cost wrappers around DSTs.

#![cfg_attr(not(test), no_std)]

/// The whole point.
/// 
/// See the [crate documentation](index.html) for more information.
#[macro_export]
macro_rules! dst_type {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {
            dst: $dst:ty,
            formal_name: $fname:ident,
            condition: $cond:expr,
            alloc: $($alloc:tt)*
        }
    ) => {
        $(#[$attr])*
        $vis struct $name {
            inner: $dst
        }

        impl $name {
            /// Constructor for a reference that does not check the preconditions.
            /// 
            /// # Safety
            /// 
            /// The caller must ensure that the preconditions are met. 
            pub const unsafe fn new_unchecked(inner: &$dst) -> &Self {
                &*(inner as *const $dst as *const Self)
            }
            
            /// Constructor for mutable reference that does not check the preconditions.
            /// 
            /// # Safety
            /// 
            /// The caller must ensure that the `dst` satisfies the
            /// precondition `cond`.
            pub unsafe fn new_mut_unchecked(inner: &mut $dst) -> &mut Self {
                &mut *(inner as *mut $dst as *mut Self)
            }

            /// Constructor that creates a new reference after checking preconditions. 
            pub fn new(inner: &$dst) -> Option<&Self> {
                if ($cond)(&inner) {
                    Some(unsafe { Self::new_unchecked(inner) })
                } else {
                    None
                }
            }

            /// Constructor that creates a new mutable reference after checking preconditions. 
            pub fn new_mut(inner: &mut $dst) -> Option<&mut Self> {
                if ($cond)(&inner) {
                    Some(unsafe { Self::new_mut_unchecked(inner) })
                } else {
                    None
                }
            }

            paste::paste! { 
                /// Get the inner reference.
                pub fn [<as_ $fname:lower>](&self) -> &$dst {
                    &self.inner
                }
            
                /// Get the inner mutable reference.
                pub fn [<as_ $fname:lower _mut>](&mut self) -> &mut $dst {
                    &mut self.inner
                }
            }
        }

        #[cfg($($alloc)*)]
        impl $name {
            /// Constructor that creates a new owned value after checking preconditions.
            /// 
            /// # Safety
            /// 
            /// The caller must ensure that the `inner` satisfies the preconditions.
            pub unsafe fn new_boxed_unchecked(inner: ::alloc::boxed::Box<$dst>) -> ::alloc::boxed::Box<Self> {
                use ::alloc::boxed::Box;
                unsafe { Box::from_raw(Box::into_raw(inner) as *mut Self) }
            }

            /// Constructor that creates a new owned value after checking preconditions. 
            pub fn new_boxed(inner: ::alloc::boxed::Box<$dst>) -> Result<::alloc::boxed::Box<Self>, ::alloc::boxed::Box<$dst>> {
                if ($cond)(&inner) {
                    Ok(unsafe { Self::new_boxed_unchecked(inner) })
                } else {
                    Err(inner)
                }
            }

            /// Consumes the wrapper and returns the inner value. 
            pub fn into_boxed(self: ::alloc::boxed::Box<Self>) -> ::alloc::boxed::Box<$dst> {
                use ::alloc::boxed::Box;
                unsafe { Box::from_raw(Box::into_raw(self) as *mut $dst) }
            }
        }
    }
}
