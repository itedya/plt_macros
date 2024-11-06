## PHP-like Templating

Usage:

```rust
fn main() {
    plt_template!(some_template;r#"
        <!DOCTYPE html>
        <html>
        <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width" />
        <title><?= title ?></title>
        </head>
        <body>
            <?rs for i in 0..10 { ?>
                <?= body ?>
            <?rs } ?>
        </body>
        </html>
    "#;title: &str, body: &str);

    let response = some_template("this is a title", "some body").unwrap();
}

```