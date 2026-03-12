pub struct RequestContext<'a> {
    pub host: Option<&'a str>,
    pub path: &'a str,
}
