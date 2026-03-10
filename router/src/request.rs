pub struct RequestContext<'a> {
    pub host: &'a str,
    pub path: &'a str,
}