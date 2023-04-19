//! CSV parsing for the chaperone.

use crate::transform::{FlatData, Rules, Value, ValueType};
use csv::StringRecord;
use error::{Error, Result};
use std::{borrow::Cow, io::Cursor};

impl FlatData<'_> {
    /// Creates `FlatData` from CSV content buffer.
    ///
    /// Annoyingly, we have to clone the data because the CsvReader doesn't propagate lifetimes
    /// properly.
    pub fn from_csv<'a>(data: &'a [u8], rules: &Rules) -> Result<FlatData<'a>> {
        let mut reader = csv::Reader::from_reader(Cursor::new(data));
        let headers =
            reader.headers()?.iter().map(ToString::to_string).map(Cow::from).collect::<Vec<_>>();
        let records = reader
            .records()
            .map(|record| Self::make_flat_data_record(record?, &headers, rules))
            .collect::<Result<Vec<_>>>()?;
        Ok(FlatData { keys: headers, records })
    }

    /// Converts the typeless CSV record into semi-typed `FlatData` according ot the type
    /// mapping in the `Rules`.
    fn make_flat_data_record(
        record: StringRecord,
        headers: &[Cow<'_, str>],
        rules: &Rules,
    ) -> Result<Vec<Value<'static>>> {
        record
            .into_iter()
            .enumerate()
            .map(|(index, value)| {
                let maybe_header = headers.get(index);
                let value_type = maybe_header
                    .zip(rules.type_mapping.as_ref())
                    .and_then(|(key, map)| map.get(&**key));
                Ok(match value_type {
                    Some(ValueType::Number) => {
                        let value = if value.is_empty() {
                            0.0
                        } else {
                            value.parse::<f64>().map_err(|e| {
                                Error::invalid_argument(format!(
                                    "Failed to parse record as number for {maybe_header:?} -> {value:?}: {e:?}"
                                ))
                            })?
                        };
                        Value::Number(value)
                    }
                    _ => Value::text_owned(value),
                })
            })
            .collect()
    }
}
