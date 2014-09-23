/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::NavigatorBinding;
use dom::bindings::codegen::Bindings::NavigatorBinding::NavigatorMethods;
use dom::bindings::global::Window;
use dom::bindings::js::{JSRef, Temporary};
use dom::bindings::utils::{Reflectable, Reflector, reflect_dom_object};
use dom::navigatorinfo::NavigatorInfo;
use dom::window::Window;
use servo_util::str::DOMString;

#[deriving(Encodable)]
#[must_root]
pub struct Navigator {
    pub reflector_: Reflector //XXXjdm cycle: window->navigator->window
}

impl Navigator {
    pub fn new_inherited() -> Navigator {
        Navigator {
            reflector_: Reflector::new()
        }
    }

    pub fn new(window: JSRef<Window>) -> Temporary<Navigator> {
        reflect_dom_object(box Navigator::new_inherited(),
                           &Window(window),
                           NavigatorBinding::Wrap)
    }
}

impl<'a> NavigatorMethods for JSRef<'a, Navigator> {
    fn Product(self) -> DOMString {
        NavigatorInfo::Product()
    }

    fn TaintEnabled(self) -> bool {
        NavigatorInfo::TaintEnabled()
    }

    fn AppName(self) -> DOMString {
        NavigatorInfo::AppName()
    }

    fn AppCodeName(self) -> DOMString {
        NavigatorInfo::AppCodeName()
    }

    fn Platform(self) -> DOMString {
        NavigatorInfo::Platform()
    }
}

impl Reflectable for Navigator {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        &self.reflector_
    }
}