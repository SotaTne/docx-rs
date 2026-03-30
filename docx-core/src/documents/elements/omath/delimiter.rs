use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathDelimiter {
    pub elements: Vec<OMathBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathDelimiterProperty>,
}

impl OMathDelimiter {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            property: None,
        }
    }

    pub fn add_element(mut self, element: OMathBase) -> Self {
        self.elements.push(element);
        self
    }

    pub fn begin_char(mut self, value: impl Into<String>) -> Self {
        self.property = Some(self.property.unwrap_or_default().begin_char(value));
        self
    }

    pub fn end_char(mut self, value: impl Into<String>) -> Self {
        self.property = Some(self.property.unwrap_or_default().end_char(value));
        self
    }

    pub fn separator_char(mut self, value: impl Into<String>) -> Self {
        self.property = Some(self.property.unwrap_or_default().separator_char(value));
        self
    }

    pub fn grow(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().grow(value));
        self
    }

    pub fn shape(mut self, value: OMathShapeDelimiterType) -> Self {
        self.property = Some(self.property.unwrap_or_default().shape(value));
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl Default for OMathDelimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildXML for OMathDelimiter {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_delimiter()?
            .add_optional_child(&self.property)?
            .add_children(&self.elements)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_delimiter() {
        let xml = test_xml(
            &OMathDelimiter::new()
                .begin_char("{")
                .separator_char("|")
                .end_char("}")
                .grow(true)
                .shape(OMathShapeDelimiterType::Match)
                .add_element(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                .add_element(OMathBase::new().add_run(OMathRun::new().add_text("y"))),
        );
        assert_eq!(
            xml,
            r#"<m:d><m:dPr><m:begChr m:val="{" /><m:sepChr m:val="|" /><m:endChr m:val="}" /><m:grow m:val="1" /><m:shp m:val="match" /></m:dPr><m:e><m:r><m:t>x</m:t></m:r></m:e><m:e><m:r><m:t>y</m:t></m:r></m:e></m:d>"#
        );
    }

    case_test!(
        m_d_within_common_omath_parents,
        "m:d" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
