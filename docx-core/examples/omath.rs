use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/examples/omath.docx");
    let file = std::fs::File::create(path).unwrap();

    let display_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("x"))
                .add_subscript(OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("a")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("i")),
                ))
                .add_run(OMathRun::new().add_text("+"))
                .add_superscript(OMathSuperscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("b")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("2")),
                ))
                .add_run(OMathRun::new().add_text("+"))
                .add_sub_superscript(OMathSubSuperscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("c")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("m")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                ))
                .add_run(OMathRun::new().add_text("="))
                .add_fraction(
                    OMathFraction::new(
                        OMathNumerator::new()
                            .add_run(OMathRun::new().add_text("1"))
                            .add_run(OMathRun::new().add_text("+"))
                            .add_radical(
                                OMathRadical::new(
                                    OMathBase::new().add_run(OMathRun::new().add_text("x+1")),
                                )
                                .degree(OMathDegree::new().add_run(OMathRun::new().add_text("3"))),
                            ),
                        OMathDenominator::new()
                            .add_run(OMathRun::new().add_text("2"))
                            .add_run(OMathRun::new().add_text("+"))
                            .add_fraction(
                                OMathFraction::new(
                                    OMathNumerator::new().add_run(OMathRun::new().add_text("y")),
                                    OMathDenominator::new().add_run(OMathRun::new().add_text("z")),
                                )
                                .fraction_type(OMathFractionType::Linear),
                            ),
                    )
                    .fraction_type(OMathFractionType::Bar),
                ),
        )
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("Manual line break"))
                .add_run(OMathRun::new().add_break(BreakType::TextWrapping))
                .add_run(OMathRun::new().add_text("after m:r/w:br")),
        );

    let sigma_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("S="))
                .add_nary(
                    OMathNary::new(
                        OMathSubArgument::new().add_run(OMathRun::new().add_text("i=0")),
                        OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                        OMathBase::new().add_fraction(
                            OMathFraction::new(
                                OMathNumerator::new().add_run(OMathRun::new().add_text("i²+1")),
                                OMathDenominator::new().add_run(OMathRun::new().add_text("i+1")),
                            )
                            .fraction_type(OMathFractionType::Bar),
                        ),
                    )
                    .operator_char("∑")
                    .limit_location(OMathLimitLocationType::UnderOver),
                ),
        );

    let integral_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_nary(
                OMathNary::new(
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("0")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("1")),
                    OMathBase::new()
                        .add_fraction(
                            OMathFraction::new(
                                OMathNumerator::new()
                                    .add_superscript(OMathSuperscript::new(
                                        OMathBase::new().add_run(OMathRun::new().add_text("x")),
                                        OMathSuperArgument::new()
                                            .add_run(OMathRun::new().add_text("2")),
                                    ))
                                    .add_run(OMathRun::new().add_text("+1")),
                                OMathDenominator::new()
                                    .add_run(OMathRun::new().add_text("1+"))
                                    .add_radical(
                                        OMathRadical::new(
                                            OMathBase::new().add_run(OMathRun::new().add_text("x")),
                                        )
                                        .degree(
                                            OMathDegree::new()
                                                .add_run(OMathRun::new().add_text("3")),
                                        ),
                                    ),
                            )
                            .fraction_type(OMathFractionType::Bar),
                        )
                        .add_run(OMathRun::new().add_text(" dx")),
                )
                .operator_char("∫")
                .limit_location(OMathLimitLocationType::UnderOver),
            ),
        );

    let raw_xml_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("raw="))
                .add_raw_xml(
                    r#"<m:acc><m:accPr><m:chr m:val="̂" /></m:accPr><m:e><m:r><m:t>v</m:t></m:r></m:e></m:acc>"#,
                ),
        );

    let vector_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("vector="))
                .add_accent(OMathAccent::vector(
                    OMathBase::new().add_run(OMathRun::new().add_text("AB")),
                )),
        );

    let bar_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_bar(OMathBar::overline(
                OMathBase::new().add_delimiter(
                    OMathDelimiter::new()
                        .begin_char("(")
                        .end_char(")")
                        .add_element(OMathBase::new().add_run(OMathRun::new().add_text("x⊕y"))),
                ),
            )),
        );

    let matrix_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_matrix(
                OMathMatrix::new()
                    .add_row(
                        OMathMatrixRow::new()
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("1")))
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("1")))
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("3"))),
                    )
                    .add_row(
                        OMathMatrixRow::new()
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("1")))
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("1")))
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("1"))),
                    )
                    .add_row(
                        OMathMatrixRow::new()
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("1")))
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("1")))
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("1"))),
                    ),
            ),
        );

    let delimiter_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_delimiter(
                OMathDelimiter::new()
                    .begin_char("[")
                    .end_char("]")
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("1"))),
            ),
        );

    let function_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(OMath::new().add_function(OMathFunction::new(
            OMathFunctionName::new().add_run(OMathRun::new().add_text("sin")),
            OMathBase::new().add_run(OMathRun::new().add_text("2")),
        )));

    let logic_equation = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("A⊕B,"))
                .add_run(OMathRun::new().add_text("¬P,"))
                .add_run(OMathRun::new().add_text("x∈A∩B,"))
                .add_run(OMathRun::new().add_text("y∉C∪D")),
        );

    let function_definition = OMathPara::new()
        .justification(OMathJustificationType::Left)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("f(x)="))
                .add_fraction(
                    OMathFraction::new(
                        OMathNumerator::new().add_run(OMathRun::new().add_text("x²-1")),
                        OMathDenominator::new().add_run(OMathRun::new().add_text("x-1")),
                    )
                    .fraction_type(OMathFractionType::Linear),
                )
                .add_run(OMathRun::new().add_text(", "))
                .add_run(OMathRun::new().add_text("g"))
                .add_subscript(OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("n")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("k")),
                ))
                .add_run(OMathRun::new().add_text("(x)="))
                .add_superscript(OMathSuperscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("x+1")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("k")),
                )),
        );

    let inline_equation = OMath::new()
        .add_run(OMathRun::new().add_text("F"))
        .add_superscript(OMathSuperscript::new(
            OMathBase::new().add_run(OMathRun::new().add_text("e")),
            OMathSuperArgument::new().add_run(OMathRun::new().add_text("x")),
        ))
        .add_run(OMathRun::new().add_text("="))
        .add_radical(
            OMathRadical::new(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                .degree(OMathDegree::new().add_run(OMathRun::new().add_text("2"))),
        );

    Docx::new()
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new().add_text("OMATH example covering all currently implemented features."),
            ),
        )
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 1: mixed baseline text, subscript, superscript, sub-superscript, fractions, radicals, and line breaks.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(display_equation))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 2: sigma using typed m:nary.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(sigma_equation))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 3: integral using typed m:nary.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(integral_equation))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 4: function definitions and indexed symbols.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(function_definition))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 5: raw OMML escape hatch for not-yet-typed constructs like accents.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(raw_xml_equation))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 6: typed vector accent using an over-arrow.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(vector_equation))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 7: typed matrix.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(matrix_equation))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 8: typed delimiter.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(delimiter_equation))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 9: typed function application.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(function_equation))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 10: logical and set-theoretic symbols.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(logic_equation))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Equation 11: typed overline bar.",
            )),
        )
        .add_paragraph(Paragraph::new().add_omath_para(bar_equation))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Inline equation: "))
                .add_omath(inline_equation)
                .add_run(Run::new().add_text(" in a paragraph.")),
        )
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(
                "Note: raw XML is available as an escape hatch, but typed OMATH should remain the default path.",
            )),
        )
        .build()
        .pack(file)?;

    Ok(())
}
