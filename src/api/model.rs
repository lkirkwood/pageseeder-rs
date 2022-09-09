use std::fmt::Display;

pub enum HttpScheme {
    Http,
    Https
}

impl HttpScheme {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Https => "https"
        }
    }
}

impl Display for HttpScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}



struct PSGroup {
    name: String
}

impl PSGroup {
    pub fn short_name(&self) -> &str {
        return self.name.rsplit_once('-')
            .expect(&format!("Group name has no '-': {}", self.name)).1 
    }
}