use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathRun {
    pub children: Vec<OMathRunChild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathRunProperty>,
}

impl OMathRun {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_text(mut self, text: impl Into<String>) -> Self {
        self.children
            .push(OMathRunChild::Text(OMathText::new(text.into())));
        self
    }

    pub fn add_break(mut self, break_type: BreakType) -> Self {
        self.children
            .push(OMathRunChild::Break(Break::new(break_type)));
        self
    }

    pub fn style(mut self, value: OMathStyleType) -> Self {
        self.property = Some(self.property.unwrap_or_default().style(value));
        self
    }

    pub fn script(mut self, value: OMathScriptType) -> Self {
        self.property = Some(self.property.unwrap_or_default().script(value));
        self
    }

    pub fn align(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().align(value));
        self
    }

    pub fn manual_break(mut self, value: OMathManualBreak) -> Self {
        self.property = Some(self.property.unwrap_or_default().manual_break(value));
        self
    }

    pub fn normal_text(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().normal_text(value));
        self
    }

    pub fn literal(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().literal(value));
        self
    }
}

impl BuildXML for OMathRun {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_run()?
            .add_optional_child(&self.property)?
            .add_children(&self.children)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_run() {
        let xml = test_xml(
            &OMathRun::new()
                .style(OMathStyleType::BoldItalic)
                .script(OMathScriptType::DoubleStruck)
                .align(true)
                .manual_break(OMathManualBreak::new().align_at(1))
                .normal_text(true)
                .literal(true)
                .add_text("A"),
        );
        assert_eq!(
            xml,
            r#"<m:r><m:rPr><m:sty m:val="bi" /><m:scr m:val="double-struck" /><m:aln m:val="1" /><m:brk m:alnAt="1" /><m:nor m:val="1" /><m:lit m:val="1" /></m:rPr><m:t>A</m:t></m:r>"#
        );
    }

    case_test!(
        m_r_within_common_omath_parents,
        "m:r" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
