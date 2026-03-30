pub(crate) use serde::ser::{SerializeStruct, Serializer};
pub(crate) use serde::Serialize;
pub(crate) use std::io::Write;

pub(crate) use crate::documents::BuildXML;
pub(crate) use crate::escape::escape;
pub(crate) use crate::xml_builder::*;
pub(crate) use crate::{
    Break, BreakType, OMathBarPositionType, OMathFractionType, OMathHorizontalAlignmentType,
    OMathJustificationType, OMathLimitLocationType, OMathScriptType, OMathShapeDelimiterType,
    OMathStyleType, OMathVerticalAlignmentType, RunProperty,
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

            pub fn add_group_char(mut self, group_char: OMathGroupChar) -> Self {
                self.children
                    .push(OMathChild::GroupChar(Box::new(group_char)));
                self
            }

            pub fn add_limit_lower(mut self, limit_lower: OMathLimitLower) -> Self {
                self.children
                    .push(OMathChild::LimitLower(Box::new(limit_lower)));
                self
            }

            pub fn add_limit_upper(mut self, limit_upper: OMathLimitUpper) -> Self {
                self.children
                    .push(OMathChild::LimitUpper(Box::new(limit_upper)));
                self
            }

            pub fn add_matrix(mut self, matrix: OMathMatrix) -> Self {
                self.children.push(OMathChild::Matrix(Box::new(matrix)));
                self
            }

            pub fn add_box(mut self, math_box: OMathBox) -> Self {
                self.children.push(OMathChild::Box(Box::new(math_box)));
                self
            }

            pub fn add_border_box(mut self, border_box: OMathBorderBox) -> Self {
                self.children
                    .push(OMathChild::BorderBox(Box::new(border_box)));
                self
            }

            pub fn add_equation_array(mut self, equation_array: OMathEquationArray) -> Self {
                self.children
                    .push(OMathChild::EquationArray(Box::new(equation_array)));
                self
            }

            pub fn add_raw_xml(mut self, xml: impl Into<String>) -> Self {
                self.children.push(OMathChild::Raw(RawOMath::new(xml)));
                self
            }

            pub fn add_phantom(mut self, phantom: OMathPhantom) -> Self {
                self.children.push(OMathChild::Phantom(Box::new(phantom)));
                self
            }

            pub fn add_pre_sub_super(mut self, pre_sub_super: OMathPreSubSuper) -> Self {
                self.children
                    .push(OMathChild::PreSubSuper(Box::new(pre_sub_super)));
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
macro_rules! omath_argument_container {
    ($name:ident, $open_fn:ident) => {
        #[derive(Debug, Clone, PartialEq, Default, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub children: Vec<OMathChild>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub property: Option<OMathArgumentProperty>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub control_property: Option<OMathControlProperty>,
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn argument_size(mut self, value: i8) -> Self {
                self.property = Some(self.property.unwrap_or_default().argument_size(value));
                self
            }

            pub fn control_property(mut self, value: RunProperty) -> Self {
                self.control_property = Some(OMathControlProperty::new(value));
                self
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

            pub fn add_group_char(mut self, group_char: OMathGroupChar) -> Self {
                self.children
                    .push(OMathChild::GroupChar(Box::new(group_char)));
                self
            }

            pub fn add_limit_lower(mut self, limit_lower: OMathLimitLower) -> Self {
                self.children
                    .push(OMathChild::LimitLower(Box::new(limit_lower)));
                self
            }

            pub fn add_limit_upper(mut self, limit_upper: OMathLimitUpper) -> Self {
                self.children
                    .push(OMathChild::LimitUpper(Box::new(limit_upper)));
                self
            }

            pub fn add_matrix(mut self, matrix: OMathMatrix) -> Self {
                self.children.push(OMathChild::Matrix(Box::new(matrix)));
                self
            }

            pub fn add_box(mut self, math_box: OMathBox) -> Self {
                self.children.push(OMathChild::Box(Box::new(math_box)));
                self
            }

            pub fn add_border_box(mut self, border_box: OMathBorderBox) -> Self {
                self.children
                    .push(OMathChild::BorderBox(Box::new(border_box)));
                self
            }

            pub fn add_equation_array(mut self, equation_array: OMathEquationArray) -> Self {
                self.children
                    .push(OMathChild::EquationArray(Box::new(equation_array)));
                self
            }

            pub fn add_raw_xml(mut self, xml: impl Into<String>) -> Self {
                self.children.push(OMathChild::Raw(RawOMath::new(xml)));
                self
            }

            pub fn add_phantom(mut self, phantom: OMathPhantom) -> Self {
                self.children.push(OMathChild::Phantom(Box::new(phantom)));
                self
            }

            pub fn add_pre_sub_super(mut self, pre_sub_super: OMathPreSubSuper) -> Self {
                self.children
                    .push(OMathChild::PreSubSuper(Box::new(pre_sub_super)));
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
                    .add_optional_child(&self.property)?
                    .add_children(&self.children)?
                    .add_optional_child(&self.control_property)?
                    .close()?
                    .into_inner()
            }
        }
    };
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
#[derive(Debug, Clone, PartialEq)]
pub enum OMathChild {
    Run(Box<OMathRun>),
    Accent(Box<OMathAccent>),
    Bar(Box<OMathBar>),
    BorderBox(Box<OMathBorderBox>),
    Box(Box<OMathBox>),
    Delimiter(Box<OMathDelimiter>),
    EquationArray(Box<OMathEquationArray>),
    Fraction(Box<OMathFraction>),
    Function(Box<OMathFunction>),
    GroupChar(Box<OMathGroupChar>),
    LimitLower(Box<OMathLimitLower>),
    LimitUpper(Box<OMathLimitUpper>),
    Matrix(Box<OMathMatrix>),
    Radical(Box<OMathRadical>),
    Phantom(Box<OMathPhantom>),
    PreSubSuper(Box<OMathPreSubSuper>),
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
            OMathChild::BorderBox(v) => {
                let mut t = serializer.serialize_struct("OMathBorderBox", 2)?;
                t.serialize_field("type", "borderBox")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Box(v) => {
                let mut t = serializer.serialize_struct("OMathBox", 2)?;
                t.serialize_field("type", "box")?;
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
            OMathChild::EquationArray(v) => {
                let mut t = serializer.serialize_struct("OMathEquationArray", 2)?;
                t.serialize_field("type", "equationArray")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::Function(v) => {
                let mut t = serializer.serialize_struct("OMathFunction", 2)?;
                t.serialize_field("type", "function")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::GroupChar(v) => {
                let mut t = serializer.serialize_struct("OMathGroupChar", 2)?;
                t.serialize_field("type", "groupChar")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::LimitLower(v) => {
                let mut t = serializer.serialize_struct("OMathLimitLower", 2)?;
                t.serialize_field("type", "limitLower")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::LimitUpper(v) => {
                let mut t = serializer.serialize_struct("OMathLimitUpper", 2)?;
                t.serialize_field("type", "limitUpper")?;
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
            OMathChild::Phantom(v) => {
                let mut t = serializer.serialize_struct("OMathPhantom", 2)?;
                t.serialize_field("type", "phantom")?;
                t.serialize_field("data", v)?;
                t.end()
            }
            OMathChild::PreSubSuper(v) => {
                let mut t = serializer.serialize_struct("OMathPreSubSuper", 2)?;
                t.serialize_field("type", "preSubSuper")?;
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
            OMathChild::BorderBox(v) => v.build_to(stream),
            OMathChild::Box(v) => v.build_to(stream),
            OMathChild::Delimiter(v) => v.build_to(stream),
            OMathChild::EquationArray(v) => v.build_to(stream),
            OMathChild::Fraction(v) => v.build_to(stream),
            OMathChild::Function(v) => v.build_to(stream),
            OMathChild::GroupChar(v) => v.build_to(stream),
            OMathChild::LimitLower(v) => v.build_to(stream),
            OMathChild::LimitUpper(v) => v.build_to(stream),
            OMathChild::Matrix(v) => v.build_to(stream),
            OMathChild::Radical(v) => v.build_to(stream),
            OMathChild::Phantom(v) => v.build_to(stream),
            OMathChild::PreSubSuper(v) => v.build_to(stream),
            OMathChild::Subscript(v) => v.build_to(stream),
            OMathChild::Superscript(v) => v.build_to(stream),
            OMathChild::SubSuperscript(v) => v.build_to(stream),
            OMathChild::Nary(v) => v.build_to(stream),
            OMathChild::Raw(v) => v.build_to(stream),
        }
    }
}

#[cfg(test)]
pub(crate) fn test_xml(node: &impl BuildXML) -> String {
    String::from_utf8(node.build()).expect("xml should be utf-8")
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    macro_rules! attach_main_child_to_container {
        ($variant:ident, $parent:expr, $child:expr) => {{
            match $child {
                MinimalNode::Accent(value) => Ok(MinimalNode::$variant($parent.add_accent(value))),
                MinimalNode::Bar(value) => Ok(MinimalNode::$variant($parent.add_bar(value))),
                MinimalNode::BorderBox(value) => {
                    Ok(MinimalNode::$variant($parent.add_border_box(value)))
                }
                MinimalNode::Box(value) => Ok(MinimalNode::$variant($parent.add_box(value))),
                MinimalNode::Delimiter(value) => {
                    Ok(MinimalNode::$variant($parent.add_delimiter(value)))
                }
                MinimalNode::EquationArray(value) => {
                    Ok(MinimalNode::$variant($parent.add_equation_array(value)))
                }
                MinimalNode::Fraction(value) => {
                    Ok(MinimalNode::$variant($parent.add_fraction(value)))
                }
                MinimalNode::Function(value) => {
                    Ok(MinimalNode::$variant($parent.add_function(value)))
                }
                MinimalNode::GroupChar(value) => {
                    Ok(MinimalNode::$variant($parent.add_group_char(value)))
                }
                MinimalNode::LimitLower(value) => {
                    Ok(MinimalNode::$variant($parent.add_limit_lower(value)))
                }
                MinimalNode::LimitUpper(value) => {
                    Ok(MinimalNode::$variant($parent.add_limit_upper(value)))
                }
                MinimalNode::Matrix(value) => Ok(MinimalNode::$variant($parent.add_matrix(value))),
                MinimalNode::Nary(value) => Ok(MinimalNode::$variant($parent.add_nary(value))),
                MinimalNode::Phantom(value) => {
                    Ok(MinimalNode::$variant($parent.add_phantom(value)))
                }
                MinimalNode::PreSubSuper(value) => {
                    Ok(MinimalNode::$variant($parent.add_pre_sub_super(value)))
                }
                MinimalNode::Radical(value) => {
                    Ok(MinimalNode::$variant($parent.add_radical(value)))
                }
                MinimalNode::Run(value) => Ok(MinimalNode::$variant($parent.add_run(value))),
                MinimalNode::Subscript(value) => {
                    Ok(MinimalNode::$variant($parent.add_subscript(value)))
                }
                MinimalNode::SubSuperscript(value) => {
                    Ok(MinimalNode::$variant($parent.add_sub_superscript(value)))
                }
                MinimalNode::Superscript(value) => {
                    Ok(MinimalNode::$variant($parent.add_superscript(value)))
                }
                child => Err(format!(
                    "unsupported child attachment: {} within {}",
                    child.tag(),
                    stringify!($variant)
                )),
            }
        }};
    }

    #[derive(Debug)]
    pub(crate) enum MinimalNode {
        OMath(OMath),
        Base(OMathBase),
        Degree(OMathDegree),
        SubArgument(OMathSubArgument),
        SuperArgument(OMathSuperArgument),
        Limit(OMathLimit),
        Accent(OMathAccent),
        AccentProperty(OMathAccentProperty),
        ArgumentProperty(OMathArgumentProperty),
        Bar(OMathBar),
        BarProperty(OMathBarProperty),
        BorderBox(OMathBorderBox),
        BorderBoxProperty(OMathBorderBoxProperty),
        Box(OMathBox),
        ControlProperty(OMathControlProperty),
        Delimiter(OMathDelimiter),
        DelimiterProperty(OMathDelimiterProperty),
        EquationArray(OMathEquationArray),
        EquationArrayProperty(OMathEquationArrayProperty),
        Fraction(OMathFraction),
        FractionProperty(OMathFractionProperty),
        Function(OMathFunction),
        FunctionName(OMathFunctionName),
        FunctionProperty(OMathFunctionProperty),
        GroupChar(OMathGroupChar),
        GroupCharProperty(OMathGroupCharProperty),
        Justification(OMathJustification),
        LimitLower(OMathLimitLower),
        LimitUpper(OMathLimitUpper),
        LimitUpperProperty(OMathLimitUpperProperty),
        ManualBreak(OMathManualBreak),
        Matrix(OMathMatrix),
        MatrixProperty(OMathMatrixProperty),
        MatrixRow(OMathMatrixRow),
        Nary(OMathNary),
        NaryProperty(OMathNaryProperty),
        Numerator(OMathNumerator),
        Denominator(OMathDenominator),
        Para(OMathPara),
        ParaProperty(OMathParaProperty),
        Phantom(OMathPhantom),
        PhantomProperty(OMathPhantomProperty),
        PreSubSuper(OMathPreSubSuper),
        PreSubSuperProperty(OMathPreSubSuperProperty),
        Radical(OMathRadical),
        RadicalProperty(OMathRadicalProperty),
        Run(OMathRun),
        RunProperty(OMathRunProperty),
        BoxProperty(OMathBoxProperty),
        SubscriptProperty(OMathSubscriptProperty),
        Subscript(OMathSubscript),
        SubSuperscript(OMathSubSuperscript),
        SubSuperscriptProperty(OMathSubSuperscriptProperty),
        LimitLowerProperty(OMathLimitLowerProperty),
        Superscript(OMathSuperscript),
        SuperscriptProperty(OMathSuperscriptProperty),
        Text(OMathText),
    }

    impl MinimalNode {
        fn tag(&self) -> &'static str {
            match self {
                Self::OMath(_) => "m:oMath",
                Self::Base(_) => "m:e",
                Self::Degree(_) => "m:deg",
                Self::SubArgument(_) => "m:sub",
                Self::SuperArgument(_) => "m:sup",
                Self::Limit(_) => "m:lim",
                Self::Accent(_) => "m:acc",
                Self::AccentProperty(_) => "m:accPr",
                Self::ArgumentProperty(_) => "m:argPr",
                Self::Bar(_) => "m:bar",
                Self::BarProperty(_) => "m:barPr",
                Self::BorderBox(_) => "m:borderBox",
                Self::BorderBoxProperty(_) => "m:borderBoxPr",
                Self::Box(_) => "m:box",
                Self::ControlProperty(_) => "m:ctrlPr",
                Self::Delimiter(_) => "m:d",
                Self::DelimiterProperty(_) => "m:dPr",
                Self::EquationArray(_) => "m:eqArr",
                Self::EquationArrayProperty(_) => "m:eqArrPr",
                Self::Fraction(_) => "m:f",
                Self::FractionProperty(_) => "m:fPr",
                Self::Function(_) => "m:func",
                Self::FunctionName(_) => "m:fName",
                Self::FunctionProperty(_) => "m:funcPr",
                Self::GroupChar(_) => "m:groupChr",
                Self::GroupCharProperty(_) => "m:groupChrPr",
                Self::Justification(_) => "m:jc",
                Self::LimitLower(_) => "m:limLow",
                Self::LimitUpper(_) => "m:limUpp",
                Self::LimitUpperProperty(_) => "m:limUppPr",
                Self::ManualBreak(_) => "m:brk",
                Self::Matrix(_) => "m:m",
                Self::MatrixProperty(_) => "m:mPr",
                Self::MatrixRow(_) => "m:mr",
                Self::Nary(_) => "m:nary",
                Self::NaryProperty(_) => "m:naryPr",
                Self::Numerator(_) => "m:num",
                Self::Denominator(_) => "m:den",
                Self::Para(_) => "m:oMathPara",
                Self::ParaProperty(_) => "m:oMathParaPr",
                Self::Phantom(_) => "m:phant",
                Self::PhantomProperty(_) => "m:phantPr",
                Self::PreSubSuper(_) => "m:sPre",
                Self::PreSubSuperProperty(_) => "m:sPrePr",
                Self::Radical(_) => "m:rad",
                Self::RadicalProperty(_) => "m:radPr",
                Self::Run(_) => "m:r",
                Self::RunProperty(_) => "m:rPr",
                Self::BoxProperty(_) => "m:boxPr",
                Self::SubscriptProperty(_) => "m:sSubPr",
                Self::Subscript(_) => "m:sSub",
                Self::SubSuperscript(_) => "m:sSubSup",
                Self::SubSuperscriptProperty(_) => "m:sSubSupPr",
                Self::LimitLowerProperty(_) => "m:limLowPr",
                Self::Superscript(_) => "m:sSup",
                Self::SuperscriptProperty(_) => "m:sSupPr",
                Self::Text(_) => "m:t",
            }
        }

        pub(crate) fn add_child(self, child: MinimalNode) -> Result<Self, String> {
            match (self, child) {
                (Self::OMath(parent), child) => {
                    attach_main_child_to_container!(OMath, parent, child)
                }
                (Self::Base(mut parent), Self::ArgumentProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Base(parent))
                }
                (Self::Base(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(child);
                    Ok(Self::Base(parent))
                }
                (Self::Base(parent), child) => {
                    attach_main_child_to_container!(Base, parent, child)
                }
                (Self::Numerator(mut parent), Self::ArgumentProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Numerator(parent))
                }
                (Self::Numerator(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(child);
                    Ok(Self::Numerator(parent))
                }
                (Self::Numerator(parent), child) => {
                    attach_main_child_to_container!(Numerator, parent, child)
                }
                (Self::Denominator(mut parent), Self::ArgumentProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Denominator(parent))
                }
                (Self::Denominator(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(child);
                    Ok(Self::Denominator(parent))
                }
                (Self::Denominator(parent), child) => {
                    attach_main_child_to_container!(Denominator, parent, child)
                }
                (Self::Degree(mut parent), Self::ArgumentProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Degree(parent))
                }
                (Self::Degree(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(child);
                    Ok(Self::Degree(parent))
                }
                (Self::Degree(parent), child) => {
                    attach_main_child_to_container!(Degree, parent, child)
                }
                (Self::SubArgument(mut parent), Self::ArgumentProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::SubArgument(parent))
                }
                (Self::SubArgument(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(child);
                    Ok(Self::SubArgument(parent))
                }
                (Self::SubArgument(parent), child) => {
                    attach_main_child_to_container!(SubArgument, parent, child)
                }
                (Self::SuperArgument(mut parent), Self::ArgumentProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::SuperArgument(parent))
                }
                (Self::SuperArgument(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(child);
                    Ok(Self::SuperArgument(parent))
                }
                (Self::SuperArgument(parent), child) => {
                    attach_main_child_to_container!(SuperArgument, parent, child)
                }
                (Self::Limit(mut parent), Self::ArgumentProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Limit(parent))
                }
                (Self::Limit(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(child);
                    Ok(Self::Limit(parent))
                }
                (Self::Limit(parent), child) => {
                    attach_main_child_to_container!(Limit, parent, child)
                }
                (Self::Accent(mut parent), Self::AccentProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Accent(parent))
                }
                (Self::Bar(mut parent), Self::BarProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Bar(parent))
                }
                (Self::BorderBox(mut parent), Self::BorderBoxProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::BorderBox(parent))
                }
                (Self::Box(mut parent), Self::BoxProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Box(parent))
                }
                (Self::Delimiter(mut parent), Self::DelimiterProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Delimiter(parent))
                }
                (Self::EquationArray(mut parent), Self::EquationArrayProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::EquationArray(parent))
                }
                (Self::Fraction(mut parent), Self::FractionProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Fraction(parent))
                }
                (Self::Fraction(mut parent), Self::Numerator(child)) => {
                    parent.numerator = child;
                    Ok(Self::Fraction(parent))
                }
                (Self::Fraction(mut parent), Self::Denominator(child)) => {
                    parent.denominator = child;
                    Ok(Self::Fraction(parent))
                }
                (Self::Function(mut parent), Self::FunctionProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Function(parent))
                }
                (Self::Function(mut parent), Self::FunctionName(child)) => {
                    parent.name = child;
                    Ok(Self::Function(parent))
                }
                (Self::Function(mut parent), Self::Base(child)) => {
                    parent.base = child;
                    Ok(Self::Function(parent))
                }
                (Self::FunctionName(mut parent), Self::ArgumentProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::FunctionName(parent))
                }
                (Self::FunctionName(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(child);
                    Ok(Self::FunctionName(parent))
                }
                (Self::FunctionName(mut parent), Self::Run(child)) => {
                    parent.children.push(OMathChild::Run(Box::new(child)));
                    Ok(Self::FunctionName(parent))
                }
                (Self::GroupChar(mut parent), Self::GroupCharProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::GroupChar(parent))
                }
                (Self::LimitLower(mut parent), Self::LimitLowerProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::LimitLower(parent))
                }
                (Self::LimitLower(mut parent), Self::Base(child)) => {
                    parent.base = child;
                    Ok(Self::LimitLower(parent))
                }
                (Self::LimitLower(mut parent), Self::Limit(child)) => {
                    parent.limit = child;
                    Ok(Self::LimitLower(parent))
                }
                (Self::LimitUpper(mut parent), Self::LimitUpperProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::LimitUpper(parent))
                }
                (Self::LimitUpper(mut parent), Self::Base(child)) => {
                    parent.base = child;
                    Ok(Self::LimitUpper(parent))
                }
                (Self::LimitUpper(mut parent), Self::Limit(child)) => {
                    parent.limit = child;
                    Ok(Self::LimitUpper(parent))
                }
                (Self::Matrix(mut parent), Self::MatrixProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Matrix(parent))
                }
                (Self::Matrix(parent), Self::MatrixRow(child)) => {
                    Ok(Self::Matrix(parent.add_row(child)))
                }
                (Self::Nary(mut parent), Self::NaryProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Nary(parent))
                }
                (Self::Nary(mut parent), Self::SubArgument(child)) => {
                    parent.sub_argument = child;
                    Ok(Self::Nary(parent))
                }
                (Self::Nary(mut parent), Self::SuperArgument(child)) => {
                    parent.super_argument = child;
                    Ok(Self::Nary(parent))
                }
                (Self::Nary(mut parent), Self::Base(child)) => {
                    parent.base = child;
                    Ok(Self::Nary(parent))
                }
                (Self::Para(mut parent), Self::ParaProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Para(parent))
                }
                (Self::Para(parent), Self::OMath(child)) => Ok(Self::Para(parent.add_math(child))),
                (Self::ParaProperty(parent), Self::Justification(child)) => {
                    Ok(Self::ParaProperty(parent.justification(child)))
                }
                (Self::Phantom(mut parent), Self::PhantomProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Phantom(parent))
                }
                (Self::PreSubSuper(mut parent), Self::PreSubSuperProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::PreSubSuper(parent))
                }
                (Self::PreSubSuper(mut parent), Self::SubArgument(child)) => {
                    parent.sub_argument = child;
                    Ok(Self::PreSubSuper(parent))
                }
                (Self::PreSubSuper(mut parent), Self::SuperArgument(child)) => {
                    parent.super_argument = child;
                    Ok(Self::PreSubSuper(parent))
                }
                (Self::PreSubSuper(mut parent), Self::Base(child)) => {
                    parent.base = child;
                    Ok(Self::PreSubSuper(parent))
                }
                (Self::Radical(mut parent), Self::RadicalProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Radical(parent))
                }
                (Self::Radical(mut parent), Self::Degree(child)) => {
                    parent.degree = Some(child);
                    Ok(Self::Radical(parent))
                }
                (Self::Radical(mut parent), Self::Base(child)) => {
                    parent.base = child;
                    Ok(Self::Radical(parent))
                }
                (Self::Run(mut parent), Self::RunProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Run(parent))
                }
                (Self::Run(parent), Self::Text(child)) => {
                    Ok(Self::Run(parent.add_text(child.text)))
                }
                (Self::BoxProperty(parent), Self::ManualBreak(child)) => {
                    Ok(Self::BoxProperty(parent.manual_break(child)))
                }
                (Self::BoxProperty(parent), Self::ControlProperty(child)) => Ok(Self::BoxProperty(
                    parent.control_property(child.run_property.clone()),
                )),
                (Self::AccentProperty(parent), Self::ControlProperty(child)) => Ok(
                    Self::AccentProperty(parent.control_property(child.run_property.clone())),
                ),
                (Self::BarProperty(parent), Self::ControlProperty(child)) => Ok(Self::BarProperty(
                    parent.control_property(child.run_property.clone()),
                )),
                (Self::BorderBoxProperty(parent), Self::ControlProperty(child)) => Ok(
                    Self::BorderBoxProperty(parent.control_property(child.run_property.clone())),
                ),
                (Self::DelimiterProperty(parent), Self::ControlProperty(child)) => Ok(
                    Self::DelimiterProperty(parent.control_property(child.run_property.clone())),
                ),
                (Self::EquationArrayProperty(parent), Self::ControlProperty(child)) => {
                    Ok(Self::EquationArrayProperty(
                        parent.control_property(child.run_property.clone()),
                    ))
                }
                (Self::FractionProperty(parent), Self::ControlProperty(child)) => Ok(
                    Self::FractionProperty(parent.control_property(child.run_property.clone())),
                ),
                (Self::FunctionProperty(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(OMathControlProperty::new(child.run_property));
                    Ok(Self::FunctionProperty(parent))
                }
                (Self::GroupCharProperty(parent), Self::ControlProperty(child)) => Ok(
                    Self::GroupCharProperty(parent.control_property(child.run_property.clone())),
                ),
                (Self::LimitLowerProperty(_), Self::ControlProperty(child)) => {
                    Ok(Self::LimitLowerProperty(OMathLimitLowerProperty::new(
                        child.run_property.clone(),
                    )))
                }
                (Self::LimitUpperProperty(_), Self::ControlProperty(child)) => {
                    Ok(Self::LimitUpperProperty(OMathLimitUpperProperty::new(
                        child.run_property.clone(),
                    )))
                }
                (Self::MatrixProperty(mut parent), Self::ControlProperty(child)) => {
                    parent.control_property = Some(child);
                    Ok(Self::MatrixProperty(parent))
                }
                (Self::NaryProperty(parent), Self::ControlProperty(child)) => Ok(
                    Self::NaryProperty(parent.control_property(child.run_property.clone())),
                ),
                (Self::PhantomProperty(parent), Self::ControlProperty(child)) => Ok(
                    Self::PhantomProperty(parent.control_property(child.run_property.clone())),
                ),
                (Self::PreSubSuperProperty(_), Self::ControlProperty(child)) => {
                    Ok(Self::PreSubSuperProperty(OMathPreSubSuperProperty::new(
                        child.run_property.clone(),
                    )))
                }
                (Self::RadicalProperty(parent), Self::ControlProperty(child)) => Ok(
                    Self::RadicalProperty(parent.control_property(child.run_property.clone())),
                ),
                (Self::RunProperty(parent), Self::ManualBreak(child)) => {
                    Ok(Self::RunProperty(parent.manual_break(child)))
                }
                (Self::SubscriptProperty(_), Self::ControlProperty(child)) => {
                    Ok(Self::SubscriptProperty(OMathSubscriptProperty::new(
                        child.run_property.clone(),
                    )))
                }
                (Self::Subscript(mut parent), Self::SubscriptProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Subscript(parent))
                }
                (Self::Subscript(mut parent), Self::Base(child)) => {
                    parent.base = child;
                    Ok(Self::Subscript(parent))
                }
                (Self::Subscript(mut parent), Self::SubArgument(child)) => {
                    parent.sub_argument = child;
                    Ok(Self::Subscript(parent))
                }
                (Self::SubSuperscriptProperty(parent), Self::ControlProperty(child)) => {
                    Ok(Self::SubSuperscriptProperty(
                        parent.control_property(child.run_property.clone()),
                    ))
                }
                (Self::SubSuperscript(mut parent), Self::SubSuperscriptProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::SubSuperscript(parent))
                }
                (Self::SubSuperscript(mut parent), Self::Base(child)) => {
                    parent.base = child;
                    Ok(Self::SubSuperscript(parent))
                }
                (Self::SubSuperscript(mut parent), Self::SubArgument(child)) => {
                    parent.sub_argument = child;
                    Ok(Self::SubSuperscript(parent))
                }
                (Self::SubSuperscript(mut parent), Self::SuperArgument(child)) => {
                    parent.super_argument = child;
                    Ok(Self::SubSuperscript(parent))
                }
                (Self::SuperscriptProperty(_), Self::ControlProperty(child)) => {
                    Ok(Self::SuperscriptProperty(OMathSuperscriptProperty::new(
                        child.run_property.clone(),
                    )))
                }
                (Self::Superscript(mut parent), Self::SuperscriptProperty(child)) => {
                    parent.property = Some(child);
                    Ok(Self::Superscript(parent))
                }
                (Self::Superscript(mut parent), Self::Base(child)) => {
                    parent.base = child;
                    Ok(Self::Superscript(parent))
                }
                (Self::Superscript(mut parent), Self::SuperArgument(child)) => {
                    parent.super_argument = child;
                    Ok(Self::Superscript(parent))
                }
                (parent, child) => Err(format!(
                    "unsupported child attachment: {} within {}",
                    child.tag(),
                    parent.tag()
                )),
            }
        }
    }

    impl BuildXML for MinimalNode {
        fn build_to<W: Write>(
            &self,
            stream: crate::xml::writer::EventWriter<W>,
        ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
            match self {
                Self::OMath(value) => value.build_to(stream),
                Self::Base(value) => value.build_to(stream),
                Self::Degree(value) => value.build_to(stream),
                Self::SubArgument(value) => value.build_to(stream),
                Self::SuperArgument(value) => value.build_to(stream),
                Self::Limit(value) => value.build_to(stream),
                Self::Accent(value) => value.build_to(stream),
                Self::AccentProperty(value) => value.build_to(stream),
                Self::ArgumentProperty(value) => value.build_to(stream),
                Self::Bar(value) => value.build_to(stream),
                Self::BarProperty(value) => value.build_to(stream),
                Self::BorderBox(value) => value.build_to(stream),
                Self::BorderBoxProperty(value) => value.build_to(stream),
                Self::Box(value) => value.build_to(stream),
                Self::ControlProperty(value) => value.build_to(stream),
                Self::Delimiter(value) => value.build_to(stream),
                Self::DelimiterProperty(value) => value.build_to(stream),
                Self::EquationArray(value) => value.build_to(stream),
                Self::EquationArrayProperty(value) => value.build_to(stream),
                Self::Fraction(value) => value.build_to(stream),
                Self::FractionProperty(value) => value.build_to(stream),
                Self::Function(value) => value.build_to(stream),
                Self::FunctionName(value) => value.build_to(stream),
                Self::FunctionProperty(value) => value.build_to(stream),
                Self::GroupChar(value) => value.build_to(stream),
                Self::GroupCharProperty(value) => value.build_to(stream),
                Self::Justification(value) => value.build_to(stream),
                Self::LimitLower(value) => value.build_to(stream),
                Self::LimitUpper(value) => value.build_to(stream),
                Self::LimitUpperProperty(value) => value.build_to(stream),
                Self::ManualBreak(value) => value.build_to(stream),
                Self::Matrix(value) => value.build_to(stream),
                Self::MatrixProperty(value) => XMLBuilder::from(stream)
                    .open_omath_matrix_property()?
                    .apply_opt(value.base_justification.as_ref(), |item, b| {
                        b.omath_char_tag("m:baseJc", &item.to_string())
                    })?
                    .apply_opt(value.hide_placeholder, |item, b| {
                        b.omath_on_off("m:plcHide", item)
                    })?
                    .apply_opt(value.row_spacing_rule, |item, b| {
                        b.omath_integer_value("m:rSpRule", item)
                    })?
                    .apply_opt(value.column_gap_rule, |item, b| {
                        b.omath_integer_value("m:cGpRule", item)
                    })?
                    .apply_opt(value.row_spacing, |item, b| {
                        b.omath_integer_value("m:rSp", item)
                    })?
                    .apply_opt(value.column_spacing, |item, b| {
                        b.omath_integer_value("m:cSp", item)
                    })?
                    .apply_opt(value.column_gap, |item, b| {
                        b.omath_integer_value("m:cGp", item)
                    })?
                    .apply_opt(value.column_justification.as_ref(), |item, b| {
                        b.open_omath_matrix_columns()?
                            .open_omath_matrix_column()?
                            .open_omath_matrix_column_property()?
                            .omath_integer_value("m:count", 1)?
                            .omath_char_tag("m:mcJc", &item.to_string())?
                            .close()?
                            .close()?
                            .close()
                    })?
                    .add_optional_child(&value.control_property)?
                    .close()?
                    .into_inner(),
                Self::MatrixRow(value) => value.build_to(stream),
                Self::Nary(value) => value.build_to(stream),
                Self::NaryProperty(value) => value.build_to(stream),
                Self::Numerator(value) => value.build_to(stream),
                Self::Denominator(value) => value.build_to(stream),
                Self::Para(value) => value.build_to(stream),
                Self::ParaProperty(value) => value.build_to(stream),
                Self::Phantom(value) => value.build_to(stream),
                Self::PhantomProperty(value) => value.build_to(stream),
                Self::PreSubSuper(value) => value.build_to(stream),
                Self::PreSubSuperProperty(value) => value.build_to(stream),
                Self::Radical(value) => value.build_to(stream),
                Self::RadicalProperty(value) => value.build_to(stream),
                Self::Run(value) => value.build_to(stream),
                Self::RunProperty(value) => value.build_to(stream),
                Self::BoxProperty(value) => value.build_to(stream),
                Self::SubscriptProperty(value) => value.build_to(stream),
                Self::Subscript(value) => value.build_to(stream),
                Self::SubSuperscript(value) => value.build_to(stream),
                Self::SubSuperscriptProperty(value) => value.build_to(stream),
                Self::LimitLowerProperty(value) => value.build_to(stream),
                Self::Superscript(value) => value.build_to(stream),
                Self::SuperscriptProperty(value) => value.build_to(stream),
                Self::Text(value) => value.build_to(stream),
            }
        }
    }

    pub(crate) fn xml(node: &impl BuildXML) -> String {
        String::from_utf8(node.build()).expect("xml should be utf-8")
    }

    pub(crate) fn assert_child_is_within_parent(xml: &str, child: &str, parent: &str) {
        let open = format!("<{parent}>");
        let close = format!("</{parent}>");
        let start = xml
            .find(&open)
            .unwrap_or_else(|| panic!("missing parent open tag: {parent}"));
        let end = xml[start..]
            .find(&close)
            .map(|offset| start + offset)
            .unwrap_or_else(|| panic!("missing parent close tag: {parent}"));
        let inner = &xml[start + open.len()..end];

        assert!(
            inner.contains(&format!("<{child}>")) || inner.contains(&format!("<{child} ")),
            "expected {child} inside {parent}, got xml: {xml}"
        );
    }

    pub(crate) fn create_node(tag: &str) -> MinimalNode {
        match tag {
            "m:oMath" => MinimalNode::OMath(OMath::new().add_run(OMathRun::new().add_text("x"))),
            "m:e" => MinimalNode::Base(OMathBase::new().add_run(OMathRun::new().add_text("x"))),
            "m:deg" => {
                MinimalNode::Degree(OMathDegree::new().add_run(OMathRun::new().add_text("2")))
            }
            "m:sub" => MinimalNode::SubArgument(
                OMathSubArgument::new().add_run(OMathRun::new().add_text("i")),
            ),
            "m:sup" => MinimalNode::SuperArgument(
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
            ),
            "m:lim" => {
                MinimalNode::Limit(OMathLimit::new().add_run(OMathRun::new().add_text("x→0")))
            }
            "m:acc" => MinimalNode::Accent(OMathAccent::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
            "m:accPr" => MinimalNode::AccentProperty(OMathAccentProperty::default()),
            "m:argPr" => MinimalNode::ArgumentProperty(OMathArgumentProperty::new()),
            "m:bar" => MinimalNode::Bar(OMathBar::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
            "m:barPr" => MinimalNode::BarProperty(OMathBarProperty::default()),
            "m:borderBox" => MinimalNode::BorderBox(OMathBorderBox::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
            "m:borderBoxPr" => MinimalNode::BorderBoxProperty(OMathBorderBoxProperty::default()),
            "m:box" => MinimalNode::Box(OMathBox::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
            "m:ctrlPr" => {
                MinimalNode::ControlProperty(OMathControlProperty::new(RunProperty::new().bold()))
            }
            "m:d" => MinimalNode::Delimiter(
                OMathDelimiter::new()
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("x"))),
            ),
            "m:dPr" => MinimalNode::DelimiterProperty(OMathDelimiterProperty::default()),
            "m:eqArr" => MinimalNode::EquationArray(
                OMathEquationArray::new()
                    .add_element(OMathBase::new().add_run(OMathRun::new().add_text("x=1"))),
            ),
            "m:eqArrPr" => {
                MinimalNode::EquationArrayProperty(OMathEquationArrayProperty::default())
            }
            "m:f" => MinimalNode::Fraction(OMathFraction::new(
                OMathNumerator::new().add_run(OMathRun::new().add_text("1")),
                OMathDenominator::new().add_run(OMathRun::new().add_text("2")),
            )),
            "m:fPr" => MinimalNode::FractionProperty(OMathFractionProperty::default()),
            "m:func" => MinimalNode::Function(OMathFunction::new(
                OMathFunctionName::new().add_run(OMathRun::new().add_text("sin")),
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
            "m:fName" => MinimalNode::FunctionName(
                OMathFunctionName::new().add_run(OMathRun::new().add_text("sin")),
            ),
            "m:funcPr" => MinimalNode::FunctionProperty(OMathFunctionProperty::default()),
            "m:groupChr" => MinimalNode::GroupChar(OMathGroupChar::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
            "m:groupChrPr" => MinimalNode::GroupCharProperty(OMathGroupCharProperty::default()),
            "m:jc" => {
                MinimalNode::Justification(OMathJustification::new(OMathJustificationType::Center))
            }
            "m:limLow" => MinimalNode::LimitLower(OMathLimitLower::new(
                OMathBase::new().add_run(OMathRun::new().add_text("lim")),
                OMathLimit::new().add_run(OMathRun::new().add_text("x→0")),
            )),
            "m:limLowPr" => MinimalNode::LimitLowerProperty(OMathLimitLowerProperty::default()),
            "m:limUpp" => MinimalNode::LimitUpper(OMathLimitUpper::new(
                OMathBase::new().add_run(OMathRun::new().add_text("lim")),
                OMathLimit::new().add_run(OMathRun::new().add_text("x→∞")),
            )),
            "m:limUppPr" => MinimalNode::LimitUpperProperty(OMathLimitUpperProperty::default()),
            "m:brk" => MinimalNode::ManualBreak(OMathManualBreak::new().align_at(1)),
            "m:m" => MinimalNode::Matrix(
                OMathMatrix::new().add_row(
                    OMathMatrixRow::new()
                        .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("x"))),
                ),
            ),
            "m:mPr" => MinimalNode::MatrixProperty(OMathMatrixProperty::default()),
            "m:mr" => MinimalNode::MatrixRow(
                OMathMatrixRow::new()
                    .add_cell(OMathBase::new().add_run(OMathRun::new().add_text("x"))),
            ),
            "m:nary" => MinimalNode::Nary(OMathNary::new(
                OMathSubArgument::new().add_run(OMathRun::new().add_text("i=0")),
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
            "m:naryPr" => MinimalNode::NaryProperty(OMathNaryProperty::default()),
            "m:num" => {
                MinimalNode::Numerator(OMathNumerator::new().add_run(OMathRun::new().add_text("a")))
            }
            "m:den" => MinimalNode::Denominator(
                OMathDenominator::new().add_run(OMathRun::new().add_text("b")),
            ),
            "m:oMathPara" => MinimalNode::Para(
                OMathPara::new().add_math(OMath::new().add_run(OMathRun::new().add_text("x"))),
            ),
            "m:oMathParaPr" => MinimalNode::ParaProperty(OMathParaProperty::default()),
            "m:phant" => MinimalNode::Phantom(OMathPhantom::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
            "m:phantPr" => MinimalNode::PhantomProperty(OMathPhantomProperty::default()),
            "m:sPre" => MinimalNode::PreSubSuper(OMathPreSubSuper::new(
                OMathSubArgument::new().add_run(OMathRun::new().add_text("m")),
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                OMathBase::new().add_run(OMathRun::new().add_text("T")),
            )),
            "m:sPrePr" => MinimalNode::PreSubSuperProperty(OMathPreSubSuperProperty::default()),
            "m:rad" => MinimalNode::Radical(OMathRadical::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )),
            "m:radPr" => MinimalNode::RadicalProperty(OMathRadicalProperty::default()),
            "m:r" => MinimalNode::Run(OMathRun::new().add_text("x")),
            "m:rPr" => MinimalNode::RunProperty(OMathRunProperty::default()),
            "m:boxPr" => MinimalNode::BoxProperty(OMathBoxProperty::default()),
            "m:sSubPr" => MinimalNode::SubscriptProperty(OMathSubscriptProperty::default()),
            "m:sSub" => MinimalNode::Subscript(OMathSubscript::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
                OMathSubArgument::new().add_run(OMathRun::new().add_text("i")),
            )),
            "m:sSubSup" => MinimalNode::SubSuperscript(OMathSubSuperscript::new(
                OMathBase::new().add_run(OMathRun::new().add_text("T")),
                OMathSubArgument::new().add_run(OMathRun::new().add_text("m")),
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
            )),
            "m:sSubSupPr" => {
                MinimalNode::SubSuperscriptProperty(OMathSubSuperscriptProperty::default())
            }
            "m:sSup" => MinimalNode::Superscript(OMathSuperscript::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("2")),
            )),
            "m:sSupPr" => MinimalNode::SuperscriptProperty(OMathSuperscriptProperty::default()),
            "m:t" => MinimalNode::Text(OMathText::new("x")),
            _ => panic!("missing minimal xml builder for tag={tag}"),
        }
    }
}

#[cfg(test)]
macro_rules! within_case {
    ($child:literal within $parent:literal) => {{
        let built = $crate::documents::elements::omath::test_support::xml(
            &$crate::documents::elements::omath::test_support::create_node($parent)
                .add_child($crate::documents::elements::omath::test_support::create_node($child))
                .unwrap_or_else(|error| panic!("{error}")),
        );
        $crate::documents::elements::omath::test_support::assert_child_is_within_parent(
            &built, $child, $parent,
        );
        built
    }};
    ($child:literal within $first_parent:literal $(| $rest_parent:literal)+) => {{
        let built = $crate::documents::elements::omath::test_support::xml(
            &$crate::documents::elements::omath::test_support::create_node($first_parent)
                .add_child($crate::documents::elements::omath::test_support::create_node($child))
                .unwrap_or_else(|error| panic!("{error}")),
        );
        $crate::documents::elements::omath::test_support::assert_child_is_within_parent(
            &built, $child, $first_parent,
        );
        $(
            {
                let built = $crate::documents::elements::omath::test_support::xml(
                    &$crate::documents::elements::omath::test_support::create_node($rest_parent)
                        .add_child($crate::documents::elements::omath::test_support::create_node($child))
                        .unwrap_or_else(|error| panic!("{error}")),
                );
                $crate::documents::elements::omath::test_support::assert_child_is_within_parent(
                    &built, $child, $rest_parent,
                );
            }
        )+
        built
    }};
}

#[cfg(test)]
macro_rules! not_within_case {
    ($child:literal within $parent:literal) => {{
        assert!(
            $crate::documents::elements::omath::test_support::create_node($parent)
                .add_child($crate::documents::elements::omath::test_support::create_node($child))
                .is_err(),
            "did not expect {} within {} to be attachable",
            $child,
            $parent
        );
    }};
    ($child:literal within $first_parent:literal $(| $rest_parent:literal)+) => {{
        assert!(
            $crate::documents::elements::omath::test_support::create_node($first_parent)
                .add_child($crate::documents::elements::omath::test_support::create_node($child))
                .is_err(),
            "did not expect {} within {} to be attachable",
            $child,
            $first_parent
        );
        $(
            assert!(
                $crate::documents::elements::omath::test_support::create_node($rest_parent)
                    .add_child($crate::documents::elements::omath::test_support::create_node($child))
                    .is_err(),
                "did not expect {} within {} to be attachable",
                $child,
                $rest_parent
            );
        )+
    }};
}

#[cfg(test)]
macro_rules! case {
    ($child:literal within $parent:literal) => {
        within_case!($child within $parent)
    };
    ($child:literal within $first_parent:literal $(| $rest_parent:literal)+) => {
        within_case!($child within $first_parent $(| $rest_parent)+)
    };
}

#[cfg(test)]
macro_rules! not_case {
    ($child:literal within $parent:literal) => {
        not_within_case!($child within $parent)
    };
    ($child:literal within $first_parent:literal $(| $rest_parent:literal)+) => {
        not_within_case!($child within $first_parent $(| $rest_parent)+)
    };
}

#[cfg(test)]
macro_rules! case_test {
    ($name:ident, $child:literal within $parent:literal) => {
        #[test]
        fn $name() {
            case!($child within $parent);
        }
    };
    ($name:ident, $child:literal within $first_parent:literal $(| $rest_parent:literal)+) => {
        #[test]
        fn $name() {
            case!($child within $first_parent $(| $rest_parent)+);
        }
    };
    ($name:ident, not $child:literal within $parent:literal) => {
        #[test]
        fn $name() {
            not_case!($child within $parent);
        }
    };
    ($name:ident, not $child:literal within $first_parent:literal $(| $rest_parent:literal)+) => {
        #[test]
        fn $name() {
            not_case!($child within $first_parent $(| $rest_parent)+);
        }
    };
}

mod accent;
mod accent_property;
mod argument_property;
mod bar;
mod bar_property;
mod border_box;
mod border_box_property;
mod box_property;
mod containers;
mod control_property;
mod delimiter;
mod delimiter_property;
mod equation_array;
mod equation_array_property;
mod fraction;
mod fraction_property;
mod function;
mod function_name;
mod function_property;
mod group_char;
mod group_char_property;
mod justification;
mod limit_lower;
mod limit_lower_property;
mod limit_upper;
mod limit_upper_property;
mod manual_break;
mod math_box;
mod matrix;
mod matrix_row;
mod nary;
mod nary_property;
mod para;
mod para_property;
mod phantom;
mod phantom_property;
mod pre_sub_super;
mod pre_sub_super_property;
mod radical;
mod radical_property;
mod raw_omath;
mod run;
mod run_property;
mod sub_superscript;
mod sub_superscript_property;
mod subscript;
mod subscript_property;
mod superscript;
mod superscript_property;
mod text;

pub use accent::*;
pub use accent_property::*;
pub use argument_property::*;
pub use bar::*;
pub use bar_property::*;
pub use border_box::*;
pub use border_box_property::*;
pub use box_property::*;
pub use containers::*;
pub use control_property::*;
pub use delimiter::*;
pub use delimiter_property::*;
pub use equation_array::*;
pub use equation_array_property::*;
pub use fraction::*;
pub use fraction_property::*;
pub use function::*;
pub use function_name::*;
pub use function_property::*;
pub use group_char::*;
pub use group_char_property::*;
pub use justification::*;
pub use limit_lower::*;
pub use limit_lower_property::*;
pub use limit_upper::*;
pub use limit_upper_property::*;
pub use manual_break::*;
pub use math_box::*;
pub use matrix::*;
pub use matrix_row::*;
pub use nary::*;
pub use nary_property::*;
pub use para::*;
pub use para_property::*;
pub use phantom::*;
pub use phantom_property::*;
pub use pre_sub_super::*;
pub use pre_sub_super_property::*;
pub use radical::*;
pub use radical_property::*;
pub use raw_omath::*;
pub use run::*;
pub use run_property::*;
pub use sub_superscript::*;
pub use sub_superscript_property::*;
pub use subscript::*;
pub use subscript_property::*;
pub use superscript::*;
pub use superscript_property::*;
pub use text::*;
