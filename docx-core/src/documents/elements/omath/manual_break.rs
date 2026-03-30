use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathManualBreak {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_at: Option<u8>,
}

impl OMathManualBreak {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn align_at(mut self, value: u8) -> Self {
        self.align_at = Some(value);
        self
    }
}

impl BuildXML for OMathManualBreak {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .omath_manual_break(self.align_at)?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_manual_break() {
        let xml = test_xml(&OMathManualBreak::new().align_at(1));
        assert_eq!(xml, r#"<m:brk m:alnAt="1" />"#);
    }

    case_test!(m_brk_within_supported_parents, "m:brk" within "m:boxPr" | "m:rPr");
}
