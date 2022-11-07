use std::path::PathBuf;

use jsx_dom_expressions::TransformVisitor;
use swc_core::common::{chain, Mark};
use swc_core::{
    ecma::parser::{EsConfig, Syntax},
    ecma::transforms::base::resolver,
    ecma::transforms::testing::test_fixture,
    ecma::visit::as_folder,
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
fn jsx_dom_expressions_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                as_folder(TransformVisitor::new())
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
