use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./omml.docx");
    let file = std::fs::File::create(path).unwrap();

    let display_equation = r#"
<m:oMathPara>
  <m:oMath>
    <m:r>
      <m:t>A=πr²</m:t>
    </m:r>
  </m:oMath>
</m:oMathPara>
"#;

    let inline_equation = r#"
<m:oMath>
  <m:r>
    <m:t>x+y=z</m:t>
  </m:r>
</m:oMath>
"#;

    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Introduction text")))
        .add_paragraph(Paragraph::new().add_omml(display_equation))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Inline equation: "))
                .add_run(Run::new().add_omml(inline_equation))
                .add_run(Run::new().add_text(" in a paragraph.")),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Conclusion text")))
        .build()
        .pack(file)?;

    Ok(())
}
