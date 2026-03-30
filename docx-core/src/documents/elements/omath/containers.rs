use super::*;

omath_container!(OMath, open_omath);
omath_argument_container!(OMathBase, open_omath_base);
omath_argument_container!(OMathNumerator, open_omath_numerator);
omath_argument_container!(OMathDenominator, open_omath_denominator);
omath_argument_container!(OMathDegree, open_omath_degree);
omath_argument_container!(OMathSubArgument, open_omath_sub_arg);
omath_argument_container!(OMathSuperArgument, open_omath_sup_arg);
omath_argument_container!(OMathLimit, open_omath_limit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_container_types() {
        assert_eq!(
            test_xml(&OMath::new().add_run(OMathRun::new().add_text("x"))),
            r#"<m:oMath><m:r><m:t>x</m:t></m:r></m:oMath>"#
        );
        assert_eq!(
            test_xml(&OMathBase::new().add_run(OMathRun::new().add_text("x"))),
            r#"<m:e><m:r><m:t>x</m:t></m:r></m:e>"#
        );
        assert_eq!(
            test_xml(
                &OMathNumerator::new()
                    .argument_size(-1)
                    .control_property(RunProperty::new().bold())
                    .add_run(OMathRun::new().add_text("1"))
            ),
            r#"<m:num><m:argPr><m:argSz m:val="-1" /></m:argPr><m:r><m:t>1</m:t></m:r><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:num>"#
        );
        assert_eq!(
            test_xml(
                &OMathDenominator::new()
                    .control_property(RunProperty::new().italic())
                    .add_run(OMathRun::new().add_text("2"))
            ),
            r#"<m:den><m:r><m:t>2</m:t></m:r><m:ctrlPr><w:rPr><w:i /><w:iCs /></w:rPr></m:ctrlPr></m:den>"#
        );
        assert_eq!(
            test_xml(&OMathDegree::new().add_run(OMathRun::new().add_text("3"))),
            r#"<m:deg><m:r><m:t>3</m:t></m:r></m:deg>"#
        );
        assert_eq!(
            test_xml(&OMathSubArgument::new().add_run(OMathRun::new().add_text("i"))),
            r#"<m:sub><m:r><m:t>i</m:t></m:r></m:sub>"#
        );
        assert_eq!(
            test_xml(&OMathSuperArgument::new().add_run(OMathRun::new().add_text("n"))),
            r#"<m:sup><m:r><m:t>n</m:t></m:r></m:sup>"#
        );
        assert_eq!(
            test_xml(&OMathLimit::new().add_run(OMathRun::new().add_text("x→0"))),
            r#"<m:lim><m:r><m:t>x→0</m:t></m:r></m:lim>"#
        );
    }

    case_test!(m_ctrl_pr_within_m_num_and_m_den, "m:ctrlPr" within "m:num" | "m:den");
    case_test!(m_ctrl_pr_not_within_m_o_math, not "m:ctrlPr" within "m:oMath");
}
