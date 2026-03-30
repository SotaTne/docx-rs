use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathBorderBoxProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_top: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_bottom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_left: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_right: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_horizontal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_vertical: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_bottom_left_to_top_right: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_top_left_to_bottom_right: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathBorderBoxProperty {
    pub fn hide_top(mut self, value: bool) -> Self {
        self.hide_top = Some(value);
        self
    }

    pub fn hide_bottom(mut self, value: bool) -> Self {
        self.hide_bottom = Some(value);
        self
    }

    pub fn hide_left(mut self, value: bool) -> Self {
        self.hide_left = Some(value);
        self
    }

    pub fn hide_right(mut self, value: bool) -> Self {
        self.hide_right = Some(value);
        self
    }

    pub fn strike_horizontal(mut self, value: bool) -> Self {
        self.strike_horizontal = Some(value);
        self
    }

    pub fn strike_vertical(mut self, value: bool) -> Self {
        self.strike_vertical = Some(value);
        self
    }

    pub fn strike_bottom_left_to_top_right(mut self, value: bool) -> Self {
        self.strike_bottom_left_to_top_right = Some(value);
        self
    }

    pub fn strike_top_left_to_bottom_right(mut self, value: bool) -> Self {
        self.strike_top_left_to_bottom_right = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathBorderBoxProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_border_box_property()?
            .apply_opt(self.hide_top, |value, b| b.omath_on_off("m:hideTop", value))?
            .apply_opt(self.hide_bottom, |value, b| {
                b.omath_on_off("m:hideBot", value)
            })?
            .apply_opt(self.hide_left, |value, b| {
                b.omath_on_off("m:hideLeft", value)
            })?
            .apply_opt(self.hide_right, |value, b| {
                b.omath_on_off("m:hideRight", value)
            })?
            .apply_opt(self.strike_horizontal, |value, b| {
                b.omath_on_off("m:strikeH", value)
            })?
            .apply_opt(self.strike_vertical, |value, b| {
                b.omath_on_off("m:strikeV", value)
            })?
            .apply_opt(self.strike_bottom_left_to_top_right, |value, b| {
                b.omath_on_off("m:strikeBLTR", value)
            })?
            .apply_opt(self.strike_top_left_to_bottom_right, |value, b| {
                b.omath_on_off("m:strikeTLBR", value)
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
    fn builds_border_box_property() {
        let xml = test_xml(
            &OMathBorderBoxProperty::default()
                .hide_top(true)
                .hide_right(true)
                .strike_horizontal(true)
                .control_property(RunProperty::new().bold()),
        );
        assert_eq!(
            xml,
            r#"<m:borderBoxPr><m:hideTop m:val="1" /><m:hideRight m:val="1" /><m:strikeH m:val="1" /><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:borderBoxPr>"#
        );
    }

    case_test!(m_border_box_pr_within_m_border_box, "m:borderBoxPr" within "m:borderBox");
}
