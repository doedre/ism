
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    NotEnoughInput { line_number: usize },
    WrongCommentFormat {
        line_number: usize,
        line: String,
        note: String
    },
    MissingField {
        line_number: usize,
        line: String,
        note: String
    },
    NotFloat {
        line_number: usize,
        line: String,
        note: String
    },
    NotInt {
        line_number: usize,
        line: String,
        note: String,
    },
    UnknownItem {
        line_number: usize,
        column: usize,
        value_width: usize,
        line: String,
        note: String
    },
    UnknownCollisionPartner {
        line_number: usize,
        line: String,
        note: String,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let linenum_width = 6;

        match self {
            Self::NotEnoughInput { line_number } => {
                write!(f, "{:>linenum_width$} |\n", line_number)?;
                write!(f, "{:>linenum_width$} | {:^<linenum_width$}\n", " ", "^")?;
                write!(f, "{:>linenum_width$} = Line {} is empty, but there should be more input.\n", " ", line_number)?;

                Ok(())
            },
            Self::WrongCommentFormat { line_number, line, note } => {
                write!(f, "{:>linenum_width$} | {}\n", line_number, line)?;
                write!(f, "{:>linenum_width$} | ^\n", " ")?;
                write!(f, "{:>linenum_width$} = {}.\n", " ", note)?;

                Ok(())
            },
            Self::MissingField { line_number, line, note } => {
                let line_len = line.len();
                write!(f, "{:>linenum_width$} | {}\n", line_number, line)?;
                write!(f, "{:>linenum_width$} | {:>line_len$} {:^<linenum_width$}\n", " ", " ", "^")?;
                write!(f, "{:>linenum_width$} = {}.\n", " ", note)?;

                Ok(())
            },
            Self::NotFloat { line_number, line, note } => {
                let line_len = line.len();
                write!(f, "{:>linenum_width$} | {}\n", line_number, line)?;
                write!(f, "{:>linenum_width$} | {:^<line_len$}\n", " ", "^")?;
                write!(f, "{:>linenum_width$} = {}.\n", " ", note)?;

                Ok(())
            },
            Self::NotInt { line_number, line, note } => {
                let line_len = line.len();
                write!(f, "{:>linenum_width$} | {}\n", line_number, line)?;
                write!(f, "{:>linenum_width$} | {:^<line_len$}\n", " ", "^")?;
                write!(f, "{:>linenum_width$} = {}.\n", " ", note)?;

                Ok(())
            },
            Self::UnknownItem { line_number, column, value_width, line, note } => {
                write!(f, "{:>linenum_width$} | {}\n", line_number, line.replace("\t", " "))?;
                write!(f, "{:>linenum_width$} | {:>column$}{:^<value_width$}\n", " ", " ", "^")?;
                write!(f, "{:>linenum_width$} = {}.\n", " ", note)?;

                Ok(())
            },
            Self::UnknownCollisionPartner { line_number, line, note } => {
                let skip = line.find(char::is_alphanumeric).unwrap_or(0);
                let item_len = line.split_whitespace().next().unwrap_or("").len();
                write!(f, "{:>linenum_width$} | {}\n", line_number, line)?;
                write!(f, "{:>linenum_width$} | {:>skip$}{:^<item_len$}\n", " ", " ", "^")?;
                write!(f, "{:>linenum_width$} = {}.\n", " ", note)?;

                Ok(())
            }
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct CollisionPartnerData {
    name: CollisionPartnerId,
    information: String,
    temperatures: Vec<f64>,
    rates: Vec<CollisionalRates>,
}

#[derive(Debug, Default, PartialEq)]
pub struct ElementData {
    name: String,
    information: String,
    weight: f64,
    energy_levels: Vec<EnergyLevel>,
    radiative_transitions: Vec<RadiativeTransition>,
    collision_partners: Vec<CollisionPartnerData>,
}

impl ElementData {
    fn validate_and_parse_comment(line_number: usize, line: &str) -> Result<Comment, ParseError> {
        match line.trim().starts_with("!") {
            true => Ok(line.parse().expect("Parsing comment should not fail")),
            false => Err(ParseError::WrongCommentFormat {
                line_number: line_number,
                line: String::from(line),
                note: String::from("Comment should begin with `!` character")
            })
        }
    }
}

impl std::str::FromStr for ElementData {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().enumerate();

        let mut line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: 1})?;
        let mut _comment: Comment = Self::validate_and_parse_comment(line.0, line.1)?;

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        let (name, mut information) = match line.1.parse::<ElementName>() {
            Ok(elem_name) => (elem_name.name, elem_name.information),
            Err(_) => panic!("Parsing element name should not fail")
        };

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        _comment = Self::validate_and_parse_comment(line.0, line.1)?;

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        let weight: f64 = match line.1.trim().parse() {
            Ok(w) => w,
            Err(_) => return Err(ParseError::NotFloat {
                line_number: line.0,
                line: String::from(line.1),
                note: String::from("Expected floating point number")
            })
        };

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        _comment = Self::validate_and_parse_comment(line.0, line.1)?;

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        let nlev = match line.1.parse::<NumberOfEnergyLevels>() {
            Ok(n) => n.0,
            Err(_) => return Err(ParseError::NotInt {
                line_number: line.0,
                line: String::from(line.1),
                note: String::from("Expected integer")
            })
        };

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        _comment = Self::validate_and_parse_comment(line.0, line.1)?;

        let energy_level_lines = lines.by_ref().take(nlev as usize);
        let energy_levels = energy_level_lines
            .map(|el| Ok(match el.1.parse::<EnergyLevel>() {
                Ok(enlev) => enlev,
                Err(e) => match e {
                    EnergyLevelParseError::MissingField{field, expected} => {
                        return Err(ParseError::MissingField {
                            line_number: el.0,
                            line: String::from(el.1),
                            note: format!("Missing field `{}` with value of {} type", field, expected)
                        })
                    },
                    EnergyLevelParseError::UnknownFormat{field, value, expected} => {
                        return Err(ParseError::UnknownItem {
                            line_number: el.0,
                            column: el.1.find(&value).unwrap_or(0),
                            value_width: value.len(),
                            line: String::from(el.1),
                            note: format!(
                                "Value `{}` from field `{}` has wrong type (should be {})",
                                value,
                                field,
                                expected
                            )
                        })
                    }
                }
            }))
            .collect::<Result<Vec<_>, _>>()?;

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        _comment = Self::validate_and_parse_comment(line.0, line.1)?;

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        let nlin = match line.1.parse::<NumberOfRadiativeTransitions>() {
            Ok(n) => n.0,
            Err(_) => return Err(ParseError::NotInt {
                line_number: line.0,
                line: String::from(line.1),
                note: String::from("Expected integer")
            })
        };

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        _comment = Self::validate_and_parse_comment(line.0, line.1)?;

        let radiative_transition_lines = lines.by_ref().take(nlin as usize);
        let radiative_transitions = radiative_transition_lines
            .map(|el| Ok(match el.1.parse::<RadiativeTransition>() {
                Ok(enlev) => enlev,
                Err(e) => match e {
                    RadiativeTransitionParseError::MissingField{field, expected} => {
                        return Err(ParseError::MissingField {
                            line_number: el.0,
                            line: String::from(el.1),
                            note: format!("Missing field `{}` with value of {} type", field, expected)
                        })
                    },
                    RadiativeTransitionParseError::UnknownFormat{field, value, expected} => {
                        return Err(ParseError::UnknownItem {
                            line_number: el.0,
                            column: el.1.find(&value).unwrap_or(0),
                            value_width: value.len(),
                            line: String::from(el.1),
                            note: format!(
                                "Value `{}` from field `{}` has wrong type (should be {})",
                                value,
                                field,
                                expected
                            )
                        })
                    }
                }
            }))
            .collect::<Result<Vec<_>, _>>()?;

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        _comment = Self::validate_and_parse_comment(line.0, line.1)?;

        line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
        let npart = match line.1.parse::<NumberOfCollisionPartners>() {
            Ok(n) => n.0,
            Err(_) => return Err(ParseError::NotInt {
                line_number: line.0,
                line: String::from(line.1),
                note: String::from("Expected integer")
            })
        };

        let mut collision_partners: Vec<CollisionPartnerData> = Vec::with_capacity(npart as usize);
        for _ in 1..(npart + 1) {
            line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
            _comment = Self::validate_and_parse_comment(line.0, line.1)?;

            line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
            let (name, information) = match line.1.parse::<CollisionPartnerName>() {
                Ok(cp_name) => (cp_name.name, cp_name.information),
                Err(_) => return Err(ParseError::UnknownCollisionPartner {
                    line_number: line.0,
                    line: String::from(line.1),
                    note: String::from("Unknown collision partner id (1=H2, 2=para-H2, 3=ortho-H2, 4=electrons, 5=H, 6=He, 7=H+)")
                })
            };

            line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
            _comment = Self::validate_and_parse_comment(line.0, line.1)?;

            line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
            let ncol = match line.1.parse::<NumberOfCollisionalTransitions>() {
                Ok(n) => n.0,
                Err(_) => return Err(ParseError::NotInt {
                    line_number: line.0,
                    line: String::from(line.1),
                    note: String::from("Expected integer")
                })
            };

            line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
            _comment = Self::validate_and_parse_comment(line.0, line.1)?;

            line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
            let _ntemp = match line.1.parse::<NumberOfCollisionalTemperatures>() {
                Ok(n) => n.0,
                Err(_) => return Err(ParseError::NotInt {
                    line_number: line.0,
                    line: String::from(line.1),
                    note: String::from("Expected integer")
                })
            };

            line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
            _comment = Self::validate_and_parse_comment(line.0, line.1)?;

            line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
            let temperatures = match line.1.parse::<CollisionalTemperatures>() {
                Ok(temps) => temps.0,
                Err(e) => return Err(ParseError::UnknownItem {
                    line_number: line.0,
                    column: line.1.find(&e.value).unwrap_or(0),
                    value_width: e.value.len(),
                    line: String::from(line.1),
                    note: format!(
                        "Value `{}` has wrong type (should be floating point number)",
                        e.value,
                    )
                })
            };

            line = lines.next().ok_or(ParseError::NotEnoughInput{line_number: line.0 + 1})?;
            _comment = Self::validate_and_parse_comment(line.0, line.1)?;

            let collisional_rates_lines = lines.by_ref().take(ncol as usize);
            let rates = collisional_rates_lines
                .map(|el| Ok(match el.1.parse::<CollisionalRates>() {
                    Ok(colrate) => colrate,
                    Err(e) => match e {
                        CollisionalRatesParseError::MissingField{field, expected} => {
                            return Err(ParseError::MissingField {
                                line_number: el.0,
                                line: String::from(el.1),
                                note: format!("Missing field `{}` with value of {} type", field, expected)
                            })
                        },
                        CollisionalRatesParseError::UnknownFormat{field, value, expected} => {
                            return Err(ParseError::UnknownItem {
                                line_number: el.0,
                                column: el.1.find(&value).unwrap_or(0),
                                value_width: value.len(),
                                line: String::from(el.1),
                                note: format!(
                                    "Value `{}` from field `{}` has wrong type (should be {})",
                                    value,
                                    field,
                                    expected
                                )
                            })
                        }
                    }
                }))
                .collect::<Result<Vec<_>, _>>()?;

            collision_partners.push(CollisionPartnerData {name, information, temperatures, rates});
        }

        let additional_info = lines
            .map(|el| if !el.1.trim().is_empty() {
                    Ok(match Self::validate_and_parse_comment(el.0, el.1) {
                        Ok(comment) => comment.0 + " ",
                        Err(_) => return Err(ParseError::WrongCommentFormat {
                            line_number: el.0,
                            line: String::from(el.1),
                            note: format!(
                                "{} collision partners were read, only comments with additional information should be left",
                                npart
                            )
                        })
                    })
                } else {
                    Ok(String::new())
                }
            )
            .collect::<Result<String, _>>()?;

        information.push_str(". ");
        information.push_str(&additional_info);

        Ok(Self { name, information, weight, energy_levels, radiative_transitions, collision_partners })
    }
}

#[derive(Debug, PartialEq)]
struct Comment(String);

impl std::str::FromStr for Comment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            String::from(s.trim_matches(|c| c == ' ' || c == '!' || c == '\n'))
        ))
    }
}

#[derive(Debug, PartialEq)]
struct ElementName {
    name: String,
    information: String,
}

impl std::str::FromStr for ElementName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (data_str, comment_str) = s
            .trim()
            .split_once(' ')
            .unwrap_or((s, ""));
        let name = String::from(data_str);
        let information = String::from(
            comment_str.trim_matches(|c| c == ' ' || c == '!' || c == '\n')
        );

        Ok(Self { name, information })
    }
}

#[derive(Debug, PartialEq)]
struct ElementWeight(f64);

impl std::str::FromStr for ElementWeight {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<f64>() {
            Ok(n) => Ok(ElementWeight(n)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, PartialEq)]
struct NumberOfEnergyLevels(u32);

impl std::str::FromStr for NumberOfEnergyLevels {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<u32>() {
            Ok(n) => Ok(Self(n)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ExpectedFieldValue {
    Integer,
    Float,
}

impl std::fmt::Display for ExpectedFieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedFieldValue::Integer => write!(f, "integer"),
            ExpectedFieldValue::Float => write!(f, "floating point number"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum SplittedFieldParseError<F> {
    MissingField {
        field: F,
        expected: ExpectedFieldValue,
    },
    UnknownFormat {
        field: F,
        value: String,
        expected: ExpectedFieldValue,
    },
}

#[derive(Debug, Default, PartialEq)]
struct EnergyLevel {
    level: u32,
    energy: f64,
    stat_weight: f64,
    qnums: String,
}

#[derive(Debug, PartialEq)]
enum EnergyLevelField {
    Level = 0,
    Energy,
    StatisticalWeight,
}

impl std::fmt::Display for EnergyLevelField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnergyLevelField::Level => write!(f, "level"),
            EnergyLevelField::Energy => write!(f, "energy [cm-1]"),
            EnergyLevelField::StatisticalWeight => write!(f, "statistical weight")
        }
    }
}

type EnergyLevelParseError = SplittedFieldParseError<EnergyLevelField>;

impl std::str::FromStr for EnergyLevel {
    type Err = EnergyLevelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split_whitespace();
        let mut values_beg = values.clone();

        let level = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: EnergyLevelField::Level,
                expected: ExpectedFieldValue::Integer,
            })?
            .parse::<u32>();

        let level = match level {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: EnergyLevelField::Level,
                value: String::from(values_beg.nth(EnergyLevelField::Level as usize).unwrap()),
                expected: ExpectedFieldValue::Integer,
            })
        };

        let energy = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: EnergyLevelField::Energy,
                expected: ExpectedFieldValue::Float,
            })?
            .parse::<f64>();

        let energy = match energy {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: EnergyLevelField::Energy,
                value: String::from(values_beg.nth(EnergyLevelField::Energy as usize).unwrap()),
                expected: ExpectedFieldValue::Float,
            })
        };

        let stat_weight = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: EnergyLevelField::StatisticalWeight,
                expected: ExpectedFieldValue::Float,
            })?
            .parse::<f64>();

        let stat_weight = match stat_weight {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: EnergyLevelField::StatisticalWeight,
                value: String::from(values_beg.nth(EnergyLevelField::StatisticalWeight as usize).unwrap()),
                expected: ExpectedFieldValue::Float,
            })
        };

        let qnums: String = values
            .map(|e| e.to_owned() + " ")
            .collect::<String>()
            .trim_matches(|c| c == ' ' || c == '!' || c == '\'' || c == '\n')
            .to_string();

        Ok(Self {
            level,
            energy,
            stat_weight,
            qnums
        })
    }
}

#[derive(Debug, PartialEq)]
struct NumberOfRadiativeTransitions(u32);

impl std::str::FromStr for NumberOfRadiativeTransitions {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<u32>() {
            Ok(n) => Ok(Self(n)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct RadiativeTransition {
    transition: u32,
    up: u32,
    low: u32,
    aeinst: f64,
    extra: String,
}

#[derive(Debug, PartialEq)]
enum RadiativeTransitionField {
    Transition = 0,
    UpperLevel,
    LowerLevel,
    SpontaneousDecayRate,
}

impl std::fmt::Display for RadiativeTransitionField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RadiativeTransitionField::Transition => write!(f, "transition"),
            RadiativeTransitionField::UpperLevel => write!(f, "upper level"),
            RadiativeTransitionField::LowerLevel => write!(f, "lower level"),
            RadiativeTransitionField::SpontaneousDecayRate => write!(f, "spontaneous decay rate [s-1]")
        }
    }
}

type RadiativeTransitionParseError = SplittedFieldParseError<RadiativeTransitionField>;

impl std::str::FromStr for RadiativeTransition {
    type Err = RadiativeTransitionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split_whitespace();
        let mut values_beg = values.clone();

        let transition = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: RadiativeTransitionField::Transition,
                expected: ExpectedFieldValue::Integer,
            })?
            .parse::<u32>();

        let transition = match transition {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: RadiativeTransitionField::Transition,
                value: String::from(values_beg.nth(RadiativeTransitionField::Transition as usize).unwrap()),
                expected: ExpectedFieldValue::Integer,
            })
        };

        let up = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: RadiativeTransitionField::UpperLevel,
                expected: ExpectedFieldValue::Integer,
            })?
            .parse::<u32>();

        let up = match up {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: RadiativeTransitionField::UpperLevel,
                value: String::from(values_beg.nth(RadiativeTransitionField::UpperLevel as usize).unwrap()),
                expected: ExpectedFieldValue::Integer,
            })
        };

        let low = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: RadiativeTransitionField::LowerLevel,
                expected: ExpectedFieldValue::Integer,
            })?
            .parse::<u32>();

        let low = match low {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: RadiativeTransitionField::LowerLevel,
                value: String::from(values_beg.nth(RadiativeTransitionField::LowerLevel as usize).unwrap()),
                expected: ExpectedFieldValue::Integer,
            })
        };

        let aeinst = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: RadiativeTransitionField::SpontaneousDecayRate,
                expected: ExpectedFieldValue::Float,
            })?
            .parse::<f64>();

        let aeinst = match aeinst {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: RadiativeTransitionField::SpontaneousDecayRate,
                value: String::from(values_beg.nth(RadiativeTransitionField::SpontaneousDecayRate as usize).unwrap()),
                expected: ExpectedFieldValue::Float,
            })
        };

        let extra: String = values
            .map(|e| e.to_owned() + " ")
            .collect::<String>()
            .trim_matches(|c| c == ' ' || c == '!' || c == '\'' || c == '\n')
            .to_string();

        Ok(Self {
            transition,
            up,
            low,
            aeinst,
            extra
        })
    }
}

#[derive(Debug, PartialEq)]
struct NumberOfCollisionPartners(u32);

impl std::str::FromStr for NumberOfCollisionPartners {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<u32>() {
            Ok(n) => Ok(Self(n)),
            Err(e) => Err(e),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, PartialEq)]
enum CollisionPartnerId {
    #[default]
    H2 = 1,
    pH2,
    oH2,
    electrons,
    HI,
    He,
    HII,
}

#[derive(Debug, PartialEq)]
struct CollisionPartnerIdParseError;

impl std::convert::From<std::num::ParseIntError> for CollisionPartnerIdParseError {
    fn from(_item: std::num::ParseIntError) -> Self {
        Self
    }
}

impl TryFrom<u32> for CollisionPartnerId {
    type Error = CollisionPartnerIdParseError;

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        match item {
            x if x == CollisionPartnerId::H2 as u32 => Ok(CollisionPartnerId::H2),
            x if x == CollisionPartnerId::pH2 as u32 => Ok(CollisionPartnerId::pH2),
            x if x == CollisionPartnerId::oH2 as u32 => Ok(CollisionPartnerId::oH2),
            x if x == CollisionPartnerId::electrons as u32 => Ok(CollisionPartnerId::electrons),
            x if x == CollisionPartnerId::HI as u32 => Ok(CollisionPartnerId::HI),
            x if x == CollisionPartnerId::He as u32 => Ok(CollisionPartnerId::He),
            x if x == CollisionPartnerId::HII as u32 => Ok(CollisionPartnerId::HII),
            _ => Err(CollisionPartnerIdParseError),
        }
    }
}

#[derive(Debug, PartialEq)]
struct CollisionPartnerName {
    name: CollisionPartnerId,
    information: String,
}

impl std::str::FromStr for CollisionPartnerName {
    type Err = CollisionPartnerIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (data_str, comment_str) = s
            .trim()
            .split_once(' ')
            .unwrap_or((s, ""));

        let name = match data_str.parse::<u32>() {
            Ok(n) => CollisionPartnerId::try_from(n)?,
            Err(e) => return Err(CollisionPartnerIdParseError::from(e)),
        };

        let information = String::from(
            comment_str.trim_matches(|c| c == ' ' || c == '!' || c == '\n')
        );

        Ok(Self { name, information })
    }
}

#[derive(Debug, PartialEq)]
struct NumberOfCollisionalTransitions(u32);

impl std::str::FromStr for NumberOfCollisionalTransitions {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<u32>() {
            Ok(n) => Ok(Self(n)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, PartialEq)]
struct NumberOfCollisionalTemperatures(u32);

impl std::str::FromStr for NumberOfCollisionalTemperatures {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<u32>() {
            Ok(n) => Ok(Self(n)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, PartialEq)]
struct CollisionalTemperatures(Vec<f64>);

#[derive(Debug, PartialEq)]
struct CollisionalTemperaturesParseError {
    value: String,
}

impl std::str::FromStr for CollisionalTemperatures {
    type Err = CollisionalTemperaturesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: Vec<f64> = vec!();

        for i in s.split_whitespace() {
            let item = match i.parse::<f64>() {
                Ok(n) => n,
                Err(_) => return Err(Self::Err { value: String::from(i) }),
            };

            result.push(item);
        }

        Ok(Self(result))
    }
}

#[derive(Debug, Default, PartialEq)]
struct CollisionalRates {
    transition: u32,
    up: u32,
    low: u32,
    rates: Vec<f64>,
}

#[derive(Debug, PartialEq)]
enum CollisionalRatesField {
    Transition = 0,
    UpperLevel,
    LowerLevel,
    RateCoefficients,
}

impl std::fmt::Display for CollisionalRatesField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollisionalRatesField::Transition => write!(f, "transition"),
            CollisionalRatesField::UpperLevel => write!(f, "upper level"),
            CollisionalRatesField::LowerLevel => write!(f, "lower level"),
            CollisionalRatesField::RateCoefficients => write!(f, "rate coefficients [cm3 s-1]")
        }
    }
}

type CollisionalRatesParseError = SplittedFieldParseError<CollisionalRatesField>;

impl std::str::FromStr for CollisionalRates {
    type Err = CollisionalRatesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split_whitespace();
        let mut values_beg = values.clone();

        let transition = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: CollisionalRatesField::Transition,
                expected: ExpectedFieldValue::Integer,
            })?
            .parse::<u32>();

        let transition = match transition {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: CollisionalRatesField::Transition,
                value: String::from(values_beg.nth(CollisionalRatesField::Transition as usize).unwrap()),
                expected: ExpectedFieldValue::Integer,
            })
        };

        let up = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: CollisionalRatesField::UpperLevel,
                expected: ExpectedFieldValue::Integer,
            })?
            .parse::<u32>();

        let up = match up {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: CollisionalRatesField::UpperLevel,
                value: String::from(values_beg.nth(CollisionalRatesField::UpperLevel as usize).unwrap()),
                expected: ExpectedFieldValue::Integer,
            })
        };

        let low = values
            .next()
            .ok_or(Self::Err::MissingField {
                field: CollisionalRatesField::LowerLevel,
                expected: ExpectedFieldValue::Integer,
            })?
            .parse::<u32>();

        let low = match low {
            Ok(n) => n,
            Err(_) => return Err(Self::Err::UnknownFormat {
                field: CollisionalRatesField::LowerLevel,
                value: String::from(values_beg.nth(CollisionalRatesField::LowerLevel as usize).unwrap()),
                expected: ExpectedFieldValue::Integer,
            })
        };

        let mut rates: Vec<f64> = vec!();
        for i in values {
            let item = match i.parse::<f64>() {
                Ok(n) => n,
                Err(_) => return Err(Self::Err::UnknownFormat {
                    field: CollisionalRatesField::RateCoefficients,
                    value: String::from(i),
                    expected: ExpectedFieldValue::Float,
                })
            };

            rates.push(item);
        }

        Ok(Self {
            transition,
            up,
            low,
            rates
        })
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_comment() {
        let s = "! Comment      ";
        let expected = Ok(Comment(String::from("Comment")));

        assert_eq!(
            s.parse::<Comment>(),
            expected,
            "Wrong result for default comment string `{}`",
            s
        );
    }

    #[test]
    fn parse_u32() {
        let s = "  65 ";
        let expected_number_of_energy_levels = Ok(NumberOfEnergyLevels(65));
        let expected_number_of_radiative_transitions = Ok(NumberOfRadiativeTransitions(65));
        let expected_number_of_collision_partners = Ok(NumberOfCollisionPartners(65));
        let expected_number_of_collisional_transitions = Ok(NumberOfCollisionalTransitions(65));
        let expected_number_of_collisional_temperatures = Ok(NumberOfCollisionalTemperatures(65));

        assert_eq!(
            s.parse::<NumberOfEnergyLevels>(),
            expected_number_of_energy_levels,
            "Cannot parse u32 value from string `{}`",
            s
        );
        assert_eq!(
            s.parse::<NumberOfRadiativeTransitions>(),
            expected_number_of_radiative_transitions,
            "Cannot parse u32 value from string `{}`",
            s
        );
        assert_eq!(
            s.parse::<NumberOfCollisionPartners>(),
            expected_number_of_collision_partners,
            "Cannot parse u32 value from string `{}`",
            s
        );
        assert_eq!(
            s.parse::<NumberOfCollisionalTransitions>(),
            expected_number_of_collisional_transitions,
            "Cannot parse u32 value from string `{}`",
            s
        );
        assert_eq!(
            s.parse::<NumberOfCollisionalTemperatures>(),
            expected_number_of_collisional_temperatures,
            "Cannot parse u32 value from string `{}`",
            s
        );
    }

    #[test]
    fn parse_element_name() {
        let s = "  TEST ! Additional information  ";
        let expected = Ok(ElementName {
            name: String::from("TEST"),
            information: String::from("Additional information"),
        });

        assert_eq!(
            s.parse::<ElementName>(),
            expected,
            "Wrong result for default format string `{}`",
            s
        );
    }

    #[test]
    fn parse_element_weight() {
        let s = " 32.3  ";
        let expected = Ok(ElementWeight(32.3));

        assert_eq!(
            s.parse::<ElementWeight>(),
            expected,
            "Cannot parse f32 value from string `{}`",
            s
        );
    }

    #[test]
    fn parse_energy_level() {
        let s = "   32  32.4    1e-12   ! ' 3 5 6'";
        let expected = Ok(EnergyLevel {
            level: 32,
            energy: 32.4,
            stat_weight: 1e-12,
            qnums: String::from("3 5 6")
        });

        assert_eq!(
            s.parse::<EnergyLevel>(),
            expected,
            "Wrong result for default format string `{}`",
            s
        );
    }

    #[test]
    fn parse_radiative_transition() {
        let s = "  45 32 9  1e-14     345.32    Additional";
        let expected = Ok(RadiativeTransition {
            transition: 45,
            up: 32,
            low: 9,
            aeinst: 1e-14,
            extra: String::from("345.32 Additional"),
        });

        assert_eq!(
            s.parse::<RadiativeTransition>(),
            expected,
            "Wrong result for default format string `{}`",
            s
        );
    }

    #[test]
    fn parse_collision_partner_name() {
        let s = "2 ! Additional info ";
        let expected = Ok(CollisionPartnerName {
            name: CollisionPartnerId::pH2,
            information: String::from("Additional info")
        });

        assert_eq!(
            s.parse::<CollisionPartnerName>(),
            expected,
            "Wrong result for default format string `{}`",
            s
        );
    }

    #[test]
    fn parse_collisional_temperatures() {
        let s = "10.1 20.2  30.3     40.4   ";
        let expected = Ok(CollisionalTemperatures(vec!(10.1, 20.2, 30.3, 40.4)));

        assert_eq!(
            s.parse::<CollisionalTemperatures>(),
            expected,
            "Wrong result for default format string `{}`",
            s
        );
    }

    #[test]
    fn parse_collisional_rates() {
        let s = "65 42 13    12e-12 13e-13 14e-14";
        let expected = Ok(CollisionalRates {
            transition: 65,
            up: 42,
            low: 13,
            rates: vec!(12e-12, 13e-13, 14e-14),
        });

        assert_eq!(
            s.parse::<CollisionalRates>(),
            expected,
            "Wrong result for default format string `{}`",
            s
        );
    }

    #[test]
    fn parse_lamda_file_contents() -> Result<(), ParseError> {
        let s = r#"!MOLECULE
        O (neutral atom)
        !MOLECULAR WEIGHT
        16.0
        !NUMBER OF ENERGY LEVELS
        3
        !LEVEL + ENERGIES(cm^-1) + WEIGHT + Qnum
           1    0.000000000   5.0  3_P_2  ! 2S+1  L  J = 3 P 2
           2  158.2687410     3.0  3_P_1  ! 2S+1  L  J = 3 P 1
           3  226.9852492     1.0  3_P_0  ! 2S+1  L  J = 3 P 0
        !NUMBER OF RADIATIVE TRANSITIONS
        3
        !TRANS + UP + LOW + EINSTEINA(s^-1) + FREQ(GHz) + E_u(K)
            1     2     1   8.910E-05  4744.77749   227.712
            2     3     1   1.340E-10  6804.84658   326.579
            3     3     2   1.750E-05  2060.06909   326.579
        !NUMBER OF COLL PARTNERS
        6
        !COLLISIONS BETWEEN
        5 O + H  ! Lique et al. 2018, MNRAS 474, 2313 corrected, transmitted by M. Wolfire. T-points selected by E. Roueff
        !NUMBER OF COLL TRANS
        3
        !NUMBER OF COLL TEMPS
        19
        !COLL TEMPS
           10.000      20.000      30.000      40.000      60.000      80.000      110.00      160.00      220.00      320.00      450.00      630.00      890.00      1260.0      1780.0      2510.0      3550.0      5010.0      8000.0
        !TRANS + UP + LOW + COLLRATES(cm^3 s^-1)
            1     2     1   7.0204e-11  8.2028e-11  9.0584e-11  9.8459e-11  1.1421e-10  1.3039e-10  1.5488e-10  1.9425e-10  2.3747e-10  2.9974e-10  3.6597e-10  4.3801e-10  5.1576e-10  5.9551e-10  6.7682e-10  7.6338e-10  8.6209e-10  9.7867e-10  1.1762e-09
            2     3     1   7.3118e-11  6.9519e-11  7.1053e-11  7.4232e-11  8.2569e-11  9.2191e-11  1.0783e-10  1.3515e-10  1.6763e-10  2.1768e-10  2.7340e-10  3.3530e-10  4.0291e-10  4.7408e-10  5.5063e-10  6.3692e-10  7.3695e-10  8.4933e-10  1.0160e-09
            3     3     2   1.2258e-10  1.1282e-10  1.1049e-10  1.1007e-10  1.1069e-10  1.1194e-10  1.1472e-10  1.2189e-10  1.3383e-10  1.5806e-10  1.9211e-10  2.3911e-10  3.0442e-10  3.9226e-10  5.0606e-10  6.4819e-10  8.2175e-10  1.0238e-09  1.3390e-09
        !COLLISIONS BETWEEN
        6 O + He  ! Lique et al. 2018, MNRAS 474, 2313 corrected, transmitted by M. Wolfire. T-points selected by E. Roueff
        !NUMBER OF COLL TRANS
        3
        !NUMBER OF COLL TEMPS
        19
        !COLL TEMPS
           10.000      20.000      30.000      40.000      60.000      80.000      110.00      160.00      220.00      320.00      450.00      630.00      890.00      1260.0      1780.0      2510.0      3550.0      5010.0      8000.0
        !TRANS + UP + LOW + COLLRATES(cm^3 s^-1)
            1     2     1   1.6482e-11  1.8573e-11  2.1463e-11  2.4598e-11  3.0966e-11  3.7104e-11  4.5611e-11  5.7945e-11  7.0394e-11  8.7752e-11  1.0736e-10  1.3238e-10  1.6591e-10  2.0812e-10  2.5733e-10  3.1253e-10  3.7520e-10  4.4631e-10  5.5849e-10
            2     3     1   2.7998e-11  2.9454e-11  3.3389e-11  3.8068e-11  4.8082e-11  5.8091e-11  7.2279e-11  9.3025e-11  1.1352e-10  1.4031e-10  1.6777e-10  2.0034e-10  2.4300e-10  2.9705e-10  3.6009e-10  4.2922e-10  5.0444e-10  5.8567e-10  7.0673e-10
            3     3     2   1.0152e-13  1.6305e-13  2.5139e-13  3.6504e-13  6.6603e-13  1.0595e-12  1.8033e-12  3.3720e-12  5.6364e-12  9.9193e-12  1.5822e-11  2.3904e-11  3.4853e-11  4.9010e-11  6.6911e-11  8.9450e-11  1.1803e-10  1.5344e-10  2.1404e-10
        !COLLISIONS BETWEEN
        2 O + p-H2  ! Lique et al. 2018, MNRAS 474, 2313 corrected, transmitted by M. Wolfire. T-points selected by E. Roueff
        !NUMBER OF COLL TRANS
        3
        !NUMBER OF COLL TEMPS
        19
        !COLL TEMPS
           10.000      20.000      30.000      40.000      60.000      80.000      110.00      160.00      220.00      320.00      450.00      630.00      890.00      1260.0      1780.0      2510.0      3550.0      5010.0      8000.0
        !TRANS + UP + LOW + COLLRATES(cm^3 s^-1)
            1     2     1   1.1818e-10  1.2795e-10  1.3314e-10  1.3766e-10  1.4621e-10  1.5405e-10  1.6409e-10  1.7678e-10  1.8771e-10  2.0139e-10  2.1690e-10  2.3769e-10  2.6604e-10  3.0174e-10  3.4447e-10  3.9528e-10  4.5692e-10  5.2977e-10  6.3237e-10
            2     3     1   8.1964e-11  1.0202e-10  1.1233e-10  1.2001e-10  1.3180e-10  1.4038e-10  1.4902e-10  1.5630e-10  1.5920e-10  1.6019e-10  1.6294e-10  1.7246e-10  1.9279e-10  2.2497e-10  2.6829e-10  3.2276e-10  3.9034e-10  4.7055e-10  5.8402e-10
            3     3     2   7.9957e-14  1.5935e-13  2.6308e-13  3.9685e-13  6.6622e-13  8.6942e-13  1.0522e-12  1.1780e-12  1.2352e-12  1.3677e-12  1.7490e-12  2.6158e-12  4.2863e-12  7.0486e-12  1.1151e-11  1.6878e-11  2.4697e-11  3.4836e-11  5.0910e-11
        !COLLISIONS BETWEEN
        3 O + o-H2  ! Lique et al. 2018, MNRAS 474, 2313 corrected, transmitted by M. Wolfire. T-points selected by E. Roueff
        !NUMBER OF COLL TRANS
        3
        !NUMBER OF COLL TEMPS
        19
        !COLL TEMPS
           10.000      20.000      30.000      40.000      60.000      80.000      110.00      160.00      220.00      320.00      450.00      630.00      890.00      1260.0      1780.0      2510.0      3550.0      5010.0      8000.0
        !TRANS + UP + LOW + COLLRATES(cm^3 s^-1)
            1     2     1   1.3258e-10  1.3972e-10  1.4475e-10  1.4972e-10  1.5993e-10  1.7007e-10  1.8457e-10  2.0635e-10  2.2904e-10  2.6107e-10  2.9598e-10  3.3664e-10  3.8519e-10  4.4088e-10  5.0314e-10  5.7270e-10  6.5175e-10  7.3905e-10  8.5173e-10
            2     3     1   6.5072e-11  7.6028e-11  8.2408e-11  8.7995e-11  9.8545e-11  1.0856e-10  1.2254e-10  1.4300e-10  1.6361e-10  1.9165e-10  2.2195e-10  2.5888e-10  3.0675e-10  3.6650e-10  4.3772e-10  5.2065e-10  6.1746e-10  7.2668e-10  8.7341e-10
            3     3     2   2.6483e-12  3.0795e-12  3.3059e-12  3.4907e-12  3.8261e-12  4.1442e-12  4.6053e-12  5.3504e-12  6.2198e-12  7.6176e-12  9.3446e-12  1.1600e-11  1.4670e-11  1.8803e-11  2.4297e-11  3.1535e-11  4.1039e-11  5.2862e-11  7.0467e-11
        !COLLISIONS BETWEEN
        7 O + H+  !  computed from xsections of Spirko et al. J. Phys B 36, 1645, 2003 by E. Roueff, oct 2019 at same temperatures
        !NUMBER OF COLL TRANS
        3
        !NUMBER OF COLL TEMPS
        19
        !COLL TEMPS
           10.000      20.000      30.000      40.000      60.000      80.000      110.00      160.00      220.00      320.00      450.00      630.00      890.00      1260.0      1780.0      2510.0      3550.0      5010.0      8000.0
        !TRANS + UP + LOW + COLLRATES(cm^3 s^-1)
            1     2     1   2.4006e-11	4.4688e-11	6.4277e-11	8.3187e-11	1.1965e-10	1.5485e-10	2.0602e-10	2.8826e-10	3.8350e-10	5.3660e-10	7.2842e-10	9.8488e-10	1.3424e-09	1.8334e-09	2.4990e-09	3.4007e-09	4.6401e-09	6.3190e-09	9.6131e-09
            2     3     1   4.1218e-12	8.8188e-12	1.3761e-11	1.8868e-11	2.9441e-11	4.0369e-11	5.7255e-11	8.6371e-11	1.2250e-10	1.8480e-10	2.6863e-10	3.8860e-10	5.6775e-10	8.3143e-10	1.2147e-09	1.7711e-09	2.5909e-09	3.7811e-09	6.3190e-09
            3     3     2   1.7356e-10	2.2838e-10	2.6815e-10	3.0050e-10	3.5283e-10	3.9540e-10	4.4854e-10	5.2028e-10	5.9020e-10	6.8459e-10	7.8354e-10	8.9520e-10	1.0264e-09	1.1779e-09	1.3506e-09	1.5475e-09	1.7752e-09	2.0346e-09	2.4488e-09
        !COLLISIONS BETWEEN
        4 O + e  ! Bell et al. 1998, MNRAS, 293, L83
        !NUMBER OF COLL TRANS
        3
        !NUMBER OF COLL TEMPS
        5
        !COLL TEMPS
        50.0 100.0 500.0 1000. 3000.
        !TRANS + UP + LOW + COLLRATES(cm^3 s^-1)
            1     2     1   3.4E-10  3.6E-10  3.3E-10  3.1E-10  3.1E-10
            2     3     1   3.9E-10  4.3E-10  4.3E-10  4.1E-10  4.2E-10
            3     3     2   3.3E-13  7.7E-13  4.1E-12  6.5E-12  1.1E-11
        !NOTES
        ! A-values are from the NIST database.
        ! Accurate transition frequencies measured by Zink et al. 1991, ApJ 371, L85.
        ! Transition frequencies for the 17O and 18O isotopes can be found
        ! in Brown, Evenson, Zink (1993, Phys. Rev. A, 48, 3761) and in
        ! DeNatale et al. (1993, Phys. Rev. A, 48, 3757). The latter reference
        ! presents more precise values.
        "#;

        let result = s.parse::<ElementData>();

        match result {
            Ok(ed) => {
                assert_eq!(ed.radiative_transitions.len(), 3);
                assert_eq!(ed.collision_partners.len(), 6);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
}
