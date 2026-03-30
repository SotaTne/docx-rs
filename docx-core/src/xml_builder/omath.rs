use super::XMLBuilder;
use super::XmlEvent;
use crate::xml::writer::Result;
use std::io::Write;

impl<W: Write> XMLBuilder<W> {
    open!(open_omath, "m:oMath");
    open!(open_omath_para, "m:oMathPara");
    open!(open_omath_para_property, "m:oMathParaPr");
    open!(open_omath_run, "m:r");
    open!(open_omath_run_property, "m:rPr");
    open!(open_omath_control_property, "m:ctrlPr");
    open!(open_omath_accent, "m:acc");
    open!(open_omath_accent_property, "m:accPr");
    open!(open_omath_bar, "m:bar");
    open!(open_omath_bar_property, "m:barPr");
    open!(open_omath_delimiter, "m:d");
    open!(open_omath_delimiter_property, "m:dPr");
    open!(open_omath_fraction, "m:f");
    open!(open_omath_fraction_property, "m:fPr");
    open!(open_omath_function, "m:func");
    open!(open_omath_function_property, "m:funcPr");
    open!(open_omath_function_name, "m:fName");
    open!(open_omath_argument_property, "m:argPr");
    open!(open_omath_group_char, "m:groupChr");
    open!(open_omath_group_char_property, "m:groupChrPr");
    open!(open_omath_limit_lower, "m:limLow");
    open!(open_omath_limit_lower_property, "m:limLowPr");
    open!(open_omath_limit_upper, "m:limUpp");
    open!(open_omath_limit_upper_property, "m:limUppPr");
    open!(open_omath_limit, "m:lim");
    open!(open_omath_box, "m:box");
    open!(open_omath_box_property, "m:boxPr");
    open!(open_omath_border_box, "m:borderBox");
    open!(open_omath_border_box_property, "m:borderBoxPr");
    open!(open_omath_equation_array, "m:eqArr");
    open!(open_omath_equation_array_property, "m:eqArrPr");
    open!(open_omath_math_properties, "m:mathPr");
    open!(open_omath_matrix, "m:m");
    open!(open_omath_matrix_property, "m:mPr");
    open!(open_omath_matrix_columns, "m:mcs");
    open!(open_omath_matrix_column, "m:mc");
    open!(open_omath_matrix_column_property, "m:mcPr");
    open!(open_omath_matrix_row, "m:mr");
    open!(open_omath_numerator, "m:num");
    open!(open_omath_denominator, "m:den");
    open!(open_omath_radical, "m:rad");
    open!(open_omath_radical_property, "m:radPr");
    open!(open_omath_degree, "m:deg");
    open!(open_omath_base, "m:e");
    open!(open_omath_subscript, "m:sSub");
    open!(open_omath_superscript, "m:sSup");
    open!(open_omath_sub_superscript, "m:sSubSup");
    open!(open_omath_nary, "m:nary");
    open!(open_omath_nary_property, "m:naryPr");
    open!(open_omath_phantom, "m:phant");
    open!(open_omath_phantom_property, "m:phantPr");
    open!(open_omath_pre_sub_super, "m:sPre");
    open!(open_omath_pre_sub_super_property, "m:sPrePr");
    open!(open_omath_sub_arg, "m:sub");
    open!(open_omath_sup_arg, "m:sup");

    pub(crate) fn omath_text(self, text: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:t"))?
            .write(text)?
            .close()
    }

    pub(crate) fn omath_justification(self, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:jc").attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_fraction_type(self, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:type").attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_style(self, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:sty").attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_script(self, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:scr").attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_break_binary(self, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:brkBin").attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_break_binary_subtraction(self, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:brkBinSub").attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_align_scripts(self, value: bool) -> Result<Self> {
        self.omath_on_off("m:alnScr", value)
    }

    pub(crate) fn omath_math_font(self, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:mathFont").attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_operator_char(self, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:chr").attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_limit_location(self, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("m:limLoc").attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_char_tag(self, tag: &str, value: &str) -> Result<Self> {
        self.write(XmlEvent::start_element(tag).attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_integer_value(self, tag: &str, value: usize) -> Result<Self> {
        let value = value.to_string();
        self.write(XmlEvent::start_element(tag).attr("m:val", &value))?
            .close()
    }

    pub(crate) fn omath_signed_integer_value(self, tag: &str, value: i32) -> Result<Self> {
        let value = value.to_string();
        self.write(XmlEvent::start_element(tag).attr("m:val", &value))?
            .close()
    }

    pub(crate) fn omath_on_off(self, tag: &str, value: bool) -> Result<Self> {
        let value = if value { "1" } else { "0" };
        self.write(XmlEvent::start_element(tag).attr("m:val", value))?
            .close()
    }

    pub(crate) fn omath_empty_tag(self, tag: &str) -> Result<Self> {
        self.write(XmlEvent::start_element(tag))?.close()
    }

    pub(crate) fn omath_manual_break(self, align_at: Option<u8>) -> Result<Self> {
        let tag = match align_at {
            Some(value) => XmlEvent::start_element("m:brk").attr("m:alnAt", &value.to_string()),
            None => XmlEvent::start_element("m:brk"),
        };
        self.write(tag)?.close()
    }
}
