mod accent;
mod bar;
mod containers;
mod delimiter;
mod fraction;
mod function;
mod group_char;
mod layout;
mod limit;
mod nary;
mod para;
mod radical;
mod run;
mod sub_superscript;
mod subscript;
mod superscript;

use std::io::Read;
use std::str::FromStr;

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OMathXMLElement {
    OMathPara,
    OMathParaPr,
    OMath,
    Run,
    RunProperty,
    Text,
    Justification,
    ArgumentProperty,
    ArgumentSize,
    ControlProperty,
    Fraction,
    FractionProperty,
    Type,
    Numerator,
    Denominator,
    Base,
    Accent,
    AccentProperty,
    AccentChar,
    Bar,
    BarProperty,
    Position,
    Delimiter,
    DelimiterProperty,
    BeginChar,
    EndChar,
    SeparatorChar,
    Grow,
    Shape,
    Radical,
    RadicalProperty,
    Degree,
    HideDegree,
    Subscript,
    SubscriptProperty,
    SubArgument,
    SubSuperscript,
    SubSuperscriptProperty,
    Superscript,
    SuperscriptProperty,
    SuperArgument,
    Function,
    FunctionProperty,
    FunctionName,
    Nary,
    NaryProperty,
    LimitLocation,
    HideSubArgument,
    HideSuperArgument,
    LimitLower,
    LimitLowerProperty,
    LimitUpper,
    LimitUpperProperty,
    Limit,
    GroupChar,
    GroupCharProperty,
    VerticalJustification,
    Matrix,
    MatrixProperty,
    MatrixRow,
    BaseJustification,
    HidePlaceholder,
    RowSpacingRule,
    ColumnGapRule,
    RowSpacing,
    ColumnSpacing,
    ColumnGap,
    MatrixColumns,
    MatrixColumn,
    MatrixColumnProperty,
    MatrixColumnCount,
    MatrixColumnJustification,
    Box,
    BoxProperty,
    OperatorEmulator,
    NoBreak,
    Differential,
    BorderBox,
    BorderBoxProperty,
    HideTop,
    HideBottom,
    HideLeft,
    HideRight,
    StrikeHorizontal,
    StrikeVertical,
    StrikeBottomLeftToTopRight,
    StrikeTopLeftToBottomRight,
    Phantom,
    PhantomProperty,
    Show,
    ZeroWidth,
    ZeroAscent,
    ZeroDescent,
    Transparent,
    EquationArray,
    EquationArrayProperty,
    MaxDistribution,
    ObjectDistribution,
    PreSubSuper,
    PreSubSuperProperty,
    Break,
    Alignment,
    AlignScripts,
    Style,
    Script,
    NormalText,
    Literal,
    Unsupported,
}

impl FromStr for OMathXMLElement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "oMathPara" => Self::OMathPara,
            "oMathParaPr" => Self::OMathParaPr,
            "oMath" => Self::OMath,
            "r" => Self::Run,
            "rPr" => Self::RunProperty,
            "t" => Self::Text,
            "jc" => Self::Justification,
            "argPr" => Self::ArgumentProperty,
            "argSz" => Self::ArgumentSize,
            "ctrlPr" => Self::ControlProperty,
            "f" => Self::Fraction,
            "fPr" => Self::FractionProperty,
            "type" => Self::Type,
            "num" => Self::Numerator,
            "den" => Self::Denominator,
            "e" => Self::Base,
            "acc" => Self::Accent,
            "accPr" => Self::AccentProperty,
            "chr" => Self::AccentChar,
            "bar" => Self::Bar,
            "barPr" => Self::BarProperty,
            "pos" => Self::Position,
            "d" => Self::Delimiter,
            "dPr" => Self::DelimiterProperty,
            "begChr" => Self::BeginChar,
            "endChr" => Self::EndChar,
            "sepChr" => Self::SeparatorChar,
            "grow" => Self::Grow,
            "shp" => Self::Shape,
            "rad" => Self::Radical,
            "radPr" => Self::RadicalProperty,
            "deg" => Self::Degree,
            "degHide" => Self::HideDegree,
            "sSub" => Self::Subscript,
            "sSubPr" => Self::SubscriptProperty,
            "sub" => Self::SubArgument,
            "sSubSup" => Self::SubSuperscript,
            "sSubSupPr" => Self::SubSuperscriptProperty,
            "sSup" => Self::Superscript,
            "sSupPr" => Self::SuperscriptProperty,
            "sup" => Self::SuperArgument,
            "func" => Self::Function,
            "funcPr" => Self::FunctionProperty,
            "fName" => Self::FunctionName,
            "nary" => Self::Nary,
            "naryPr" => Self::NaryProperty,
            "limLoc" => Self::LimitLocation,
            "subHide" => Self::HideSubArgument,
            "supHide" => Self::HideSuperArgument,
            "limLow" => Self::LimitLower,
            "limLowPr" => Self::LimitLowerProperty,
            "limUpp" => Self::LimitUpper,
            "limUppPr" => Self::LimitUpperProperty,
            "lim" => Self::Limit,
            "groupChr" => Self::GroupChar,
            "groupChrPr" => Self::GroupCharProperty,
            "vertJc" => Self::VerticalJustification,
            "m" => Self::Matrix,
            "mPr" => Self::MatrixProperty,
            "mr" => Self::MatrixRow,
            "baseJc" => Self::BaseJustification,
            "plcHide" => Self::HidePlaceholder,
            "rSpRule" => Self::RowSpacingRule,
            "cGpRule" => Self::ColumnGapRule,
            "rSp" => Self::RowSpacing,
            "cSp" => Self::ColumnSpacing,
            "cGp" => Self::ColumnGap,
            "mcs" => Self::MatrixColumns,
            "mc" => Self::MatrixColumn,
            "mcPr" => Self::MatrixColumnProperty,
            "count" => Self::MatrixColumnCount,
            "mcJc" => Self::MatrixColumnJustification,
            "box" => Self::Box,
            "boxPr" => Self::BoxProperty,
            "opEmu" => Self::OperatorEmulator,
            "noBreak" => Self::NoBreak,
            "diff" => Self::Differential,
            "borderBox" => Self::BorderBox,
            "borderBoxPr" => Self::BorderBoxProperty,
            "hideTop" => Self::HideTop,
            "hideBot" => Self::HideBottom,
            "hideLeft" => Self::HideLeft,
            "hideRight" => Self::HideRight,
            "strikeH" => Self::StrikeHorizontal,
            "strikeV" => Self::StrikeVertical,
            "strikeBLTR" => Self::StrikeBottomLeftToTopRight,
            "strikeTLBR" => Self::StrikeTopLeftToBottomRight,
            "phant" => Self::Phantom,
            "phantPr" => Self::PhantomProperty,
            "show" => Self::Show,
            "zeroWid" => Self::ZeroWidth,
            "zeroAsc" => Self::ZeroAscent,
            "zeroDesc" => Self::ZeroDescent,
            "transp" => Self::Transparent,
            "eqArr" => Self::EquationArray,
            "eqArrPr" => Self::EquationArrayProperty,
            "maxDist" => Self::MaxDistribution,
            "objDist" => Self::ObjectDistribution,
            "sPre" => Self::PreSubSuper,
            "sPrePr" => Self::PreSubSuperProperty,
            "brk" => Self::Break,
            "aln" => Self::Alignment,
            "alnScr" => Self::AlignScripts,
            "sty" => Self::Style,
            "scr" => Self::Script,
            "nor" => Self::NormalText,
            "lit" => Self::Literal,
            _ => Self::Unsupported,
        })
    }
}

pub(crate) fn omath_val(attrs: &[OwnedAttribute]) -> Option<String> {
    read(attrs, "val")
}

pub(crate) fn omath_bool(attrs: &[OwnedAttribute]) -> bool {
    match omath_val(attrs) {
        Some(v) => !is_false(&v),
        None => true,
    }
}

pub(crate) fn omath_child_from_start<R: Read>(
    r: &mut EventReader<R>,
    name: &OwnedName,
    attrs: &[OwnedAttribute],
) -> Result<Option<OMathChild>, ReaderError> {
    if name.prefix.as_deref() != Some("m") {
        return Ok(None);
    }

    Ok(match OMathXMLElement::from_str(&name.local_name).unwrap() {
        OMathXMLElement::Run => Some(OMathChild::Run(Box::new(OMathRun::read(r, attrs)?))),
        OMathXMLElement::Fraction => Some(OMathChild::Fraction(Box::new(OMathFraction::read(
            r, attrs,
        )?))),
        OMathXMLElement::Accent => Some(OMathChild::Accent(Box::new(OMathAccent::read(r, attrs)?))),
        OMathXMLElement::Bar => Some(OMathChild::Bar(Box::new(OMathBar::read(r, attrs)?))),
        OMathXMLElement::Delimiter => Some(OMathChild::Delimiter(Box::new(OMathDelimiter::read(
            r, attrs,
        )?))),
        OMathXMLElement::Radical => {
            Some(OMathChild::Radical(Box::new(OMathRadical::read(r, attrs)?)))
        }
        OMathXMLElement::Subscript => Some(OMathChild::Subscript(Box::new(OMathSubscript::read(
            r, attrs,
        )?))),
        OMathXMLElement::SubSuperscript => Some(OMathChild::SubSuperscript(Box::new(
            OMathSubSuperscript::read(r, attrs)?,
        ))),
        OMathXMLElement::Superscript => Some(OMathChild::Superscript(Box::new(
            OMathSuperscript::read(r, attrs)?,
        ))),
        OMathXMLElement::Function => Some(OMathChild::Function(Box::new(OMathFunction::read(
            r, attrs,
        )?))),
        OMathXMLElement::Nary => Some(OMathChild::Nary(Box::new(OMathNary::read(r, attrs)?))),
        OMathXMLElement::LimitLower => Some(OMathChild::LimitLower(Box::new(
            OMathLimitLower::read(r, attrs)?,
        ))),
        OMathXMLElement::LimitUpper => Some(OMathChild::LimitUpper(Box::new(
            OMathLimitUpper::read(r, attrs)?,
        ))),
        OMathXMLElement::GroupChar => Some(OMathChild::GroupChar(Box::new(OMathGroupChar::read(
            r, attrs,
        )?))),
        OMathXMLElement::Matrix => Some(OMathChild::Matrix(Box::new(OMathMatrix::read(r, attrs)?))),
        OMathXMLElement::Box => Some(OMathChild::Box(Box::new(OMathBox::read(r, attrs)?))),
        OMathXMLElement::BorderBox => Some(OMathChild::BorderBox(Box::new(OMathBorderBox::read(
            r, attrs,
        )?))),
        OMathXMLElement::Phantom => {
            Some(OMathChild::Phantom(Box::new(OMathPhantom::read(r, attrs)?)))
        }
        OMathXMLElement::EquationArray => Some(OMathChild::EquationArray(Box::new(
            OMathEquationArray::read(r, attrs)?,
        ))),
        OMathXMLElement::PreSubSuper => Some(OMathChild::PreSubSuper(Box::new(
            OMathPreSubSuper::read(r, attrs)?,
        ))),
        _ => None,
    })
}

pub(crate) fn read_omath_children_until<R: Read>(
    r: &mut EventReader<R>,
    end: OMathXMLElement,
) -> Result<Vec<OMathChild>, ReaderError> {
    let mut children = Vec::new();
    loop {
        match r.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if let Some(child) = omath_child_from_start(r, &name, &attributes)? {
                    children.push(child);
                } else if name.prefix.as_deref() == Some("m") {
                    return Err(ReaderError::XMLReadError);
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.prefix.as_deref() == Some("m")
                    && OMathXMLElement::from_str(&name.local_name).unwrap() == end
                {
                    return Ok(children);
                }
            }
            Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
            Err(_) => return Err(ReaderError::XMLReadError),
            _ => {}
        }
    }
}
