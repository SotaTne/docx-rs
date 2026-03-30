use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathControlProperty {
    pub run_property: RunProperty,
}

impl OMathControlProperty {
    pub fn new(run_property: RunProperty) -> Self {
        Self { run_property }
    }
}

impl BuildXML for OMathControlProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_control_property()?
            .add_child(&self.run_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_control_property() {
        let xml = test_xml(&OMathControlProperty::new(
            RunProperty::new().bold().italic(),
        ));
        assert_eq!(
            xml,
            r#"<m:ctrlPr><w:rPr><w:b /><w:bCs /><w:i /><w:iCs /></w:rPr></m:ctrlPr>"#
        );
    }

    case_test!(
        m_ctrl_pr_within_supported_omath_parents,
        "m:ctrlPr" within "m:accPr" | "m:barPr" | "m:borderBoxPr" | "m:boxPr" | "m:deg" | "m:den" | "m:dPr" | "m:e" | "m:eqArrPr" | "m:fName" | "m:fPr" | "m:funcPr" | "m:groupChrPr" | "m:lim" | "m:limLowPr" | "m:limUppPr" | "m:mPr" | "m:naryPr" | "m:num" | "m:phantPr" | "m:radPr" | "m:sPrePr" | "m:sSubPr" | "m:sSubSupPr" | "m:sSupPr" | "m:sub" | "m:sup"
    );
    case_test!(
        m_ctrl_pr_not_within_unsupported_parents,
        not "m:ctrlPr" within "m:oMath" | "m:ctrlPr"
    );
}
