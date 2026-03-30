use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathSubscriptProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathSubscriptProperty {
    pub fn new(value: RunProperty) -> Self {
        Self {
            control_property: Some(OMathControlProperty::new(value)),
        }
    }
}

impl BuildXML for OMathSubscriptProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .write(XmlEvent::start_element("m:sSubPr"))?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_subscript_property() {
        let xml = test_xml(&OMathSubscriptProperty::new(RunProperty::new().italic()));
        assert_eq!(
            xml,
            r#"<m:sSubPr><m:ctrlPr><w:rPr><w:i /><w:iCs /></w:rPr></m:ctrlPr></m:sSubPr>"#
        );
    }

    case_test!(m_s_sub_pr_within_m_s_sub, "m:sSubPr" within "m:sSub");
}
