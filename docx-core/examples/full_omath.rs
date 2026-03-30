use docx_rs::*;

fn add_tag_block(mut docx: Docx, tag: &str, description: &str, samples: Vec<Paragraph>) -> Docx {
    docx = docx.add_paragraph(
        Paragraph::new().add_run(Run::new().add_text(format!("{tag}: {description}"))),
    );

    for sample in samples {
        docx = docx.add_paragraph(sample);
    }

    docx.add_paragraph(Paragraph::new())
}

fn math_paragraph(omath_para: OMathPara) -> Paragraph {
    Paragraph::new().add_omath_para(omath_para)
}

fn inline_math_paragraph(prefix: &str, omath: OMath, suffix: &str) -> Paragraph {
    Paragraph::new()
        .add_run(Run::new().add_text(prefix))
        .add_omath(omath)
        .add_run(Run::new().add_text(suffix))
}

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/examples/full_omath.docx");
    let file = std::fs::File::create(path).unwrap();

    let omath_para_center = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(OMath::new().add_run(OMathRun::new().add_text("a+b=c")));

    let omath_para_left = OMathPara::new()
        .justification(OMathJustificationType::Left)
        .add_math(OMath::new().add_run(OMathRun::new().add_text("left-aligned")));

    let run_text_basic = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("A⊕B,"))
                .add_run(OMathRun::new().add_text("¬P,"))
                .add_run(OMathRun::new().add_text("x∈A∩B,"))
                .add_run(OMathRun::new().add_text("y∉C∪D")),
        );

    let run_with_break = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("line1"))
                .add_run(OMathRun::new().add_break(BreakType::TextWrapping))
                .add_run(OMathRun::new().add_text("line2")),
        );

    let run_with_style = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(
                    OMathRun::new()
                        .style(OMathStyleType::BoldItalic)
                        .align(true)
                        .add_text("A"),
                )
                .add_run(OMathRun::new().add_text(" "))
                .add_run(
                    OMathRun::new()
                        .script(OMathScriptType::DoubleStruck)
                        .add_text("B"),
                )
                .add_run(OMathRun::new().add_text(" "))
                .add_run(OMathRun::new().normal_text(true).add_text("normal"))
                .add_run(OMathRun::new().add_text(" "))
                .add_run(OMathRun::new().literal(true).add_text("lit")),
        );

    let run_with_property_break = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_run(
                OMathRun::new()
                    .manual_break(OMathManualBreak::new().align_at(1))
                    .add_text("property break"),
            ),
        );

    let inline_root_sample = OMath::new().add_run(OMathRun::new().add_text("inline"));

    let fraction_bar = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_fraction(
                OMathFraction::new(
                    OMathNumerator::new().add_run(OMathRun::new().add_text("x+1")),
                    OMathDenominator::new().add_run(OMathRun::new().add_text("y+1")),
                )
                .fraction_type(OMathFractionType::Bar),
            ),
        );

    let fraction_linear = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_fraction(
                OMathFraction::new(
                    OMathNumerator::new()
                        .argument_size(-1)
                        .add_run(OMathRun::new().add_text("a")),
                    OMathDenominator::new().add_run(OMathRun::new().add_text("b")),
                )
                .fraction_type(OMathFractionType::Linear),
            ),
        );

    let fraction_skewed = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_fraction(
                OMathFraction::new(
                    OMathNumerator::new().add_run(OMathRun::new().add_text("a+b")),
                    OMathDenominator::new().add_run(OMathRun::new().add_text("c+d")),
                )
                .fraction_type(OMathFractionType::Skewed)
                .control_property(RunProperty::new().italic()),
            ),
        );

    let fraction_nobar = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_fraction(
                OMathFraction::new(
                    OMathNumerator::new().add_run(OMathRun::new().add_text("p")),
                    OMathDenominator::new().add_run(OMathRun::new().add_text("q")),
                )
                .fraction_type(OMathFractionType::NoBar),
            ),
        );

    let radical_sqrt = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(OMath::new().add_radical(OMathRadical::new(
            OMathBase::new().add_run(OMathRun::new().add_text("x+1")),
        )));

    let radical_cuberoot = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_radical(
                OMathRadical::new(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                    .degree(OMathDegree::new().add_run(OMathRun::new().add_text("3"))),
            ),
        );

    let subscript_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_subscript(
                OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("a")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("i")),
                )
                .control_property(RunProperty::new().italic()),
            ),
        );

    let superscript_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_superscript(
                OMathSuperscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("x")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("2")),
                )
                .control_property(RunProperty::new().bold()),
            ),
        );

    let subsuperscript_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_sub_superscript(
                OMathSubSuperscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("T")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("m")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                )
                .align_scripts(true),
            ),
        );

    let nary_sigma = OMathPara::new()
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
                                OMathNumerator::new()
                                    .add_superscript(OMathSuperscript::new(
                                        OMathBase::new().add_run(OMathRun::new().add_text("i")),
                                        OMathSuperArgument::new()
                                            .add_run(OMathRun::new().add_text("2")),
                                    ))
                                    .add_run(OMathRun::new().add_text("+1")),
                                OMathDenominator::new().add_run(OMathRun::new().add_text("i+1")),
                            )
                            .fraction_type(OMathFractionType::Bar),
                        ),
                    )
                    .operator_char("∑")
                    .limit_location(OMathLimitLocationType::UnderOver),
                ),
        );

    let nary_integral = OMathPara::new()
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
                        .add_run(OMathRun::new().add_text("dx")),
                )
                .operator_char("∫")
                .limit_location(OMathLimitLocationType::UnderOver),
            ),
        );

    let nary_integral_subsup = OMathPara::new()
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
                        .add_run(OMathRun::new().add_text("dx")),
                )
                .operator_char("∫")
                .limit_location(OMathLimitLocationType::SubscriptSuperscript),
            ),
        );

    let function_sin = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(OMath::new().add_function(OMathFunction::new(
            OMathFunctionName::new().add_run(OMathRun::new().add_text("sin")),
            OMathBase::new().add_run(OMathRun::new().add_text("2x")),
        )));

    let function_log = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_function(OMathFunction::new(
                OMathFunctionName::new()
                    .argument_size(1)
                    .add_run(OMathRun::new().add_text("log")),
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
        );

    let function_with_control_property = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_function(
                OMathFunction::new(
                    OMathFunctionName::new()
                        .add_run(OMathRun::new().add_text("max"))
                        .control_property(RunProperty::new().italic()),
                    OMathBase::new().add_run(OMathRun::new().add_text("(x,y)")),
                )
                .control_property(RunProperty::new().bold()),
            ),
        );

    let limit_lower = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_limit_lower(
                    OMathLimitLower::new(
                        OMathBase::new().add_run(OMathRun::new().add_text("lim")),
                        OMathLimit::new().add_run(OMathRun::new().add_text("n→∞")),
                    )
                    .control_property(RunProperty::new().bold()),
                )
                .add_superscript(OMathSuperscript::new(
                    OMathBase::new().add_delimiter(
                        OMathDelimiter::new()
                            .begin_char("(")
                            .end_char(")")
                            .add_element(
                                OMathBase::new().add_run(OMathRun::new().add_text("1+1/n")),
                            ),
                    ),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                )),
        );

    let limit_upper = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_limit_upper(
                    OMathLimitUpper::new(
                        OMathBase::new().add_run(OMathRun::new().add_text("lim")),
                        OMathLimit::new().add_run(OMathRun::new().add_text("n→∞")),
                    )
                    .control_property(RunProperty::new().italic()),
                )
                .add_subscript(OMathSubscript::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("a")),
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("n")),
                )),
        );

    let group_char_overbrace = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(OMath::new().add_group_char(OMathGroupChar::overbrace(
            OMathBase::new().add_run(OMathRun::new().add_text("a+b+c")),
        )));

    let group_char_underbrace = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_group_char(
                OMathGroupChar::underbrace(
                    OMathBase::new().add_run(OMathRun::new().add_text("x+y+z")),
                )
                .vertical_justification(OMathBarPositionType::Bottom)
                .control_property(RunProperty::new().bold()),
            ),
        );

    let group_char_custom = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_group_char(
                OMathGroupChar::new(
                    OMathBase::new()
                        .argument_size(-1)
                        .control_property(RunProperty::new().italic())
                        .add_run(OMathRun::new().add_text("p+q")),
                )
                .accent_char("⏞")
                .position(OMathBarPositionType::Top)
                .vertical_justification(OMathBarPositionType::Top)
                .control_property(RunProperty::new().italic()),
            ),
        );

    let box_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(OMath::new().add_box(OMathBox::new(
            OMathBase::new().add_run(OMathRun::new().add_text("x+1")),
        )));

    let box_property_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_box(
                OMathBox::new(OMathBase::new().add_run(OMathRun::new().add_text("dx")))
                    .operator_emulator(true)
                    .no_break(true)
                    .differential(true)
                    .manual_break(OMathManualBreak::new().align_at(2))
                    .align(true)
                    .control_property(RunProperty::new().bold()),
            ),
        );

    let border_box_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_border_box(
                OMathBorderBox::new(OMathBase::new().add_run(OMathRun::new().add_text("A")))
                    .hide_top(true)
                    .hide_bottom(true)
                    .hide_left(true)
                    .hide_right(true)
                    .strike_horizontal(true)
                    .strike_vertical(true)
                    .strike_bottom_left_to_top_right(true)
                    .strike_top_left_to_bottom_right(true)
                    .control_property(RunProperty::new().italic()),
            ),
        );

    let phantom_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("x+"))
                .add_phantom(OMathPhantom::new(
                    OMathBase::new().add_run(OMathRun::new().add_text("y")),
                ))
                .add_run(OMathRun::new().add_text("+z")),
        );

    let phantom_visible_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("x+"))
                .add_phantom(
                    OMathPhantom::new(OMathBase::new().add_run(OMathRun::new().add_text("y")))
                        .show(true)
                        .zero_width(true)
                        .zero_ascent(true)
                        .zero_descent(true)
                        .transparent(true)
                        .control_property(RunProperty::new().bold()),
                )
                .add_run(OMathRun::new().add_text("+z")),
        );

    let equation_array_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_equation_array(
                OMathEquationArray::new()
                    .object_distribution(true)
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("x=1")))
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("y=2")))
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("z=3"))),
            ),
        );

    let equation_array_property_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_equation_array(
                OMathEquationArray::new()
                    .base_justification(OMathVerticalAlignmentType::Center)
                    .max_distribution(true)
                    .row_spacing_rule(1)
                    .row_spacing(2)
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("u=1")))
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("v=2"))),
            ),
        );

    let pre_sub_super_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_pre_sub_super(
                OMathPreSubSuper::new(
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("m")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                    OMathBase::new().add_run(OMathRun::new().add_text("T")),
                )
                .control_property(RunProperty::new().bold()),
            ),
        );

    let accent_vector = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(OMath::new().add_accent(OMathAccent::vector(
            OMathBase::new().add_run(OMathRun::new().add_text("AB")),
        )));

    let accent_hat = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_accent(
                OMathAccent::new(OMathBase::new().add_run(OMathRun::new().add_text("v")))
                    .accent_char("\u{0302}")
                    .control_property(RunProperty::new().bold()),
            ),
        );

    let bar_overline = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_bar(OMathBar::overline(
                OMathBase::new().add_delimiter(
                    OMathDelimiter::new()
                        .begin_char("(")
                        .end_char(")")
                        .add_element(OMathBase::new().add_run(OMathRun::new().add_text("x+y"))),
                ),
            )),
        );

    let bar_simple = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_bar(
                OMathBar::new(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                    .position(OMathBarPositionType::Bottom)
                    .control_property(RunProperty::new().italic()),
            ),
        );

    let delimiter_brackets = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_delimiter(
                OMathDelimiter::new()
                    .begin_char("[")
                    .end_char("]")
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("1"))),
            ),
        );

    let delimiter_parentheses = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_delimiter(
                OMathDelimiter::new()
                    .begin_char("(")
                    .end_char(")")
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("a+b"))),
            ),
        );

    let delimiter_extended = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_delimiter(
                OMathDelimiter::new()
                    .begin_char("{")
                    .separator_char("|")
                    .end_char("}")
                    .grow(true)
                    .shape(OMathShapeDelimiterType::Match)
                    .control_property(RunProperty::new().bold())
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("y")))
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("z"))),
            ),
        );

    let matrix_basic = OMathPara::new()
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

    let matrix_column_vector = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_matrix(
                OMathMatrix::new()
                    .add_row(
                        OMathMatrixRow::new()
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("x"))),
                    )
                    .add_row(
                        OMathMatrixRow::new()
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("y"))),
                    )
                    .add_row(
                        OMathMatrixRow::new()
                            .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("z"))),
                    ),
            ),
        );

    let matrix_with_properties = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_matrix(
                OMathMatrix::new()
                    .base_justification(OMathVerticalAlignmentType::Center)
                    .hide_placeholder(true)
                    .row_spacing_rule(1)
                    .column_gap_rule(1)
                    .row_spacing(2)
                    .column_spacing(18)
                    .column_gap(3)
                    .column_justification(OMathHorizontalAlignmentType::Right)
                    .control_property(RunProperty::new().bold())
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
            ),
        );

    let radical_hidden_degree = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_radical(
                OMathRadical::new(OMathBase::new().add_run(OMathRun::new().add_text("x+y")))
                    .hide_degree(true)
                    .control_property(RunProperty::new().italic())
                    .degree(OMathDegree::new().add_run(OMathRun::new().add_text("3"))),
            ),
        );

    let nary_grow_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_nary(
                OMathNary::new(
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("k=1")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("m")),
                    OMathBase::new().add_run(OMathRun::new().add_text("a_k")),
                )
                .operator_char("∏")
                .limit_location(OMathLimitLocationType::SubscriptSuperscript)
                .grow(true)
                .control_property(RunProperty::new().bold()),
            ),
        );

    let nary_hidden_limits_sample = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new().add_nary(
                OMathNary::new(
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("i=1")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                    OMathBase::new().add_run(OMathRun::new().add_text("a_i")),
                )
                .operator_char("∑")
                .hide_sub_argument(true)
                .hide_super_argument(true),
            ),
        );

    let raw_xml_escape = OMathPara::new()
        .justification(OMathJustificationType::Center)
        .add_math(
            OMath::new()
                .add_run(OMathRun::new().add_text("raw="))
                .add_raw_xml(
                    r#"<m:acc><m:accPr><m:chr m:val="̂" /></m:accPr><m:e><m:r><m:t>v</m:t></m:r></m:e></m:acc>"#,
                ),
        );

    let inline_omath = OMath::new()
        .add_superscript(OMathSuperscript::new(
            OMathBase::new().add_run(OMathRun::new().add_text("e")),
            OMathSuperArgument::new().add_run(OMathRun::new().add_text("x")),
        ))
        .add_run(OMathRun::new().add_text("="))
        .add_radical(OMathRadical::new(
            OMathBase::new().add_run(OMathRun::new().add_text("x")),
        ));

    let mut docx = Docx::new().add_paragraph(
        Paragraph::new().add_run(Run::new().add_text(
            "OMATH example organized by supported tag. Each block shows the tag, a short explanation, and one or more concrete patterns.",
        )),
    );

    docx = add_tag_block(
        docx,
        "m:oMathPara + m:jc",
        "display math paragraph and paragraph justification",
        vec![
            math_paragraph(omath_para_center),
            math_paragraph(omath_para_left),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:oMath",
        "inline math root inside a normal paragraph",
        vec![
            inline_math_paragraph("Inline example: ", inline_omath, "."),
            inline_math_paragraph("Simple inline root: ", inline_root_sample, "."),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:r + m:t + w:br",
        "math runs, text nodes, and explicit line breaks",
        vec![
            math_paragraph(run_text_basic),
            math_paragraph(run_with_break),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:rPr + m:sty + m:scr + m:nor + m:lit",
        "math run properties for style, script family, normal text, literal text, and run-level manual break metadata",
        vec![
            math_paragraph(run_with_style),
            math_paragraph(run_with_property_break),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:f",
        "fractions with multiple supported fraction types",
        vec![
            math_paragraph(fraction_bar),
            math_paragraph(fraction_linear),
            math_paragraph(fraction_skewed),
            math_paragraph(fraction_nobar),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:rad",
        "square roots and explicit-degree radicals",
        vec![
            math_paragraph(radical_sqrt),
            math_paragraph(radical_cuberoot),
            math_paragraph(radical_hidden_degree),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:sSub",
        "subscript expressions",
        vec![math_paragraph(subscript_sample)],
    );

    docx = add_tag_block(
        docx,
        "m:sSup",
        "superscript expressions",
        vec![math_paragraph(superscript_sample)],
    );

    docx = add_tag_block(
        docx,
        "m:sSubSup + m:alnScr",
        "combined subscript and superscript expressions with optional script alignment",
        vec![math_paragraph(subsuperscript_sample)],
    );

    docx = add_tag_block(
        docx,
        "m:nary",
        "n-ary operators such as sigma and integrals",
        vec![
            math_paragraph(nary_sigma),
            math_paragraph(nary_integral),
            math_paragraph(nary_integral_subsup),
            math_paragraph(nary_grow_sample),
            math_paragraph(nary_hidden_limits_sample),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:func",
        "function application with typed function name and argument",
        vec![
            math_paragraph(function_sin),
            math_paragraph(function_log),
            math_paragraph(function_with_control_property),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:limLow",
        "operator with a lower limit",
        vec![math_paragraph(limit_lower)],
    );

    docx = add_tag_block(
        docx,
        "m:limUpp",
        "operator with an upper limit",
        vec![math_paragraph(limit_upper)],
    );

    docx = add_tag_block(
        docx,
        "m:acc",
        "accents such as vector arrows and hats",
        vec![math_paragraph(accent_vector), math_paragraph(accent_hat)],
    );

    docx = add_tag_block(
        docx,
        "m:bar",
        "bars and overlines",
        vec![math_paragraph(bar_overline), math_paragraph(bar_simple)],
    );

    docx = add_tag_block(
        docx,
        "m:d",
        "stretch delimiters around one or more arguments",
        vec![
            math_paragraph(delimiter_brackets),
            math_paragraph(delimiter_parentheses),
            math_paragraph(delimiter_extended),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:m",
        "matrices and column vectors",
        vec![
            math_paragraph(matrix_basic),
            math_paragraph(matrix_column_vector),
            math_paragraph(matrix_with_properties),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:groupChr + m:vertJc",
        "grouping characters such as overbraces and underbraces, with optional vertical justification",
        vec![
            math_paragraph(group_char_overbrace),
            math_paragraph(group_char_underbrace),
            math_paragraph(group_char_custom),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:box",
        "boxed math layout container and box properties such as operator-emulator and differential handling",
        vec![math_paragraph(box_sample), math_paragraph(box_property_sample)],
    );

    docx = add_tag_block(
        docx,
        "m:borderBox",
        "border box layout container",
        vec![math_paragraph(border_box_sample)],
    );

    docx = add_tag_block(
        docx,
        "m:phant",
        "phantom layout container; one sample reserves space invisibly and another keeps the base visible",
        vec![
            math_paragraph(phantom_sample),
            math_paragraph(phantom_visible_sample),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:eqArr + m:objDist + m:maxDist",
        "equation arrays with aligned entries and equation-array properties such as object distribution, max distribution, and row spacing",
        vec![
            math_paragraph(equation_array_sample),
            math_paragraph(equation_array_property_sample),
        ],
    );

    docx = add_tag_block(
        docx,
        "m:sPre",
        "pre-sub-superscript expressions",
        vec![math_paragraph(pre_sub_super_sample)],
    );

    docx = add_tag_block(
        docx,
        "m:mathPr + m:mathFont + m:brkBin + m:brkBinSub + m:smallFrac + m:dispDef + m:lMargin + m:rMargin + m:defJc + m:preSp + m:postSp + m:interSp + m:intraSp + m:wrapIndent + m:intLim + m:naryLim",
        "document-level math defaults emitted to settings.xml rather than directly changing the text shown in this block",
        vec![Paragraph::new().add_run(Run::new().add_text(
            "This document writes math defaults such as Cambria Math, binary-break behavior, display defaults, default justification, spacing, wrap indent, wrap right, and integral/n-ary limit locations into word/settings.xml.",
        ))],
    );

    docx = add_tag_block(
        docx,
        "raw OMML escape hatch",
        "fallback for constructs that do not yet have a typed API",
        vec![math_paragraph(raw_xml_escape)],
    );

    docx.settings(
        Settings::new().math_properties(
            MathProperties::new()
                .math_font("Cambria Math")
                .break_binary(OMathBreakBinaryOperatorType::Repeat)
                .break_binary_subtraction(OMathBreakBinarySubtractionType::MinusPlus)
                .small_fraction(true)
                .display_defaults(true)
                .left_margin(0)
                .right_margin(0)
                .default_justification(OMathJustificationType::CenterGroup)
                .pre_spacing(120)
                .post_spacing(120)
                .inter_spacing(60)
                .intra_spacing(30)
                .wrap_indent(1440)
                .wrap_right(true)
                .integral_limit_location(OMathLimitLocationType::SubscriptSuperscript)
                .nary_limit_location(OMathLimitLocationType::UnderOver),
        ),
    )
    .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
        "Note: extend this file by adding a new tag block whenever typed OMATH support grows.",
    )))
    .build()
    .pack(file)?;

    Ok(())
}
