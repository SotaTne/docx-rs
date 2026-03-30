use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathMatrix {
    pub rows: Vec<OMathMatrixRow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathMatrixProperty>,
}

impl OMathMatrix {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            property: None,
        }
    }

    pub fn add_row(mut self, row: OMathMatrixRow) -> Self {
        self.rows.push(row);
        self
    }

    fn column_count(&self) -> usize {
        self.rows.first().map(|row| row.cells.len()).unwrap_or(0)
    }

    pub fn base_justification(mut self, value: OMathVerticalAlignmentType) -> Self {
        self.property = Some(self.property.unwrap_or_default().base_justification(value));
        self
    }

    pub fn hide_placeholder(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_placeholder(value));
        self
    }

    pub fn row_spacing_rule(mut self, value: usize) -> Self {
        self.property = Some(self.property.unwrap_or_default().row_spacing_rule(value));
        self
    }

    pub fn column_gap_rule(mut self, value: usize) -> Self {
        self.property = Some(self.property.unwrap_or_default().column_gap_rule(value));
        self
    }

    pub fn row_spacing(mut self, value: usize) -> Self {
        self.property = Some(self.property.unwrap_or_default().row_spacing(value));
        self
    }

    pub fn column_spacing(mut self, value: usize) -> Self {
        self.property = Some(self.property.unwrap_or_default().column_spacing(value));
        self
    }

    pub fn column_gap(mut self, value: usize) -> Self {
        self.property = Some(self.property.unwrap_or_default().column_gap(value));
        self
    }

    pub fn column_justification(mut self, value: OMathHorizontalAlignmentType) -> Self {
        self.property = Some(
            self.property
                .unwrap_or_default()
                .column_justification(value),
        );
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl Default for OMathMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildXML for OMathMatrix {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let column_count = self.column_count();
        let property = self.property.clone().unwrap_or_default();
        XMLBuilder::from(stream)
            .open_omath_matrix()?
            .open_omath_matrix_property()?
            .apply_opt(property.base_justification.as_ref(), |value, b| {
                b.omath_char_tag("m:baseJc", &value.to_string())
            })?
            .apply_opt(property.hide_placeholder, |value, b| {
                b.omath_on_off("m:plcHide", value)
            })?
            .apply_opt(property.row_spacing_rule, |value, b| {
                b.omath_integer_value("m:rSpRule", value)
            })?
            .apply_opt(property.column_gap_rule, |value, b| {
                b.omath_integer_value("m:cGpRule", value)
            })?
            .apply_opt(property.row_spacing, |value, b| {
                b.omath_integer_value("m:rSp", value)
            })?
            .apply_opt(property.column_spacing, |value, b| {
                b.omath_integer_value("m:cSp", value)
            })?
            .apply_opt(property.column_gap, |value, b| {
                b.omath_integer_value("m:cGp", value)
            })?
            .apply_if(column_count > 0, |b| {
                b.open_omath_matrix_columns()?
                    .open_omath_matrix_column()?
                    .open_omath_matrix_column_property()?
                    .omath_integer_value("m:count", column_count)?
                    .omath_char_tag(
                        "m:mcJc",
                        &property
                            .column_justification
                            .unwrap_or(OMathHorizontalAlignmentType::Center)
                            .to_string(),
                    )?
                    .close()?
                    .close()?
                    .close()
            })?
            .add_optional_child(&property.control_property)?
            .close()?
            .add_children(&self.rows)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathMatrixProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_justification: Option<OMathVerticalAlignmentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_placeholder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_spacing_rule: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_gap_rule: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_spacing: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_spacing: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_gap: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_justification: Option<OMathHorizontalAlignmentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathMatrixProperty {
    pub fn base_justification(mut self, value: OMathVerticalAlignmentType) -> Self {
        self.base_justification = Some(value);
        self
    }
    pub fn hide_placeholder(mut self, value: bool) -> Self {
        self.hide_placeholder = Some(value);
        self
    }
    pub fn row_spacing_rule(mut self, value: usize) -> Self {
        self.row_spacing_rule = Some(value);
        self
    }
    pub fn column_gap_rule(mut self, value: usize) -> Self {
        self.column_gap_rule = Some(value);
        self
    }
    pub fn row_spacing(mut self, value: usize) -> Self {
        self.row_spacing = Some(value);
        self
    }
    pub fn column_spacing(mut self, value: usize) -> Self {
        self.column_spacing = Some(value);
        self
    }
    pub fn column_gap(mut self, value: usize) -> Self {
        self.column_gap = Some(value);
        self
    }
    pub fn column_justification(mut self, value: OMathHorizontalAlignmentType) -> Self {
        self.column_justification = Some(value);
        self
    }
    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_matrix() {
        let xml = test_xml(
            &OMathMatrix::new()
                .base_justification(OMathVerticalAlignmentType::Center)
                .hide_placeholder(true)
                .row_spacing_rule(2)
                .column_gap_rule(1)
                .row_spacing(120)
                .column_spacing(240)
                .column_gap(60)
                .column_justification(OMathHorizontalAlignmentType::Right)
                .add_row(
                    OMathMatrixRow::new()
                        .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("a")))
                        .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("b"))),
                )
                .add_row(
                    OMathMatrixRow::new()
                        .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("c")))
                        .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("d"))),
                ),
        );
        assert_eq!(
            xml,
            r#"<m:m><m:mPr><m:baseJc m:val="center" /><m:plcHide m:val="1" /><m:rSpRule m:val="2" /><m:cGpRule m:val="1" /><m:rSp m:val="120" /><m:cSp m:val="240" /><m:cGp m:val="60" /><m:mcs><m:mc><m:mcPr><m:count m:val="2" /><m:mcJc m:val="right" /></m:mcPr></m:mc></m:mcs></m:mPr><m:mr><m:e><m:r><m:t>a</m:t></m:r></m:e><m:e><m:r><m:t>b</m:t></m:r></m:e></m:mr><m:mr><m:e><m:r><m:t>c</m:t></m:r></m:e><m:e><m:r><m:t>d</m:t></m:r></m:e></m:mr></m:m>"#
        );
    }

    case_test!(
        m_m_within_common_omath_parents,
        "m:m" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
