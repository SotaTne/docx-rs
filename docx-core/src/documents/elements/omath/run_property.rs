use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathRunProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<OMathStyleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<OMathScriptType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_break: Option<OMathManualBreak>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub literal: Option<bool>,
}

impl OMathRunProperty {
    pub fn style(mut self, value: OMathStyleType) -> Self {
        self.style = Some(value);
        self
    }

    pub fn script(mut self, value: OMathScriptType) -> Self {
        self.script = Some(value);
        self
    }

    pub fn align(mut self, value: bool) -> Self {
        self.align = Some(value);
        self
    }

    pub fn manual_break(mut self, value: OMathManualBreak) -> Self {
        self.manual_break = Some(value);
        self
    }

    pub fn normal_text(mut self, value: bool) -> Self {
        self.normal_text = Some(value);
        self
    }

    pub fn literal(mut self, value: bool) -> Self {
        self.literal = Some(value);
        self
    }
}

impl BuildXML for OMathRunProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_run_property()?
            .apply_opt(self.style.as_ref(), |value, b| {
                b.omath_style(&value.to_string())
            })?
            .apply_opt(self.script.as_ref(), |value, b| {
                b.omath_script(&value.to_string())
            })?
            .apply_opt(self.align, |value, b| b.omath_on_off("m:aln", value))?
            .add_optional_child(&self.manual_break)?
            .apply_opt(self.normal_text, |value, b| b.omath_on_off("m:nor", value))?
            .apply_opt(self.literal, |value, b| b.omath_on_off("m:lit", value))?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_run_property() {
        let xml = test_xml(
            &OMathRunProperty::default()
                .style(OMathStyleType::Bold)
                .script(OMathScriptType::Script)
                .align(true)
                .manual_break(OMathManualBreak::new().align_at(2))
                .normal_text(true)
                .literal(true),
        );
        assert_eq!(
            xml,
            r#"<m:rPr><m:sty m:val="b" /><m:scr m:val="script" /><m:aln m:val="1" /><m:brk m:alnAt="2" /><m:nor m:val="1" /><m:lit m:val="1" /></m:rPr>"#
        );
    }

    case_test!(m_r_pr_within_m_r, "m:rPr" within "m:r");
}
