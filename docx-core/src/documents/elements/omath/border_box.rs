use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathBorderBox {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathBorderBoxProperty>,
}

impl OMathBorderBox {
    pub fn new(base: OMathBase) -> Self {
        Self {
            base,
            property: None,
        }
    }

    pub fn hide_top(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_top(value));
        self
    }

    pub fn hide_bottom(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_bottom(value));
        self
    }

    pub fn hide_left(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_left(value));
        self
    }

    pub fn hide_right(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_right(value));
        self
    }

    pub fn strike_horizontal(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().strike_horizontal(value));
        self
    }

    pub fn strike_vertical(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().strike_vertical(value));
        self
    }

    pub fn strike_bottom_left_to_top_right(mut self, value: bool) -> Self {
        self.property = Some(
            self.property
                .unwrap_or_default()
                .strike_bottom_left_to_top_right(value),
        );
        self
    }

    pub fn strike_top_left_to_bottom_right(mut self, value: bool) -> Self {
        self.property = Some(
            self.property
                .unwrap_or_default()
                .strike_top_left_to_bottom_right(value),
        );
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl BuildXML for OMathBorderBox {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let property = self.property.clone().unwrap_or_default();
        XMLBuilder::from(stream)
            .open_omath_border_box()?
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
    fn builds_border_box() {
        let xml = test_xml(
            &OMathBorderBox::new(OMathBase::new().add_run(OMathRun::new().add_text("y")))
                .hide_top(true)
                .hide_right(true)
                .strike_horizontal(true)
                .strike_top_left_to_bottom_right(true),
        );
        assert_eq!(
            xml,
            r#"<m:borderBox><m:borderBoxPr><m:hideTop m:val="1" /><m:hideRight m:val="1" /><m:strikeH m:val="1" /><m:strikeTLBR m:val="1" /></m:borderBoxPr><m:e><m:r><m:t>y</m:t></m:r></m:e></m:borderBox>"#
        );
    }

    case_test!(
        m_border_box_within_common_omath_parents,
        "m:borderBox" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
