#![allow(dead_code)]

#[macro_use]
mod macros;
mod expression;
mod position;
mod statement;
mod program;

use jni::JNIEnv;
use jni::objects::JObject;
use jni_utils::JniUtils;
use viper_sys::wrappers::viper::silver::ast;

pub use self::expression::*;
pub use self::position::*;
pub use self::statement::*;
pub use self::program::*;

pub struct AstFactory<'a> {
    env: &'a JNIEnv<'a>,
    jni: JniUtils<'a>,
}

impl<'a> AstFactory<'a> {
    pub fn new(env: &'a JNIEnv) -> Self {
        let jni = JniUtils::new(env);
        AstFactory { env, jni }
    }

    // === Info ===

    fn new_no_info(&self) -> JObject {
        self.jni.unwrap_result(
            ast::NoInfo_object::with(self.env).singleton(),
        )
    }

    fn new_simple_info(&self, comments: Vec<String>) -> JObject {
        self.jni.unwrap_result(ast::SimpleInfo::with(self.env).new(
            self.jni.new_seq(
                comments.iter().map(|x| self.jni.new_string(x)).collect(),
            ),
        ))
    }

    fn new_no_trafos(&self) -> JObject {
        self.jni.unwrap_result(
            ast::NoTrafos_object::with(self.env).singleton(),
        )
    }
}
