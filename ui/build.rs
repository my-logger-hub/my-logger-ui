fn main() {
    ci_utils::css::CssCompiler::new("./css")
        .add_file("00-legacy.css")
        .add_file("01-tokens.css")
        .add_file("02-layout.css")
        .add_file("03-controls.css")
        .add_file("04-severity.css")
        .add_file("05-cards-tables.css")
        .add_file("06-logbar.css")
        .add_file("07-dialog.css")
        .add_file("08-tabs-states.css")
        .add_file("09-chart-misc.css")
        .add_file("99-responsive.css")
        .compile("./public/assets/app.css");
}
