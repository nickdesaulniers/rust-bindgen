#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct A {
    pub c: ::std::os::raw::c_uint,
    pub named_union: A__bindgen_ty_1,
    pub __bindgen_anon_1: A__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_Segment() {
    assert_eq!(
        ::std::mem::size_of::<A_Segment>(),
        8usize,
        concat!("Size of: ", stringify!(A_Segment))
    );
    assert_eq!(
        ::std::mem::align_of::<A_Segment>(),
        4usize,
        concat!("Alignment of ", stringify!(A_Segment))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<A_Segment>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).begin) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Segment),
            "::",
            stringify!(begin)
        )
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<A_Segment>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).end) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Segment),
            "::",
            stringify!(end)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union A__bindgen_ty_1 {
    pub f: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<A__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(A__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<A__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(A__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<A__bindgen_ty_1>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A__bindgen_ty_1),
            "::",
            stringify!(f)
        )
    );
}
impl Default for A__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union A__bindgen_ty_2 {
    pub d: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<A__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(A__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<A__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(A__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<A__bindgen_ty_2>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A__bindgen_ty_2),
            "::",
            stringify!(d)
        )
    );
}
impl Default for A__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        12usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        4usize,
        concat!("Alignment of ", stringify!(A))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<A>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize
        },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(c))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<A>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).named_union) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(A),
            "::",
            stringify!(named_union)
        )
    );
}
impl Default for A {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B {
    pub d: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_B_Segment() {
    assert_eq!(
        ::std::mem::size_of::<B_Segment>(),
        8usize,
        concat!("Size of: ", stringify!(B_Segment))
    );
    assert_eq!(
        ::std::mem::align_of::<B_Segment>(),
        4usize,
        concat!("Alignment of ", stringify!(B_Segment))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<B_Segment>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).begin) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(B_Segment),
            "::",
            stringify!(begin)
        )
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<B_Segment>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).end) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(B_Segment),
            "::",
            stringify!(end)
        )
    );
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(
        ::std::mem::size_of::<B>(),
        4usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        4usize,
        concat!("Alignment of ", stringify!(B))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<B>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize
        },
        0usize,
        concat!("Offset of field: ", stringify!(B), "::", stringify!(d))
    );
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StepSyntax {
    Keyword = 0,
    FunctionalWithoutKeyword = 1,
    FunctionalWithStartKeyword = 2,
    FunctionalWithEndKeyword = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct C {
    pub d: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: C__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union C__bindgen_ty_1 {
    pub mFunc: C__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_1: C__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct C__bindgen_ty_1__bindgen_ty_1 {
    pub mX1: f32,
    pub mY1: f32,
    pub mX2: f32,
    pub mY2: f32,
}
#[test]
fn bindgen_test_layout_C__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<C__bindgen_ty_1__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(C__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<C__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(C__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            let uninit = :: std :: mem :: MaybeUninit :: < C__bindgen_ty_1__bindgen_ty_1 > :: uninit () ;
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).mX1) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(mX1)
        )
    );
    assert_eq!(
        unsafe {
            let uninit = :: std :: mem :: MaybeUninit :: < C__bindgen_ty_1__bindgen_ty_1 > :: uninit () ;
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).mY1) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(mY1)
        )
    );
    assert_eq!(
        unsafe {
            let uninit = :: std :: mem :: MaybeUninit :: < C__bindgen_ty_1__bindgen_ty_1 > :: uninit () ;
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).mX2) as usize - ptr as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(mX2)
        )
    );
    assert_eq!(
        unsafe {
            let uninit = :: std :: mem :: MaybeUninit :: < C__bindgen_ty_1__bindgen_ty_1 > :: uninit () ;
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).mY2) as usize - ptr as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(mY2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct C__bindgen_ty_1__bindgen_ty_2 {
    pub mStepSyntax: StepSyntax,
    pub mSteps: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_C__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<C__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(C__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<C__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(C__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            let uninit = :: std :: mem :: MaybeUninit :: < C__bindgen_ty_1__bindgen_ty_2 > :: uninit () ;
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).mStepSyntax) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(mStepSyntax)
        )
    );
    assert_eq!(
        unsafe {
            let uninit = :: std :: mem :: MaybeUninit :: < C__bindgen_ty_1__bindgen_ty_2 > :: uninit () ;
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).mSteps) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(mSteps)
        )
    );
}
impl Default for C__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_C__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<C__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(C__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<C__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(C__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<C__bindgen_ty_1>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).mFunc) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1),
            "::",
            stringify!(mFunc)
        )
    );
}
impl Default for C__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct C_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_C_Segment() {
    assert_eq!(
        ::std::mem::size_of::<C_Segment>(),
        8usize,
        concat!("Size of: ", stringify!(C_Segment))
    );
    assert_eq!(
        ::std::mem::align_of::<C_Segment>(),
        4usize,
        concat!("Alignment of ", stringify!(C_Segment))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<C_Segment>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).begin) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C_Segment),
            "::",
            stringify!(begin)
        )
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<C_Segment>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).end) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C_Segment),
            "::",
            stringify!(end)
        )
    );
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        20usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        4usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe {
            let uninit = ::std::mem::MaybeUninit::<C>::uninit();
            let ptr = uninit.as_ptr();
            ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize
        },
        0usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(d))
    );
}
impl Default for C {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
