use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathMatrix {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut matrix = OMathMatrix::new();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::MatrixProperty => {
                            matrix.property = Some(OMathMatrixProperty::read(r, &attributes)?)
                        }
                        OMathXMLElement::MatrixRow => {
                            matrix.rows.push(OMathMatrixRow::read(r, &attributes)?)
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Matrix
                    {
                        return Ok(matrix);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathMatrixProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathMatrixProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::BaseJustification => {
                            property.base_justification = Some(
                                OMathVerticalAlignmentType::from_str(
                                    &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                                )
                                .map_err(|_| ReaderError::XMLReadError)?,
                            )
                        }
                        OMathXMLElement::HidePlaceholder => {
                            property.hide_placeholder = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::RowSpacingRule => {
                            property.row_spacing_rule = Some(
                                omath_val(&attributes)
                                    .ok_or(ReaderError::XMLReadError)?
                                    .parse()
                                    .map_err(|_| ReaderError::XMLReadError)?,
                            )
                        }
                        OMathXMLElement::ColumnGapRule => {
                            property.column_gap_rule = Some(
                                omath_val(&attributes)
                                    .ok_or(ReaderError::XMLReadError)?
                                    .parse()
                                    .map_err(|_| ReaderError::XMLReadError)?,
                            )
                        }
                        OMathXMLElement::RowSpacing => {
                            property.row_spacing = Some(
                                omath_val(&attributes)
                                    .ok_or(ReaderError::XMLReadError)?
                                    .parse()
                                    .map_err(|_| ReaderError::XMLReadError)?,
                            )
                        }
                        OMathXMLElement::ColumnSpacing => {
                            property.column_spacing = Some(
                                omath_val(&attributes)
                                    .ok_or(ReaderError::XMLReadError)?
                                    .parse()
                                    .map_err(|_| ReaderError::XMLReadError)?,
                            )
                        }
                        OMathXMLElement::ColumnGap => {
                            property.column_gap = Some(
                                omath_val(&attributes)
                                    .ok_or(ReaderError::XMLReadError)?
                                    .parse()
                                    .map_err(|_| ReaderError::XMLReadError)?,
                            )
                        }
                        OMathXMLElement::MatrixColumns => read_matrix_columns(r, &mut property)?,
                        OMathXMLElement::ControlProperty => {
                            property.control_property =
                                Some(OMathControlProperty::read(r, &attributes)?)
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::MatrixProperty
                    {
                        return Ok(property);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

fn read_matrix_columns<R: Read>(
    r: &mut EventReader<R>,
    property: &mut OMathMatrixProperty,
) -> Result<(), ReaderError> {
    loop {
        match r.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.prefix.as_deref() == Some("m")
                    && OMathXMLElement::from_str(&name.local_name).unwrap()
                        == OMathXMLElement::MatrixColumn
                {
                    read_matrix_column(r, property, &attributes)?;
                } else {
                    return Err(ReaderError::XMLReadError);
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.prefix.as_deref() == Some("m")
                    && OMathXMLElement::from_str(&name.local_name).unwrap()
                        == OMathXMLElement::MatrixColumns
                {
                    return Ok(());
                }
            }
            Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
            Err(_) => return Err(ReaderError::XMLReadError),
            _ => {}
        }
    }
}

fn read_matrix_column<R: Read>(
    r: &mut EventReader<R>,
    property: &mut OMathMatrixProperty,
    _attrs: &[OwnedAttribute],
) -> Result<(), ReaderError> {
    loop {
        match r.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.prefix.as_deref() == Some("m")
                    && OMathXMLElement::from_str(&name.local_name).unwrap()
                        == OMathXMLElement::MatrixColumnProperty
                {
                    read_matrix_column_property(r, property, &attributes)?;
                } else {
                    return Err(ReaderError::XMLReadError);
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.prefix.as_deref() == Some("m")
                    && OMathXMLElement::from_str(&name.local_name).unwrap()
                        == OMathXMLElement::MatrixColumn
                {
                    return Ok(());
                }
            }
            Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
            Err(_) => return Err(ReaderError::XMLReadError),
            _ => {}
        }
    }
}

fn read_matrix_column_property<R: Read>(
    r: &mut EventReader<R>,
    property: &mut OMathMatrixProperty,
    _attrs: &[OwnedAttribute],
) -> Result<(), ReaderError> {
    loop {
        match r.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.prefix.as_deref() != Some("m") {
                    continue;
                }
                match OMathXMLElement::from_str(&name.local_name).unwrap() {
                    OMathXMLElement::MatrixColumnCount => {}
                    OMathXMLElement::MatrixColumnJustification => {
                        property.column_justification = Some(
                            OMathHorizontalAlignmentType::from_str(
                                &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                            )
                            .map_err(|_| ReaderError::XMLReadError)?,
                        );
                    }
                    _ => return Err(ReaderError::XMLReadError),
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.prefix.as_deref() == Some("m")
                    && OMathXMLElement::from_str(&name.local_name).unwrap()
                        == OMathXMLElement::MatrixColumnProperty
                {
                    return Ok(());
                }
            }
            Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
            Err(_) => return Err(ReaderError::XMLReadError),
            _ => {}
        }
    }
}

impl ElementReader for OMathMatrixRow {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut row = OMathMatrixRow::new();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Base
                    {
                        row.cells.push(OMathBase::read(r, &attributes)?);
                    } else {
                        return Err(ReaderError::XMLReadError);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::MatrixRow
                    {
                        return Ok(row);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathBox {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut base = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::BoxProperty => {
                            property = Some(OMathBoxProperty::read(r, &attributes)?)
                        }
                        OMathXMLElement::Base => base = Some(OMathBase::read(r, &attributes)?),
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Box
                    {
                        return Ok(OMathBox {
                            base: base.ok_or(ReaderError::XMLReadError)?,
                            property,
                        });
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathBoxProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathBoxProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::OperatorEmulator => {
                            property.operator_emulator = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::NoBreak => {
                            property.no_break = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::Differential => {
                            property.differential = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::Break => {
                            property.manual_break = Some(OMathManualBreak::read(r, &attributes)?)
                        }
                        OMathXMLElement::Alignment => {
                            property.align = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::ControlProperty => {
                            property.control_property =
                                Some(OMathControlProperty::read(r, &attributes)?)
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::BoxProperty
                    {
                        return Ok(property);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathBorderBox {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut base = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::BorderBoxProperty => {
                            property = Some(OMathBorderBoxProperty::read(r, &attributes)?)
                        }
                        OMathXMLElement::Base => base = Some(OMathBase::read(r, &attributes)?),
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::BorderBox
                    {
                        return Ok(OMathBorderBox {
                            base: base.ok_or(ReaderError::XMLReadError)?,
                            property,
                        });
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathBorderBoxProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathBorderBoxProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::HideTop => {
                            property.hide_top = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::HideBottom => {
                            property.hide_bottom = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::HideLeft => {
                            property.hide_left = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::HideRight => {
                            property.hide_right = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::StrikeHorizontal => {
                            property.strike_horizontal = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::StrikeVertical => {
                            property.strike_vertical = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::StrikeBottomLeftToTopRight => {
                            property.strike_bottom_left_to_top_right = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::StrikeTopLeftToBottomRight => {
                            property.strike_top_left_to_bottom_right = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::ControlProperty => {
                            property.control_property =
                                Some(OMathControlProperty::read(r, &attributes)?)
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::BorderBoxProperty
                    {
                        return Ok(property);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathPhantom {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut base = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::PhantomProperty => {
                            property = Some(OMathPhantomProperty::read(r, &attributes)?)
                        }
                        OMathXMLElement::Base => base = Some(OMathBase::read(r, &attributes)?),
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Phantom
                    {
                        return Ok(OMathPhantom {
                            base: base.ok_or(ReaderError::XMLReadError)?,
                            property,
                        });
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathPhantomProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathPhantomProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::Show => property.show = Some(omath_bool(&attributes)),
                        OMathXMLElement::ZeroWidth => {
                            property.zero_width = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::ZeroAscent => {
                            property.zero_ascent = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::ZeroDescent => {
                            property.zero_descent = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::Transparent => {
                            property.transparent = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::ControlProperty => {
                            property.control_property =
                                Some(OMathControlProperty::read(r, &attributes)?)
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::PhantomProperty
                    {
                        return Ok(property);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathEquationArray {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut value = OMathEquationArray::new();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::EquationArrayProperty => {
                            value.property = Some(OMathEquationArrayProperty::read(r, &attributes)?)
                        }
                        OMathXMLElement::Base => {
                            value.elements.push(OMathBase::read(r, &attributes)?)
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::EquationArray
                    {
                        return Ok(value);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathEquationArrayProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathEquationArrayProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::BaseJustification => {
                            property.base_justification = Some(
                                OMathVerticalAlignmentType::from_str(
                                    &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                                )
                                .map_err(|_| ReaderError::XMLReadError)?,
                            )
                        }
                        OMathXMLElement::MaxDistribution => {
                            property.max_distribution = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::ObjectDistribution => {
                            property.object_distribution = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::RowSpacingRule => {
                            property.row_spacing_rule = Some(
                                omath_val(&attributes)
                                    .ok_or(ReaderError::XMLReadError)?
                                    .parse()
                                    .map_err(|_| ReaderError::XMLReadError)?,
                            )
                        }
                        OMathXMLElement::RowSpacing => {
                            property.row_spacing = Some(
                                omath_val(&attributes)
                                    .ok_or(ReaderError::XMLReadError)?
                                    .parse()
                                    .map_err(|_| ReaderError::XMLReadError)?,
                            )
                        }
                        OMathXMLElement::ControlProperty => {
                            property.control_property =
                                Some(OMathControlProperty::read(r, &attributes)?)
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::EquationArrayProperty
                    {
                        return Ok(property);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathPreSubSuper {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut sub_argument = None;
        let mut super_argument = None;
        let mut base = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::PreSubSuperProperty => {
                            property = Some(OMathPreSubSuperProperty::read(r, &attributes)?)
                        }
                        OMathXMLElement::SubArgument => {
                            sub_argument = Some(OMathSubArgument::read(r, &attributes)?)
                        }
                        OMathXMLElement::SuperArgument => {
                            super_argument = Some(OMathSuperArgument::read(r, &attributes)?)
                        }
                        OMathXMLElement::Base => base = Some(OMathBase::read(r, &attributes)?),
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::PreSubSuper
                    {
                        return Ok(OMathPreSubSuper {
                            sub_argument: sub_argument.ok_or(ReaderError::XMLReadError)?,
                            super_argument: super_argument.ok_or(ReaderError::XMLReadError)?,
                            base: base.ok_or(ReaderError::XMLReadError)?,
                            property,
                        });
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathPreSubSuperProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathPreSubSuperProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::ControlProperty
                    {
                        property.control_property =
                            Some(OMathControlProperty::read(r, &attributes)?);
                    } else {
                        return Err(ReaderError::XMLReadError);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::PreSubSuperProperty
                    {
                        return Ok(property);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
