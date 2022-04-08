use sailfish::TemplateOnce;
#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct IndexTemplate {
    pub title: String,
}

impl IndexTemplate {
    pub fn new<S: AsRef<str>>(input: S) -> Self {
        Self {
            title: input.as_ref().to_string(),
        }
    }
}
