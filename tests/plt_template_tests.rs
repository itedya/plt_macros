extern crate plt_macros;

#[cfg(test)]
mod tests {
    use plt_macros::plt_template;

    #[test]
    fn it_works() {
        plt_template!(some_template;r#"
        <!DOCTYPE html>
        <html>
        <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width" />
        <title><?= title ?></title>
        </head>
        <body>
            <?= body ?>
        </body>
        </html>
        "#;title: &str, body: &str);

        dbg!(some_template("this is a title", "some body").unwrap());
    }
}