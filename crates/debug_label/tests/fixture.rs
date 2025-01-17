use std::{fs::read_to_string, path::PathBuf};

use common::parse_plugin_config;
use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{EsConfig, Syntax},
        transforms::{
            base::resolver,
            compat::es2015,
            react::{react, Options, RefreshOptions},
            testing::test_fixture,
        },
    },
};
use swc_jotai_debug_label::debug_label;
use testing::fixture;

#[fixture("tests/fixtures/**/input.js")]
fn test(input: PathBuf) {
    let config =
        read_to_string(input.with_file_name("config.json")).expect("Failed to read config.json");
    let config = parse_plugin_config(&config);
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        &|t| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                debug_label(config.clone(), &PathBuf::from("atoms.ts")),
                react(
                    t.cm.clone(),
                    Some(t.comments.clone(),),
                    Options {
                        development: Some(true),
                        refresh: Some(RefreshOptions {
                            refresh_reg: "$___refreshReg$".into(),
                            refresh_sig: "$___refreshSig$".into(),
                            emit_full_signatures: false
                        }),
                        ..Default::default()
                    },
                    top_level_mark
                ),
                es2015(
                    unresolved_mark,
                    Some(t.comments.clone()),
                    Default::default()
                ),
            )
        },
        &input,
        &output,
        Default::default(),
    )
}
