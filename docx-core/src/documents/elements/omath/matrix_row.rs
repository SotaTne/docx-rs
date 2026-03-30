use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathMatrixRow {
    pub cells: Vec<OMathBase>,
}

impl OMathMatrixRow {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn add_cell(mut self, cell: OMathBase) -> Self {
        self.cells.push(cell);
        self
    }
}

impl Default for OMathMatrixRow {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildXML for OMathMatrixRow {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_matrix_row()?
            .add_children(&self.cells)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_matrix_row() {
        let xml = test_xml(
            &OMathMatrixRow::new()
                .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("y"))),
        );
        assert_eq!(
            xml,
            r#"<m:mr><m:e><m:r><m:t>x</m:t></m:r></m:e><m:e><m:r><m:t>y</m:t></m:r></m:e></m:mr>"#
        );
    }

    case_test!(m_mr_within_m_m, "m:mr" within "m:m");
}
