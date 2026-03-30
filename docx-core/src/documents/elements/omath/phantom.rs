use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathPhantom {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathPhantomProperty>,
}

impl OMathPhantom {
    pub fn new(base: OMathBase) -> Self {
        Self {
            base,
            property: None,
        }
    }

    pub fn show(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().show(value));
        self
    }

    pub fn zero_width(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().zero_width(value));
        self
    }

    pub fn zero_ascent(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().zero_ascent(value));
        self
    }

    pub fn zero_descent(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().zero_descent(value));
        self
    }

    pub fn transparent(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().transparent(value));
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl BuildXML for OMathPhantom {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let property = self.property.clone().unwrap_or_default();
        XMLBuilder::from(stream)
            .open_omath_phantom()?
            .add_child(&property)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_phantom() {
        let xml = test_xml(
            &OMathPhantom::new(OMathBase::new().add_run(OMathRun::new().add_text("z")))
                .show(true)
                .zero_width(true)
                .zero_ascent(true)
                .zero_descent(true)
                .transparent(true),
        );
        assert_eq!(
            xml,
            r#"<m:phant><m:phantPr><m:show m:val="1" /><m:zeroWid m:val="1" /><m:zeroAsc m:val="1" /><m:zeroDesc m:val="1" /><m:transp m:val="1" /></m:phantPr><m:e><m:r><m:t>z</m:t></m:r></m:e></m:phant>"#
        );
    }

    case_test!(
        m_phant_within_common_omath_parents,
        "m:phant" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
