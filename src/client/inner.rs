
type Username = String;
type Password = String;
#[derive(Clone, Default)]
pub struct InnerClient {
    pub(crate) auth: Option<(Username, Password)>,
    pub(crate) inner: reqwest::Client,
}

impl InnerClient {
    pub fn new(username: &str, password: &str) -> Result<Self, reqwest::Error>  {
        Ok(Self {
            auth: Some((username.to_owned(), password.to_owned())),
            inner: reqwest::Client::builder().build()?,
        })
    }
}

unsafe impl Send for InnerClient {
    
}

unsafe impl Sync for InnerClient {
    
}