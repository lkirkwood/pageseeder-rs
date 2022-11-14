use indexmap::IndexMap;
use quick_xml::events::Event;

#[derive(Debug, PartialEq, Eq)]
pub struct Property {
    pub name: String,
    pub title: Option<String>,
    pub value: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropertiesFragment {
    pub id: String,
    pub title: Option<String>,
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fragment {
    pub id: String,
    pub title: Option<String>,
    pub content: Vec<Event<'static>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Fragments {
    Normal(Fragment),
    Properties(PropertiesFragment),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub fragments: IndexMap<String, Fragments>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub sections: IndexMap<String, Section>,
}
