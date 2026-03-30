use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathArgumentProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument_size: Option<i8>,
}

impl OMathArgumentProperty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn argument_size(mut self, value: i8) -> Self {
        self.argument_size = Some(value);
        self
    }
}

impl BuildXML for OMathArgumentProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_argument_property()?
            .apply_opt(self.argument_size, |value, b| {
                b.omath_signed_integer_value("m:argSz", i32::from(value))
            })?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_argument_property() {
        let xml = test_xml(&OMathArgumentProperty::new().argument_size(-1));
        assert_eq!(xml, r#"<m:argPr><m:argSz m:val="-1" /></m:argPr>"#);
    }

    case_test!(
        m_arg_pr_within_argument_parents,
        "m:argPr" within "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup" | "m:fName"
    );
}
