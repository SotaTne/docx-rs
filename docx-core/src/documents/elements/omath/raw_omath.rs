use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawOMath {
    pub xml: String,
}

impl RawOMath {
    pub fn new(xml: impl Into<String>) -> Self {
        Self { xml: xml.into() }
    }
}

impl BuildXML for RawOMath {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).raw_xml(&self.xml)?.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_raw_omath() {
        let xml = test_xml(&RawOMath::new("<m:r><m:t>x</m:t></m:r>"));
        assert_eq!(xml, r#"<m:r><m:t>x</m:t></m:r>"#);
    }
}
