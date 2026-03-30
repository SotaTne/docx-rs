use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::XMLBuilder;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Omml {
    pub xml: String,
}

impl Omml {
    pub fn new(xml: impl Into<String>) -> Self {
        Self { xml: xml.into() }
    }
}

impl BuildXML for Omml {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let mut builder = XMLBuilder::from(stream);
        builder.raw_xml(&self.xml)?;
        builder.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_omml_build() {
        let xml = r#"<m:oMath xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:r><m:t>x</m:t></m:r></m:oMath>"#;
        let b = Omml::new(xml).build();
        assert_eq!(str::from_utf8(&b).unwrap(), xml);
    }
}
