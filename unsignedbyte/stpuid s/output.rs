#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
mod lib {

    #[doc = "alternative????"]
    pub fn eggman(eggs: u64) -> String {
        //!lol i touch
        let together: String =
            {
                let res =
                    ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&["I have ",
                                                                          " eggs"],
                                                                        &match (&eggs,)
                                                                             {
                                                                             (arg0,)
                                                                             =>
                                                                             [::core::fmt::ArgumentV1::new(arg0,
                                                                                                           ::core::fmt::Display::fmt)],
                                                                         }));
                res
            };
        together
    }
}
mod aaaaastrings {
    pub fn stringsrntfunsmhmh() {
        let sussusuussusususususss: &str = "eeeeeuuuuuuuuuuuuuuuoobl";
        let mut lessssss: String =
            String::from({
                             let res =
                                 ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&["",
                                                                                       " LOL "],
                                                                                     &match (&sussusuussusususususss,
                                                                                             &"u u u u u iiis ")
                                                                                          {
                                                                                          (arg0,
                                                                                           arg1)
                                                                                          =>
                                                                                          [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                        ::core::fmt::Display::fmt),
                                                                                           ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                        ::core::fmt::Display::fmt)],
                                                                                      }));
                             res
                         });
        let longgggggg: usize = sussusuussusususususss.len();
        lessssss.push('e');
        lessssss.push_str("bad");
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["cap:",
                                                               ", emp:",
                                                               ", cont:",
                                                               "\n"],
                                                             &match (&lessssss.capacity(),
                                                                     &lessssss.is_empty(),
                                                                     &lessssss.contains("bad"))
                                                                  {
                                                                  (arg0, arg1,
                                                                   arg2) =>
                                                                  [::core::fmt::ArgumentV1::new(arg0,
                                                                                                ::core::fmt::Display::fmt),
                                                                   ::core::fmt::ArgumentV1::new(arg1,
                                                                                                ::core::fmt::Display::fmt),
                                                                   ::core::fmt::ArgumentV1::new(arg2,
                                                                                                ::core::fmt::Display::fmt)],
                                                              }));
        };
        lessssss = lessssss.replace("uu", "o");
        let longgggggger: usize = lessssss.len();
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                             &match (&(sussusuussusususususss,
                                                                       &lessssss,
                                                                       longgggggg,
                                                                       longgggggger),)
                                                                  {
                                                                  (arg0,) =>
                                                                  [::core::fmt::ArgumentV1::new(arg0,
                                                                                                ::core::fmt::Debug::fmt)],
                                                              }));
        };
        {
            match (&41, &longgggggger) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                         "`,\n right: `",
                                                                                         "`"],
                                                                                       &match (&&*left_val,
                                                                                               &&*right_val)
                                                                                            {
                                                                                            (arg0,
                                                                                             arg1)
                                                                                            =>
                                                                                            [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                          ::core::fmt::Debug::fmt),
                                                                                             ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                          ::core::fmt::Debug::fmt)],
                                                                                        }))
                        }
                    }
                }
            }
        };
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                             &match (&(std::mem::size_of_val(&lessssss),
                                                                       lessssss.capacity()),)
                                                                  {
                                                                  (arg0,) =>
                                                                  [::core::fmt::ArgumentV1::new(arg0,
                                                                                                ::core::fmt::Debug::fmt)],
                                                              }));
        };
    }
    pub fn thsisnotasrtring() {
        let mut vectuur: Vec<usize> =
            <[_]>::into_vec(box [1, 3, 4, 5, 6, 7, 8, 9, 10, 2, 6]);
        let notavec: [usize; 5] = [1, 2, 3, 4, 5];
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                             &match (&(&vectuur,
                                                                       notavec),)
                                                                  {
                                                                  (arg0,) =>
                                                                  [::core::fmt::ArgumentV1::new(arg0,
                                                                                                ::core::fmt::Debug::fmt)],
                                                              }));
        };
        vectuur.pop();
        vectuur.push(69);
        let slicism = &vectuur[1..5];
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                             &match (&(&vectuur,
                                                                       &slicism),)
                                                                  {
                                                                  (arg0,) =>
                                                                  [::core::fmt::ArgumentV1::new(arg0,
                                                                                                ::core::fmt::Debug::fmt)],
                                                              }));
        };
    }
}
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["exclamatory\n"],
                                                         &match () {
                                                              () => [],
                                                          }));
    };
    let eggs: String = lib::eggman(5);
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                         &match (&eggs,) {
                                                              (arg0,) =>
                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                            ::core::fmt::Display::fmt)],
                                                          }));
    };
    aaaaastrings::stringsrntfunsmhmh();
    aaaaastrings::thsisnotasrtring();
}
