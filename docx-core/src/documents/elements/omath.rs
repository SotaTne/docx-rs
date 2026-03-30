use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::escape::escape;
use crate::xml_builder::*;
use crate::{
    Break, BreakType, OMathBarPositionType, OMathFractionType, OMathJustificationType,
    OMathLimitLocationType,
};

macro_rules! omath_container {
    ($name:ident, $open_fn:ident) => {
        #[derive(Debug, Clone, PartialEq, Default, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub children: Vec<OMathChild>,
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn add_run(mut self, run: OMathRun) -> Self {
                self.children.push(OMathChild::Run(Box::new(run)));
                self
            }

            pub fn add_fraction(mut self, fraction: OMathFraction) -> Self {
                self.children.push(OMathChild::Fraction(Box::new(fraction)));
                self
            }

            pub fn add_accent(mut self, accent: OMathAccent) -> Self {
                self.children.push(OMathChild::Accent(Box::new(accent)));
                self
            }

            pub fn add_delimiter(mut self, delimiter: OMathDelimiter) -> Self {
                self.children
                    .push(OMathChild::Delimiter(Box::new(delimiter)));
                self
            }

            pub fn add_bar(mut self, bar: OMathBar) -> Self {
                self.children.push(OMathChild::Bar(Box::new(bar)));
                self
            }

            pub fn add_radical(mut self, radical: OMathRadical) -> Self {
                self.children.push(OMathChild::Radical(Box::new(radical)));
                self
            }

            pub fn add_subscript(mut self, subscript: OMathSubscript) -> Self {
                self.children
                    .push(OMathChild::Subscript(Box::new(subscript)));
                self
            }

            pub fn add_superscript(mut self, superscript: OMathSuperscript) -> Self {
                self.children
                    .push(OMathChild::Superscript(Box::new(superscript)));
                self
            }

            pub fn add_sub_superscript(mut self, sub_superscript: OMathSubSuperscript) -> Self {
                self.children
                    .push(OMathChild::SubSuperscript(Box::new(sub_superscript)));
                self
            }

            pub fn add_nary(mut self, nary: OMathNary) -> Self {
                self.children.push(OMathChild::Nary(Box::new(nary)));
                self
            }

            pub fn add_function(mut self, function: OMathFunction) -> Self {
                self.children.push(OMathChild::Function(Box::new(function)));
                self
            }

            pub fn add_matrix(mut self, matrix: OMathMatrix) -> Self {
                self.children.push(OMathChild::Matrix(Box::new(matrix)));
                self
            }

            pub fn add_raw_xml(mut self, xml: impl Into<String>) -> Self {
                self.children.push(OMathChild::Raw(RawOMath::new(xml)));
                self
            }
        }

        impl BuildXML for $name {
            fn build_to<W: Write>(
                &self,
                stream: crate::xml::writer::EventWriter<W>,
            ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
                XMLBuilder::from(stream)
                    .$open_fn()?
                    .add_children(&self.children)?
                    .close()?
                    .into_inner()
            }
        }
    };
}

omath_container!(OMath, open_omath);
omath_container!(OMathBase, open_omath_base);
omath_container!(OMathNumerator, open_omath_numerator);
omath_container!(OMathDenominator, open_omath_denominator);
omath_container!(OMathDegree, open_omath_degree);
omath_container!(OMathSubArgument, open_omath_sub_arg);
omath_container!(OMathSuperArgument, open_omath_sup_arg);

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathPara {
    pub children: Vec<OMath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathParaProperty>,
}

impl OMathPara {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_math(mut self, math: OMath) -> Self {
        self.children.push(math);
        self
    }

    pub fn justification(mut self, value: OMathJustificationType) -> Self {
        self.property = Some(
            self.property
                .unwrap_or_default()
                .justification(OMathJustification::new(value)),
        );
        self
    }
}

impl BuildXML for OMathPara {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_para()?
            .add_optional_child(&self.property)?
            .add_children(&self.children)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathParaProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<OMathJustification>,
}

impl OMathParaProperty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn justification(mut self, justification: OMathJustification) -> Self {
        self.justification = Some(justification);
        self
    }
}

impl BuildXML for OMathParaProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_para_property()?
            .add_optional_child(&self.justification)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathJustification {
    pub value: OMathJustificationType,
}

impl OMathJustification {
    pub fn new(value: OMathJustificationType) -> Self {
        Self { value }
    }
}

impl BuildXML for OMathJustification {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .omath_justification(&self.value.to_string())?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathRun {
    pub children: Vec<OMathRunChild>,
}

impl OMathRun {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_text(mut self, text: impl Into<String>) -> Self {
        self.children
            .push(OMathRunChild::Text(OMathText::new(text.into())));
        self
    }

    pub fn add_break(mut self, break_type: BreakType) -> Self {
        self.children
            .push(OMathRunChild::Break(Break::new(break_type)));
        self
    }
}

impl BuildXML for OMathRun {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_run()?
            .add_children(&self.children)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum OMathRunChild {
    Text(OMathText),
    Break(Break),
}

impl BuildXML for OMathRunChild {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        match self {
            OMathRunChild::Text(v) => v.build_to(stream),
            OMathRunChild::Break(v) => v.build_to(stream),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathText {
    pub text: String,
}

impl OMathText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: escape(&text.into()),
        }
    }
}

impl BuildXML for OMathText {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .omath_text(&self.text)?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OMathChild {
    Run(Box<OMathRun>),
    Accent(Box<OMathAccent>),
    Bar(Box<OMathBar>),
    Delimiter(Box<OMathDelimiter>),
    Fraction(Box<OMathFraction>),
    Function(Box<OMathFunction>),
    Matrix(Box<OMathMatrix>),
    Radical(Box<OMathRadical>),
    Subscript(Box<OMathSubscript>),
    Superscript(Box<OMathSuperscript>),
    SubSuperscript(Box<OMathSubSuperscript>),
    Nary(Box<OMathNary>),
    Raw(RawOMath),
}

impl Serialize for OMathChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OMathChild::Run(v) => {
                let mut t = serializer.serialize_struct("OMathRun", 2)?;
                t.serialize_field("type", "run")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Accent(v) => {
                let mut t = serializer.serialize_struct("OMathAccent", 2)?;
                t.serialize_field("type", "accent")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Bar(v) => {
                let mut t = serializer.serialize_struct("OMathBar", 2)?;
                t.serialize_field("type", "bar")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Delimiter(v) => {
                let mut t = serializer.serialize_struct("OMathDelimiter", 2)?;
                t.serialize_field("type", "delimiter")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Fraction(v) => {
                let mut t = serializer.serialize_struct("OMathFraction", 2)?;
                t.serialize_field("type", "fraction")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Function(v) => {
                let mut t = serializer.serialize_struct("OMathFunction", 2)?;
                t.serialize_field("type", "function")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Matrix(v) => {
                let mut t = serializer.serialize_struct("OMathMatrix", 2)?;
                t.serialize_field("type", "matrix")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Radical(v) => {
                let mut t = serializer.serialize_struct("OMathRadical", 2)?;
                t.serialize_field("type", "radical")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Subscript(v) => {
                let mut t = serializer.serialize_struct("OMathSubscript", 2)?;
                t.serialize_field("type", "subscript")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Superscript(v) => {
                let mut t = serializer.serialize_struct("OMathSuperscript", 2)?;
                t.serialize_field("type", "superscript")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::SubSuperscript(v) => {
                let mut t = serializer.serialize_struct("OMathSubSuperscript", 2)?;
                t.serialize_field("type", "subSuperscript")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Nary(v) => {
                let mut t = serializer.serialize_struct("OMathNary", 2)?;
                t.serialize_field("type", "nary")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Raw(v) => {
                let mut t = serializer.serialize_struct("RawOMath", 2)?;
                t.serialize_field("type", "raw")?;
                t.serialize_field("data", v)?;
                t.end()
            }
        }
    }
}

impl BuildXML for OMathChild {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        match self {
            OMathChild::Run(v) => v.build_to(stream),
            OMathChild::Accent(v) => v.build_to(stream),
            OMathChild::Bar(v) => v.build_to(stream),
            OMathChild::Delimiter(v) => v.build_to(stream),
            OMathChild::Fraction(v) => v.build_to(stream),
            OMathChild::Function(v) => v.build_to(stream),
            OMathChild::Matrix(v) => v.build_to(stream),
            OMathChild::Radical(v) => v.build_to(stream),
            OMathChild::Subscript(v) => v.build_to(stream),
            OMathChild::Superscript(v) => v.build_to(stream),
            OMathChild::SubSuperscript(v) => v.build_to(stream),
            OMathChild::Nary(v) => v.build_to(stream),
            OMathChild::Raw(v) => v.build_to(stream),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawOMath {
    pub xml: String,
}

impl RawOMath {
    pub fn new(xml: impl Into<String>) -> Self {
        Self { xml: xml.into() }
    }
}

impl BuildXML for RawOMath {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).raw_xml(&self.xml)?.into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathAccent {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathAccentProperty>,
}

impl OMathAccent {
    pub fn new(base: OMathBase) -> Self {
        Self {
            base,
            property: None,
        }
    }

    pub fn accent_char(mut self, value: impl Into<String>) -> Self {
        self.property = Some(self.property.unwrap_or_default().accent_char(value));
        self
    }

    pub fn vector(base: OMathBase) -> Self {
        Self::new(base).accent_char("\u{20d7}")
    }
}

impl BuildXML for OMathAccent {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_accent()?
            .add_optional_child(&self.property)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathAccentProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_char: Option<String>,
}

impl OMathAccentProperty {
    pub fn accent_char(mut self, value: impl Into<String>) -> Self {
        self.accent_char = Some(value.into());
        self
    }
}

impl BuildXML for OMathAccentProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_accent_property()?
            .apply_opt(self.accent_char.as_deref(), |value, b| {
                b.omath_operator_char(value)
            })?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathBar {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathBarProperty>,
}

impl OMathBar {
    pub fn new(base: OMathBase) -> Self {
        Self {
            base,
            property: None,
        }
    }

    pub fn position(mut self, value: OMathBarPositionType) -> Self {
        self.property = Some(self.property.unwrap_or_default().position(value));
        self
    }

    pub fn overline(base: OMathBase) -> Self {
        Self::new(base).position(OMathBarPositionType::Top)
    }
}

impl BuildXML for OMathBar {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_bar()?
            .add_optional_child(&self.property)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathBarProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<OMathBarPositionType>,
}

impl OMathBarProperty {
    pub fn position(mut self, value: OMathBarPositionType) -> Self {
        self.position = Some(value);
        self
    }
}

impl BuildXML for OMathBarProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_bar_property()?
            .apply_opt(self.position.as_ref(), |value, b| {
                b.omath_char_tag("m:pos", &value.to_string())
            })?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathDelimiter {
    pub elements: Vec<OMathBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathDelimiterProperty>,
}

impl OMathDelimiter {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            property: None,
        }
    }

    pub fn add_element(mut self, element: OMathBase) -> Self {
        self.elements.push(element);
        self
    }

    pub fn begin_char(mut self, value: impl Into<String>) -> Self {
        self.property = Some(self.property.unwrap_or_default().begin_char(value));
        self
    }

    pub fn end_char(mut self, value: impl Into<String>) -> Self {
        self.property = Some(self.property.unwrap_or_default().end_char(value));
        self
    }
}

impl BuildXML for OMathDelimiter {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_delimiter()?
            .add_optional_child(&self.property)?
            .add_children(&self.elements)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathDelimiterProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_char: Option<String>,
}

impl OMathDelimiterProperty {
    pub fn begin_char(mut self, value: impl Into<String>) -> Self {
        self.begin_char = Some(value.into());
        self
    }

    pub fn end_char(mut self, value: impl Into<String>) -> Self {
        self.end_char = Some(value.into());
        self
    }
}

impl BuildXML for OMathDelimiterProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_delimiter_property()?
            .apply_opt(self.begin_char.as_deref(), |value, b| {
                b.omath_char_tag("m:begChr", value)
            })?
            .apply_opt(self.end_char.as_deref(), |value, b| {
                b.omath_char_tag("m:endChr", value)
            })?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathFraction {
    pub numerator: OMathNumerator,
    pub denominator: OMathDenominator,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathFractionProperty>,
}

impl OMathFraction {
    pub fn new(numerator: OMathNumerator, denominator: OMathDenominator) -> Self {
        Self {
            numerator,
            denominator,
            property: None,
        }
    }

    pub fn fraction_type(mut self, value: OMathFractionType) -> Self {
        self.property = Some(self.property.unwrap_or_default().fraction_type(value));
        self
    }
}

impl BuildXML for OMathFraction {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_fraction()?
            .add_optional_child(&self.property)?
            .add_child(&self.numerator)?
            .add_child(&self.denominator)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathFractionProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraction_type: Option<OMathFractionType>,
}

impl OMathFractionProperty {
    pub fn fraction_type(mut self, value: OMathFractionType) -> Self {
        self.fraction_type = Some(value);
        self
    }
}

impl BuildXML for OMathFractionProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_fraction_property()?
            .apply_opt(self.fraction_type.as_ref(), |value, b| {
                b.omath_fraction_type(&value.to_string())
            })?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathFunctionName {
    pub children: Vec<OMathChild>,
}

impl OMathFunctionName {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add_run(mut self, run: OMathRun) -> Self {
        self.children.push(OMathChild::Run(Box::new(run)));
        self
    }
}

impl BuildXML for OMathFunctionName {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_function_name()?
            .add_children(&self.children)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathFunction {
    pub name: OMathFunctionName,
    pub base: OMathBase,
}

impl OMathFunction {
    pub fn new(name: OMathFunctionName, base: OMathBase) -> Self {
        Self { name, base }
    }
}

impl BuildXML for OMathFunction {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_function()?
            .open_omath_function_property()?
            .close()?
            .add_child(&self.name)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathMatrix {
    pub rows: Vec<OMathMatrixRow>,
}

impl OMathMatrix {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn add_row(mut self, row: OMathMatrixRow) -> Self {
        self.rows.push(row);
        self
    }

    fn column_count(&self) -> usize {
        self.rows.first().map(|row| row.cells.len()).unwrap_or(0)
    }
}

impl BuildXML for OMathMatrix {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let column_count = self.column_count();
        XMLBuilder::from(stream)
            .open_omath_matrix()?
            .open_omath_matrix_property()?
            .apply_if(column_count > 0, |b| {
                b.open_omath_matrix_columns()?
                    .open_omath_matrix_column()?
                    .open_omath_matrix_column_property()?
                    .omath_integer_value("m:count", column_count)?
                    .omath_char_tag("m:mcJc", "center")?
                    .close()?
                    .close()?
                    .close()
            })?
            .close()?
            .add_children(&self.rows)?
            .close()?
            .into_inner()
    }
}

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

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathRadical {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<OMathDegree>,
}

impl OMathRadical {
    pub fn new(base: OMathBase) -> Self {
        Self { base, degree: None }
    }

    pub fn degree(mut self, degree: OMathDegree) -> Self {
        self.degree = Some(degree);
        self
    }
}

impl BuildXML for OMathRadical {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_radical()?
            .add_optional_child(&self.degree)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathSubscript {
    pub base: OMathBase,
    pub sub_argument: OMathSubArgument,
}

impl OMathSubscript {
    pub fn new(base: OMathBase, sub_argument: OMathSubArgument) -> Self {
        Self { base, sub_argument }
    }
}

impl BuildXML for OMathSubscript {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_subscript()?
            .add_child(&self.base)?
            .add_child(&self.sub_argument)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathSuperscript {
    pub base: OMathBase,
    pub super_argument: OMathSuperArgument,
}

impl OMathSuperscript {
    pub fn new(base: OMathBase, super_argument: OMathSuperArgument) -> Self {
        Self {
            base,
            super_argument,
        }
    }
}

impl BuildXML for OMathSuperscript {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_superscript()?
            .add_child(&self.base)?
            .add_child(&self.super_argument)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathSubSuperscript {
    pub base: OMathBase,
    pub sub_argument: OMathSubArgument,
    pub super_argument: OMathSuperArgument,
}

impl OMathSubSuperscript {
    pub fn new(
        base: OMathBase,
        sub_argument: OMathSubArgument,
        super_argument: OMathSuperArgument,
    ) -> Self {
        Self {
            base,
            sub_argument,
            super_argument,
        }
    }
}

impl BuildXML for OMathSubSuperscript {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_sub_superscript()?
            .add_child(&self.base)?
            .add_child(&self.sub_argument)?
            .add_child(&self.super_argument)?
            .close()?
            .into_inner()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathNary {
    pub sub_argument: OMathSubArgument,
    pub super_argument: OMathSuperArgument,
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathNaryProperty>,
}

impl OMathNary {
    pub fn new(
        sub_argument: OMathSubArgument,
        super_argument: OMathSuperArgument,
        base: OMathBase,
    ) -> Self {
        Self {
            sub_argument,
            super_argument,
            base,
            property: None,
        }
    }

    pub fn operator_char(mut self, value: impl Into<String>) -> Self {
        self.property = Some(self.property.unwrap_or_default().operator_char(value));
        self
    }

    pub fn limit_location(mut self, value: OMathLimitLocationType) -> Self {
        self.property = Some(self.property.unwrap_or_default().limit_location(value));
        self
    }

    pub fn hide_sub_argument(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_sub_argument(value));
        self
    }

    pub fn hide_super_argument(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_super_argument(value));
        self
    }
}

impl BuildXML for OMathNary {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_nary()?
            .add_optional_child(&self.property)?
            .add_child(&self.sub_argument)?
            .add_child(&self.super_argument)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

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
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str;
    use std::str::from_utf8;

    #[test]
    fn test_omath_build() {
        let b = OMath::new().add_run(OMathRun::new().add_text("x")).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<m:oMath><m:r><m:t>x</m:t></m:r></m:oMath>"#
        );
    }

    #[test]
    fn test_omath_para_build() {
        let b = OMathPara::new()
            .justification(OMathJustificationType::Right)
            .add_math(OMath::new().add_run(OMathRun::new().add_text("1+1=2")))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<m:oMathPara><m:oMathParaPr><m:jc m:val="right" /></m:oMathParaPr><m:oMath><m:r><m:t>1+1=2</m:t></m:r></m:oMath></m:oMathPara>"#
        );
    }

    #[test]
    fn test_fraction_build() {
        let b = OMath::new()
            .add_fraction(
                OMathFraction::new(
                    OMathNumerator::new().add_run(OMathRun::new().add_text("1")),
                    OMathDenominator::new().add_run(OMathRun::new().add_text("2")),
                )
                .fraction_type(OMathFractionType::Bar),
            )
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<m:oMath><m:f><m:fPr><m:type m:val="bar" /></m:fPr><m:num><m:r><m:t>1</m:t></m:r></m:num><m:den><m:r><m:t>2</m:t></m:r></m:den></m:f></m:oMath>"#
        );
    }

    #[test]
    fn test_delimiter_build() {
        let b = OMath::new()
            .add_delimiter(
                OMathDelimiter::new()
                    .begin_char("[")
                    .end_char("]")
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("1"))),
            )
            .build();
        assert_eq!(
            from_utf8(&b).unwrap(),
            r#"<m:oMath><m:d><m:dPr><m:begChr m:val="[" /><m:endChr m:val="]" /></m:dPr><m:e><m:r><m:t>1</m:t></m:r></m:e></m:d></m:oMath>"#
        );
    }

    #[test]
    fn test_bar_build() {
        let b = OMath::new()
            .add_bar(OMathBar::overline(
                OMathBase::new().add_run(OMathRun::new().add_text("x⊕y")),
            ))
            .build();
        assert_eq!(
            from_utf8(&b).unwrap(),
            r#"<m:oMath><m:bar><m:barPr><m:pos m:val="top" /></m:barPr><m:e><m:r><m:t>x⊕y</m:t></m:r></m:e></m:bar></m:oMath>"#
        );
    }

    #[test]
    fn test_function_build() {
        let b = OMath::new()
            .add_function(OMathFunction::new(
                OMathFunctionName::new().add_run(OMathRun::new().add_text("sin")),
                OMathBase::new().add_run(OMathRun::new().add_text("2")),
            ))
            .build();
        assert_eq!(
            from_utf8(&b).unwrap(),
            r#"<m:oMath><m:func><m:funcPr /><m:fName><m:r><m:t>sin</m:t></m:r></m:fName><m:e><m:r><m:t>2</m:t></m:r></m:e></m:func></m:oMath>"#
        );
    }

    #[test]
    fn test_matrix_build() {
        let b = OMath::new()
            .add_matrix(
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
                    ),
            )
            .build();
        assert_eq!(
            from_utf8(&b).unwrap(),
            r#"<m:oMath><m:m><m:mPr><m:mcs><m:mc><m:mcPr><m:count m:val="3" /><m:mcJc m:val="center" /></m:mcPr></m:mc></m:mcs></m:mPr><m:mr><m:e><m:r><m:t>1</m:t></m:r></m:e><m:e><m:r><m:t>1</m:t></m:r></m:e><m:e><m:r><m:t>3</m:t></m:r></m:e></m:mr><m:mr><m:e><m:r><m:t>1</m:t></m:r></m:e><m:e><m:r><m:t>1</m:t></m:r></m:e><m:e><m:r><m:t>1</m:t></m:r></m:e></m:mr></m:m></m:oMath>"#
        );
    }

    #[test]
    fn test_accent_build() {
        let b = OMath::new()
            .add_accent(OMathAccent::vector(
                OMathBase::new().add_run(OMathRun::new().add_text("v")),
            ))
            .build();
        assert_eq!(
            from_utf8(&b).unwrap(),
            r#"<m:oMath><m:acc><m:accPr><m:chr m:val="⃗" /></m:accPr><m:e><m:r><m:t>v</m:t></m:r></m:e></m:acc></m:oMath>"#
        );
    }

    #[test]
    fn test_radical_build() {
        let b = OMath::new()
            .add_radical(
                OMathRadical::new(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                    .degree(OMathDegree::new().add_run(OMathRun::new().add_text("3"))),
            )
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<m:oMath><m:rad><m:deg><m:r><m:t>3</m:t></m:r></m:deg><m:e><m:r><m:t>x</m:t></m:r></m:e></m:rad></m:oMath>"#
        );
    }

    #[test]
    fn test_nary_build() {
        let b = OMath::new()
            .add_nary(
                OMathNary::new(
                    OMathSubArgument::new().add_run(OMathRun::new().add_text("i=0")),
                    OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                    OMathBase::new().add_run(OMathRun::new().add_text("i")),
                )
                .operator_char("∑")
                .limit_location(OMathLimitLocationType::UnderOver),
            )
            .build();
        assert_eq!(
            from_utf8(&b).unwrap(),
            r#"<m:oMath><m:nary><m:naryPr><m:chr m:val="∑" /><m:limLoc m:val="undOvr" /></m:naryPr><m:sub><m:r><m:t>i=0</m:t></m:r></m:sub><m:sup><m:r><m:t>n</m:t></m:r></m:sup><m:e><m:r><m:t>i</m:t></m:r></m:e></m:nary></m:oMath>"#
        );
    }

    #[test]
    fn test_raw_omath_build() {
        let b = OMath::new()
            .add_run(OMathRun::new().add_text("x="))
            .add_raw_xml(r#"<m:acc><m:e><m:r><m:t>y</m:t></m:r></m:e></m:acc>"#)
            .build();
        assert_eq!(
            from_utf8(&b).unwrap(),
            r#"<m:oMath><m:r><m:t>x=</m:t></m:r><m:acc><m:e><m:r><m:t>y</m:t></m:r></m:e></m:acc></m:oMath>"#
        );
    }
}
