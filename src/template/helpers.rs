use convert_case::{Case, Casing};
use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};

pub(crate) fn to_lowercase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_lowercase())?;
    Ok(())
}

pub(crate) fn to_uppercase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_uppercase())?;
    Ok(())
}

pub(crate) fn to_titlecase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_case(Case::Title))?;
    Ok(())
}

pub(crate) fn to_camelcase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_case(Case::Camel))?;
    Ok(())
}

pub(crate) fn to_pascalcase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_uppercase().to_case(Case::Pascal))?;
    Ok(())
}

pub(crate) fn to_snakecase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_case(Case::Snake))?;
    Ok(())
}

pub(crate) fn to_flatcase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_case(Case::Flat))?;
    Ok(())
}

pub(crate) fn to_kebabcase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_case(Case::Kebab))?;
    Ok(())
}

pub(crate) fn to_cobolcase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_case(Case::Cobol))?;
    Ok(())
}

pub(crate) fn to_macrocase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&param.value().render().to_case(Case::UpperSnake))?;
    Ok(())
}
