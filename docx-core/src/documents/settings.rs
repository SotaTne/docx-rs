use super::*;
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::{
    CharacterSpacingValues, OMathBreakBinaryOperatorType, OMathBreakBinarySubtractionType,
    OMathJustificationType, OMathLimitLocationType,
};
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    default_tab_stop: DefaultTabStop,
    zoom: Zoom,
    doc_id: Option<DocId>,
    doc_vars: Vec<DocVar>,
    even_and_odd_headers: bool,
    adjust_line_height_in_table: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    character_spacing_control: Option<CharacterSpacingValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    math_properties: Option<MathProperties>,
}

impl Settings {
    pub fn new() -> Settings {
        Default::default()
    }

    pub fn doc_id(mut self, id: impl Into<String>) -> Self {
        self.doc_id = Some(DocId::new(id.into()));
        self
    }

    pub fn default_tab_stop(mut self, tab_stop: usize) -> Self {
        self.default_tab_stop = DefaultTabStop::new(tab_stop);
        self
    }

    pub fn add_doc_var(mut self, name: impl Into<String>, val: impl Into<String>) -> Self {
        self.doc_vars.push(DocVar::new(name, val));
        self
    }

    pub fn even_and_odd_headers(mut self) -> Self {
        self.even_and_odd_headers = true;
        self
    }

    pub fn adjust_line_height_in_table(mut self) -> Self {
        self.adjust_line_height_in_table = true;
        self
    }

    pub fn character_spacing_control(mut self, val: CharacterSpacingValues) -> Self {
        self.character_spacing_control = Some(val);
        self
    }

    pub fn math_properties(mut self, val: MathProperties) -> Self {
        self.math_properties = Some(val);
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_tab_stop: DefaultTabStop::new(840),
            zoom: Zoom::new(100),
            doc_id: None,
            doc_vars: vec![],
            even_and_odd_headers: false,
            adjust_line_height_in_table: false,
            character_spacing_control: None,
            math_properties: None,
        }
    }
}

impl BuildXML for Settings {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .declaration(Some(true))?
            .open_settings()?
            .add_child(&self.default_tab_stop)?
            .add_child(&self.zoom)?
            .open_compat()?
            .space_for_ul()?
            .balance_single_byte_double_byte_width()?
            .do_not_leave_backslash_alone()?
            .ul_trail_space()?
            .do_not_expand_shift_return()?
            .apply_opt(self.character_spacing_control, |v, b| {
                b.character_spacing_control(&v.to_string())
            })?
            .apply_if(self.adjust_line_height_in_table, |b| {
                b.adjust_line_height_table()
            })?
            .use_fe_layout()?
            .compat_setting(
                "compatibilityMode",
                "http://schemas.microsoft.com/office/word",
                "15",
            )?
            .compat_setting(
                "overrideTableStyleFontSizeAndJustification",
                "http://schemas.microsoft.com/office/word",
                "1",
            )?
            .compat_setting(
                "enableOpenTypeFeatures",
                "http://schemas.microsoft.com/office/word",
                "1",
            )?
            .compat_setting(
                "doNotFlipMirrorIndents",
                "http://schemas.microsoft.com/office/word",
                "1",
            )?
            .compat_setting(
                "differentiateMultirowTableHeaders",
                "http://schemas.microsoft.com/office/word",
                "1",
            )?
            .compat_setting(
                "useWord2013TrackBottomHyphenation",
                "http://schemas.microsoft.com/office/word",
                "0",
            )?
            .close()?
            .add_optional_child(&self.doc_id)?
            .apply_if(!self.doc_vars.is_empty(), |b| {
                b.open_doc_vars()?.add_children(&self.doc_vars)?.close()
            })?
            .add_optional_child(&self.math_properties)?
            .apply_if(self.even_and_odd_headers, |b| b.even_and_odd_headers())?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MathProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    math_font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    break_binary: Option<OMathBreakBinaryOperatorType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    break_binary_subtraction: Option<OMathBreakBinarySubtractionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    small_fraction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_defaults: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left_margin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right_margin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_justification: Option<OMathJustificationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_spacing: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_spacing: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inter_spacing: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intra_spacing: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrap_indent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrap_right: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integral_limit_location: Option<OMathLimitLocationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nary_limit_location: Option<OMathLimitLocationType>,
}

impl MathProperties {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn break_binary(mut self, value: OMathBreakBinaryOperatorType) -> Self {
        self.break_binary = Some(value);
        self
    }

    pub fn math_font(mut self, value: impl Into<String>) -> Self {
        self.math_font = Some(value.into());
        self
    }

    pub fn break_binary_subtraction(mut self, value: OMathBreakBinarySubtractionType) -> Self {
        self.break_binary_subtraction = Some(value);
        self
    }

    pub fn small_fraction(mut self, value: bool) -> Self {
        self.small_fraction = Some(value);
        self
    }

    pub fn display_defaults(mut self, value: bool) -> Self {
        self.display_defaults = Some(value);
        self
    }

    pub fn left_margin(mut self, value: u32) -> Self {
        self.left_margin = Some(value);
        self
    }

    pub fn right_margin(mut self, value: u32) -> Self {
        self.right_margin = Some(value);
        self
    }

    pub fn default_justification(mut self, value: OMathJustificationType) -> Self {
        self.default_justification = Some(value);
        self
    }

    pub fn pre_spacing(mut self, value: u32) -> Self {
        self.pre_spacing = Some(value);
        self
    }

    pub fn post_spacing(mut self, value: u32) -> Self {
        self.post_spacing = Some(value);
        self
    }

    pub fn inter_spacing(mut self, value: u32) -> Self {
        self.inter_spacing = Some(value);
        self
    }

    pub fn intra_spacing(mut self, value: u32) -> Self {
        self.intra_spacing = Some(value);
        self
    }

    pub fn wrap_indent(mut self, value: u32) -> Self {
        self.wrap_indent = Some(value);
        self
    }

    pub fn wrap_right(mut self, value: bool) -> Self {
        self.wrap_right = Some(value);
        self
    }

    pub fn integral_limit_location(mut self, value: OMathLimitLocationType) -> Self {
        self.integral_limit_location = Some(value);
        self
    }

    pub fn nary_limit_location(mut self, value: OMathLimitLocationType) -> Self {
        self.nary_limit_location = Some(value);
        self
    }
}

impl BuildXML for MathProperties {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_math_properties()?
            .apply_opt(self.math_font.as_deref(), |value, b| {
                b.omath_math_font(value)
            })?
            .apply_opt(self.break_binary.as_ref(), |value, b| {
                b.omath_break_binary(&value.to_string())
            })?
            .apply_opt(self.break_binary_subtraction.as_ref(), |value, b| {
                b.omath_break_binary_subtraction(&value.to_string())
            })?
            .apply_opt(self.small_fraction, |value, b| {
                b.omath_on_off("m:smallFrac", value)
            })?
            .apply_opt(self.display_defaults, |value, b| {
                if value {
                    b.omath_empty_tag("m:dispDef")
                } else {
                    b.omath_on_off("m:dispDef", value)
                }
            })?
            .apply_opt(self.left_margin, |value, b| {
                b.omath_char_tag("m:lMargin", &value.to_string())
            })?
            .apply_opt(self.right_margin, |value, b| {
                b.omath_char_tag("m:rMargin", &value.to_string())
            })?
            .apply_opt(self.default_justification.as_ref(), |value, b| {
                b.omath_char_tag("m:defJc", &value.to_string())
            })?
            .apply_opt(self.pre_spacing, |value, b| {
                b.omath_char_tag("m:preSp", &value.to_string())
            })?
            .apply_opt(self.post_spacing, |value, b| {
                b.omath_char_tag("m:postSp", &value.to_string())
            })?
            .apply_opt(self.inter_spacing, |value, b| {
                b.omath_char_tag("m:interSp", &value.to_string())
            })?
            .apply_opt(self.intra_spacing, |value, b| {
                b.omath_char_tag("m:intraSp", &value.to_string())
            })?
            .apply_opt(self.wrap_indent, |value, b| {
                b.omath_char_tag("m:wrapIndent", &value.to_string())
            })?
            .apply_opt(self.wrap_right, |value, b| {
                b.omath_on_off("m:wrapRight", value)
            })?
            .apply_opt(self.integral_limit_location.as_ref(), |value, b| {
                b.omath_char_tag("m:intLim", &value.to_string())
            })?
            .apply_opt(self.nary_limit_location.as_ref(), |value, b| {
                b.omath_char_tag("m:naryLim", &value.to_string())
            })?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_settings() {
        let c = Settings::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><w:defaultTabStop w:val="840" /><w:zoom w:percent="100" /><w:compat><w:spaceForUL /><w:balanceSingleByteDoubleByteWidth /><w:doNotLeaveBackslashAlone /><w:ulTrailSpace /><w:doNotExpandShiftReturn /><w:useFELayout /><w:compatSetting w:name="compatibilityMode" w:uri="http://schemas.microsoft.com/office/word" w:val="15" /><w:compatSetting w:name="overrideTableStyleFontSizeAndJustification" w:uri="http://schemas.microsoft.com/office/word" w:val="1" /><w:compatSetting w:name="enableOpenTypeFeatures" w:uri="http://schemas.microsoft.com/office/word" w:val="1" /><w:compatSetting w:name="doNotFlipMirrorIndents" w:uri="http://schemas.microsoft.com/office/word" w:val="1" /><w:compatSetting w:name="differentiateMultirowTableHeaders" w:uri="http://schemas.microsoft.com/office/word" w:val="1" /><w:compatSetting w:name="useWord2013TrackBottomHyphenation" w:uri="http://schemas.microsoft.com/office/word" w:val="0" /></w:compat></w:settings>"#
        );
    }

    #[test]
    fn test_settings_with_math_properties() {
        let c = Settings::new().math_properties(
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
        );
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><w:defaultTabStop w:val="840" /><w:zoom w:percent="100" /><w:compat><w:spaceForUL /><w:balanceSingleByteDoubleByteWidth /><w:doNotLeaveBackslashAlone /><w:ulTrailSpace /><w:doNotExpandShiftReturn /><w:useFELayout /><w:compatSetting w:name="compatibilityMode" w:uri="http://schemas.microsoft.com/office/word" w:val="15" /><w:compatSetting w:name="overrideTableStyleFontSizeAndJustification" w:uri="http://schemas.microsoft.com/office/word" w:val="1" /><w:compatSetting w:name="enableOpenTypeFeatures" w:uri="http://schemas.microsoft.com/office/word" w:val="1" /><w:compatSetting w:name="doNotFlipMirrorIndents" w:uri="http://schemas.microsoft.com/office/word" w:val="1" /><w:compatSetting w:name="differentiateMultirowTableHeaders" w:uri="http://schemas.microsoft.com/office/word" w:val="1" /><w:compatSetting w:name="useWord2013TrackBottomHyphenation" w:uri="http://schemas.microsoft.com/office/word" w:val="0" /></w:compat><m:mathPr><m:mathFont m:val="Cambria Math" /><m:brkBin m:val="repeat" /><m:brkBinSub m:val="-+" /><m:smallFrac m:val="1" /><m:dispDef /><m:lMargin m:val="0" /><m:rMargin m:val="0" /><m:defJc m:val="centerGroup" /><m:preSp m:val="120" /><m:postSp m:val="120" /><m:interSp m:val="60" /><m:intraSp m:val="30" /><m:wrapIndent m:val="1440" /><m:wrapRight m:val="1" /><m:intLim m:val="subSup" /><m:naryLim m:val="undOvr" /></m:mathPr></w:settings>"#
        );
    }
}
