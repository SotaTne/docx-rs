use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/examples/omath.docx");
    let file = std::fs::File::create(path).unwrap();

    let maxwell_1 = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("∇·"))
                .add_run(OMathRun::new().add_text("D"))
                .add_run(OMathRun::new().add_text("="))
                .add_subscript(OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("ρ")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("f")),
                )),
        );

    let maxwell_2 = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("∇·"))
                .add_run(OMathRun::new().add_text("B"))
                .add_run(OMathRun::new().add_text("="))
                .add_run(OMathRun::new().add_text("0")),
        );

    let maxwell_3 = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("∇×"))
                .add_run(OMathRun::new().add_text("E"))
                .add_run(OMathRun::new().add_text("="))
                .add_run(OMathRun::new().add_text("-"))
                .add_fraction(OMathFraction::new(
                    OMathNumerator::new()
                        .add_run(OMathRun::new().add_text("∂"))
                        .add_run(OMathRun::new().add_text("B")),
                    OMathDenominator::new()
                        .add_run(OMathRun::new().add_text("∂"))
                        .add_run(OMathRun::new().add_text("t")),
                )),
        );

    let maxwell_4 = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("∇×"))
                .add_run(OMathRun::new().add_text("H"))
                .add_run(OMathRun::new().add_text("="))
                .add_subscript(OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("j")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("f")),
                ))
                .add_run(OMathRun::new().add_text("+"))
                .add_fraction(OMathFraction::new(
                    OMathNumerator::new()
                        .add_run(OMathRun::new().add_text("∂"))
                        .add_run(OMathRun::new().add_text("D")),
                    OMathDenominator::new()
                        .add_run(OMathRun::new().add_text("∂"))
                        .add_run(OMathRun::new().add_text("t")),
                )),
        );

    let euler = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_superscript(OMathSuperscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("e")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("iπ")),
                ))
                .add_run(OMathRun::new().add_text("+"))
                .add_run(OMathRun::new().add_text("1"))
                .add_run(OMathRun::new().add_text("="))
                .add_run(OMathRun::new().add_text("0")),
        );

    let schrodinger = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("iℏ"))
                .add_fraction(OMathFraction::new(
                    OMathNumerator::new()
                        .add_run(OMathRun::new().add_text("∂"))
                        .add_run(OMathRun::new().add_text("Ψ"))
                        .add_delimiter(
                            OMathDelimiter::new()
                                .begin_char("(")
                                .end_char(")")
                                .add_element(
                                    OMathBase::new().add_run(OMathRun::new().add_text("r,t")),
                                ),
                        ),
                    OMathDenominator::new()
                        .add_run(OMathRun::new().add_text("∂"))
                        .add_run(OMathRun::new().add_text("t")),
                ))
                .add_run(OMathRun::new().add_text("="))
                .add_delimiter(
                    OMathDelimiter::new()
                        .begin_char("[")
                        .end_char("]")
                        .add_element(
                            OMathBase::new()
                                .add_run(OMathRun::new().add_text("-"))
                                .add_fraction(OMathFraction::new(
                                    OMathNumerator::new().add_superscript(OMathSuperscript::new(
                                        OMathBase::new().add_run(OMathRun::new().add_text("ℏ")),
                                        OMathSuperArgument::new()
                                            .add_run(OMathRun::new().add_text("2")),
                                    )),
                                    OMathDenominator::new().add_run(OMathRun::new().add_text("2m")),
                                ))
                                .add_run(OMathRun::new().add_text("∇²+V"))
                                .add_delimiter(
                                    OMathDelimiter::new()
                                        .begin_char("(")
                                        .end_char(")")
                                        .add_element(
                                            OMathBase::new()
                                                .add_run(OMathRun::new().add_text("r,t")),
                                        ),
                                ),
                        ),
                )
                .add_run(OMathRun::new().add_text("Ψ"))
                .add_delimiter(
                    OMathDelimiter::new()
                        .begin_char("(")
                        .end_char(")")
                        .add_element(OMathBase::new().add_run(OMathRun::new().add_text("r,t"))),
                ),
        );

    let relativity = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_subscript(OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("R")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("μν")),
                ))
                .add_run(OMathRun::new().add_text("-"))
                .add_fraction(OMathFraction::new(
                    OMathNumerator::new().add_run(OMathRun::new().add_text("1")),
                    OMathDenominator::new().add_run(OMathRun::new().add_text("2")),
                ))
                .add_subscript(OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("g")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("μν")),
                ))
                .add_run(OMathRun::new().add_text("R+Λ"))
                .add_subscript(OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("g")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("μν")),
                ))
                .add_run(OMathRun::new().add_text("="))
                .add_fraction(OMathFraction::new(
                    OMathNumerator::new().add_run(OMathRun::new().add_text("8πG")),
                    OMathDenominator::new().add_superscript(OMathSuperscript::new(
                        OMathBase::new().add_run(OMathRun::new().add_text("c")),
                        OMathSuperArgument::new().add_run(OMathRun::new().add_text("4")),
                    )),
                ))
                .add_subscript(OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("T")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("μν")),
                )),
        );

    let probability = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("P"))
                .add_delimiter(
                    OMathDelimiter::new()
                        .begin_char("(")
                        .end_char(")")
                        .add_element(OMathBase::new().add_run(OMathRun::new().add_text("x"))),
                )
                .add_run(OMathRun::new().add_text("="))
                .add_fraction(OMathFraction::new(
                    OMathNumerator::new().add_run(OMathRun::new().add_text("1")),
                    OMathDenominator::new().add_radical(
                        OMathRadical::new(
                            OMathBase::new()
                                .add_run(OMathRun::new().add_text("2π"))
                                .add_superscript(OMathSuperscript::new(
                                    OMathBase::new().add_run(OMathRun::new().add_text("σ")),
                                    OMathSuperArgument::new()
                                        .add_run(OMathRun::new().add_text("2")),
                                )),
                        )
                        .hide_degree(true),
                    ),
                ))
                .add_run(OMathRun::new().add_text("exp"))
                .add_delimiter(
                    OMathDelimiter::new()
                        .begin_char("(")
                        .end_char(")")
                        .add_element(
                            OMathBase::new()
                                .add_run(OMathRun::new().add_text("-"))
                                .add_fraction(OMathFraction::new(
                                    OMathNumerator::new().add_superscript(OMathSuperscript::new(
                                        OMathBase::new().add_delimiter(
                                            OMathDelimiter::new()
                                                .begin_char("(")
                                                .end_char(")")
                                                .add_element(
                                                    OMathBase::new()
                                                        .add_run(OMathRun::new().add_text("x-μ")),
                                                ),
                                        ),
                                        OMathSuperArgument::new()
                                            .add_run(OMathRun::new().add_text("2")),
                                    )),
                                    OMathDenominator::new()
                                        .add_run(OMathRun::new().add_text("2"))
                                        .add_superscript(OMathSuperscript::new(
                                            OMathBase::new().add_run(OMathRun::new().add_text("σ")),
                                            OMathSuperArgument::new()
                                                .add_run(OMathRun::new().add_text("2")),
                                        )),
                                )),
                        ),
                ),
        );

    let docx = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "Representative OMATH formulas. Each numbered section is independent and uses its own math tree.",
        )))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().bold().add_text("1. Maxwell's Equations")),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "Four independent display equations with partial derivatives, vector symbols, and curl/divergence notation.",
        )))
        .add_paragraph(Paragraph::new().add_omath_para(maxwell_1))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new().add_omath_para(maxwell_2))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new().add_omath_para(maxwell_3))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new().add_omath_para(maxwell_4))
        .add_paragraph(Paragraph::new())
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().bold().add_text("2. Euler's Identity")),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "A single independent equation combining an exponential, the imaginary unit, pi, and constants.",
        )))
        .add_paragraph(Paragraph::new().add_omath_para(euler))
        .add_paragraph(Paragraph::new())
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().bold().add_text("3. Schrodinger Equation")),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "An independent time-dependent equation with partial derivatives, fractions, brackets, and a wave function.",
        )))
        .add_paragraph(Paragraph::new().add_omath_para(schrodinger))
        .add_paragraph(Paragraph::new())
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().bold().add_text("4. Einstein Field Equation")),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "An independent equation with Greek letters, tensor subscripts, fractions, and powers.",
        )))
        .add_paragraph(Paragraph::new().add_omath_para(relativity))
        .add_paragraph(Paragraph::new())
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().bold().add_text("5. Probability Density Function")),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "An independent normal-distribution density formula built from a fraction, a radical, and an exponent.",
        )))
        .add_paragraph(Paragraph::new().add_omath_para(probability))
        .add_paragraph(Paragraph::new());

    docx.build().pack(file)?;

    Ok(())
}
