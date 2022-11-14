use indexmap::IndexMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Property {
    pub name: String,
    pub title: Option<String>,
    pub value: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Fragment {
    Normal { text: String },
    Properties { properties: Vec<Property> },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub fragments: IndexMap<String, Fragment>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub sections: IndexMap<String, Section>,
}
