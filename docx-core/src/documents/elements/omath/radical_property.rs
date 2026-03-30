use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathRadicalProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_degree: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathRadicalProperty {
    pub fn hide_degree(mut self, value: bool) -> Self {
        self.hide_degree = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathRadicalProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_radical_property()?
            .apply_opt(self.hide_degree, |value, b| {
                b.omath_on_off("m:degHide", value)
            })?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_radical_property() {
        let xml = test_xml(&OMathRadicalProperty::default().hide_degree(true));
        assert_eq!(xml, r#"<m:radPr><m:degHide m:val="1" /></m:radPr>"#);
    }

    case_test!(m_rad_pr_within_m_rad, "m:radPr" within "m:rad");
}
