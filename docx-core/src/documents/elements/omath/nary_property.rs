use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathNaryProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_location: Option<OMathLimitLocationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_sub_argument: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_super_argument: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathNaryProperty {
    pub fn operator_char(mut self, value: impl Into<String>) -> Self {
        self.operator_char = Some(value.into());
        self
    }

    pub fn limit_location(mut self, value: OMathLimitLocationType) -> Self {
        self.limit_location = Some(value);
        self
    }

    pub fn hide_sub_argument(mut self, value: bool) -> Self {
        self.hide_sub_argument = Some(value);
        self
    }

    pub fn hide_super_argument(mut self, value: bool) -> Self {
        self.hide_super_argument = Some(value);
        self
    }

    pub fn grow(mut self, value: bool) -> Self {
        self.grow = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathNaryProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_nary_property()?
            .apply_opt(self.operator_char.as_deref(), |value, b| {
                b.omath_operator_char(value)
            })?
            .apply_opt(self.limit_location.as_ref(), |value, b| {
                b.omath_limit_location(&value.to_string())
            })?
            .apply_opt(self.hide_sub_argument, |value, b| {
                b.omath_on_off("m:subHide", value)
            })?
            .apply_opt(self.hide_super_argument, |value, b| {
                b.omath_on_off("m:supHide", value)
            })?
            .apply_opt(self.grow, |value, b| b.omath_on_off("m:grow", value))?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_nary_property() {
        let xml = test_xml(
            &OMathNaryProperty::default()
                .operator_char("∏")
                .limit_location(OMathLimitLocationType::SubscriptSuperscript)
                .hide_sub_argument(true)
                .hide_super_argument(true)
                .grow(true),
        );
        assert_eq!(
            xml,
            r#"<m:naryPr><m:chr m:val="∏" /><m:limLoc m:val="subSup" /><m:subHide m:val="1" /><m:supHide m:val="1" /><m:grow m:val="1" /></m:naryPr>"#
        );
    }

    case_test!(m_nary_pr_within_m_nary, "m:naryPr" within "m:nary");
}
