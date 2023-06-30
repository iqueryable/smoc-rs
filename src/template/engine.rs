use handlebars::Handlebars;

use super::helpers;

pub struct Engine<'a> {
    handlebars: Handlebars<'a>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();

        handlebars.register_helper("lowercase", Box::new(helpers::to_lowercase));
        handlebars.register_helper("uppercase", Box::new(helpers::to_uppercase));
        handlebars.register_helper("titlecase", Box::new(helpers::to_titlecase));
        handlebars.register_helper("camelcase", Box::new(helpers::to_camelcase));
        handlebars.register_helper("pascalcase", Box::new(helpers::to_pascalcase));
        handlebars.register_helper("snakecase", Box::new(helpers::to_snakecase));
        handlebars.register_helper("flatcase", Box::new(helpers::to_flatcase));
        handlebars.register_helper("kebabcase", Box::new(helpers::to_kebabcase));
        handlebars.register_helper("cobolcase", Box::new(helpers::to_cobolcase));
        handlebars.register_helper("macrocase", Box::new(helpers::to_macrocase));

        Self { handlebars }
    }

}
