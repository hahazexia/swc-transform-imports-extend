use std::path::PathBuf;
use serde_json::json;

use modularize_imports::{modularize_imports, PackageConfig};
use swc_ecma_parser::{EsSyntax, Syntax};
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
fn modularize_imports_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_tr| {
            modularize_imports(modularize_imports::Config {
                packages: vec![
                    (
                        "@tencent/met-ui".to_string(),
                        PackageConfig {
                            casetype: Some("lowercase".to_string()),
                            preset: None,
                            transform: "@tencent/met-ui/lib/{{member}}".into(),
                            style: Some("@tencent/met-ui/lib/{{member}}/style/index.css".into()),
                            prevent_full_import: false,
                            skip_default_conversion: true,
                            library_directory: None,
                            side_effect_position: Some("after".to_string()),
                            handle_default_import: true,
                            handle_namespace_import: true,
                        },
                    ),
                    (
                        "@tencent/met-component".to_string(),
                        PackageConfig {
                            casetype: None,
                            transform: "".into(),
                            style: None,
                            preset: Some(json!({
                                "jsPath": {
                                    "Account": "account",
                                    "useTranslation": "met-i18n"
                                },
                                "cssPath": {
                                    "Account": "account/style/index.css"
                                }
                            })),
                            prevent_full_import: false,
                            skip_default_conversion: true,
                            library_directory: None,
                            side_effect_position: None,
                            handle_default_import: true,
                            handle_namespace_import: true,
                        },
                    ),
                    (
                        "react-bootstrap".to_string(),
                        PackageConfig {
                            transform: "react-bootstrap/lib/{{member}}".into(),
                            prevent_full_import: false,
                            skip_default_conversion: false,
                            handle_default_import: true,
                            handle_namespace_import: true,
                            style: None,
                            casetype: None,
                            preset: None,
                            library_directory: None,
                            side_effect_position: None,
                        },
                    ),
                    (
                        "my-library/?(((\\w*)?/?)*)".to_string(),
                        PackageConfig {
                            transform: "my-library/{{ matches.[1] }}/{{member}}".into(),
                            prevent_full_import: false,
                            skip_default_conversion: false,
                            handle_default_import: true,
                            handle_namespace_import: true,
                            style: None,
                            casetype: None,
                            preset: None,
                            library_directory: None,
                            side_effect_position: None,
                        },
                    ),
                    (
                        "my-library-2".to_string(),
                        PackageConfig {
                            transform: "my-library-2/{{ camelCase member }}".into(),
                            prevent_full_import: false,
                            skip_default_conversion: true,
                            handle_default_import: true,
                            handle_namespace_import: true,
                            style: None,
                            casetype: None,
                            preset: None,
                            library_directory: None,
                            side_effect_position: None,
                        },
                    ),
                    (
                        "my-library-3".to_string(),
                        PackageConfig {
                            transform: "my-library-3/{{ kebabCase member }}".into(),
                            prevent_full_import: false,
                            skip_default_conversion: true,
                            handle_default_import: true,
                            handle_namespace_import: true,
                            style: None,
                            casetype: None,
                            preset: None,
                            library_directory: None,
                            side_effect_position: None,
                        },
                    ),
                    (
                        "my-library-4".to_string(),
                        PackageConfig {
                            transform: Vec::from([
                                ("foo".to_string(), "my-library-4/this_is_foo".to_string()),
                                ("bar".to_string(), "my-library-4/bar".to_string()),
                                (
                                    "use(\\w*)".to_string(),
                                    "my-library-4/{{ kebabCase member }}/{{ kebabCase \
                                     memberMatches.[1] }}"
                                        .to_string(),
                                ),
                                (
                                    "(\\w*)Icon".to_string(),
                                    "my-library-4/{{ kebabCase memberMatches.[1] }}".to_string(),
                                ),
                                (
                                    "*".to_string(),
                                    "my-library-4/{{ upperCase member }}".to_string(),
                                ),
                            ])
                            .into(),
                            prevent_full_import: false,
                            skip_default_conversion: true,
                            handle_default_import: true,
                            handle_namespace_import: true,
                            style: None,
                            casetype: None,
                            preset: None,
                            library_directory: None,
                            side_effect_position: None,
                        },
                    ),
                    (
                        "my-(module-namespace|default|mixed-(named|star))".to_string(),
                        PackageConfig {
                            transform: "transformed-{{matches.[1]}}".into(),
                            prevent_full_import: false,
                            skip_default_conversion: true,
                            handle_default_import: true,
                            handle_namespace_import: true,
                            style: None,
                            casetype: None,
                            preset: None,
                            library_directory: None,
                            side_effect_position: None,
                        },
                    ),
                    (
                        "^(\\..*)(\\.tsx?)$".to_string(),
                        PackageConfig {
                            transform: "{{matches.[1]}}.js".into(),
                            prevent_full_import: false,
                            skip_default_conversion: false,
                            handle_default_import: false,
                            handle_namespace_import: false,
                            style: None,
                            casetype: None,
                            preset: None,
                            library_directory: None,
                            side_effect_position: None,
                        },
                    ),
                ]
                .into_iter()
                .collect(),
            })
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}
