use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathText {
    pub text: String,
}

impl OMathText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: escape(&text.into()),
        }
    }
}

impl BuildXML for OMathText {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .omath_text(&self.text)?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_text() {
        let xml = test_xml(&OMathText::new("x<y"));
        assert_eq!(xml, r#"<m:t>x&lt;y</m:t>"#);
    }

    case_test!(m_t_within_m_r, "m:t" within "m:r");
}
