use std::error::Error;
use std::fmt;
/* NOTE: どんなエラーがあるかな？
 * コンバート失敗した
 * 生成失敗した
 * 指輪の直径が無効な値だった
*/

#[derive(Debug)]
pub struct ConvertError {
    pub cause: String,
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConvertError: was cause by {}", self.cause)
    }
}

impl Error for ConvertError {}

#[cfg(test)]
mod tests {
    use crate::error::ConvertError;

    #[test]
    fn display_error() {
        let error = ConvertError {
            cause: String::from("missing RingDefinition"),
        };
        assert_eq!(
            format!("{}", error),
            "ConvertError: was cause by missing RingDefinition".to_owned()
        )
    }
}
