// © 2020, ETH Zurich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vir::polymorphic_vir::{ast::*, cfg, cfg::CfgMethod, Successor};

mod delete_unused_predicates;

pub use self::delete_unused_predicates::delete_unused_predicates;
