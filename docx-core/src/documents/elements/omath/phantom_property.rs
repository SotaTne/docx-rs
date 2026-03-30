use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathPhantomProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_width: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_ascent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_descent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathPhantomProperty {
    pub fn show(mut self, value: bool) -> Self {
        self.show = Some(value);
        self
    }

    pub fn zero_width(mut self, value: bool) -> Self {
        self.zero_width = Some(value);
        self
    }

    pub fn zero_ascent(mut self, value: bool) -> Self {
        self.zero_ascent = Some(value);
        self
    }

    pub fn zero_descent(mut self, value: bool) -> Self {
        self.zero_descent = Some(value);
        self
    }

    pub fn transparent(mut self, value: bool) -> Self {
        self.transparent = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathPhantomProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_phantom_property()?
            .apply_opt(self.show, |value, b| b.omath_on_off("m:show", value))?
            .apply_opt(self.zero_width, |value, b| {
                b.omath_on_off("m:zeroWid", value)
            })?
            .apply_opt(self.zero_ascent, |value, b| {
                b.omath_on_off("m:zeroAsc", value)
            })?
            .apply_opt(self.zero_descent, |value, b| {
                b.omath_on_off("m:zeroDesc", value)
            })?
            .apply_opt(self.transparent, |value, b| {
                b.omath_on_off("m:transp", value)
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
    fn builds_phantom_property() {
        let xml = test_xml(
            &OMathPhantomProperty::default()
                .show(true)
                .zero_width(true)
                .zero_ascent(true)
                .zero_descent(true)
                .transparent(true),
        );
        assert_eq!(
            xml,
            r#"<m:phantPr><m:show m:val="1" /><m:zeroWid m:val="1" /><m:zeroAsc m:val="1" /><m:zeroDesc m:val="1" /><m:transp m:val="1" /></m:phantPr>"#
        );
    }

    case_test!(m_phant_pr_within_m_phant, "m:phantPr" within "m:phant");
}
