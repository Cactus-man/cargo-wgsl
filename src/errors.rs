use naga::{front::wgsl::ParseError, valid::ValidationError, WithSpan};

#[derive(Debug)]
pub enum WgslError {
    Validate {
        src: String,
        error: WithSpan<ValidationError>,
        emitted: String,
    },
    Parse {
        error: String,
        line: usize,
        pos: usize,
    },
    Io(std::io::Error),
}

impl From<std::io::Error> for WgslError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl WgslError {
    pub fn from_parse_err(err: ParseError, src: &str) -> Self {
        let error = err.emit_to_string(src);
        let loc = err.location(src);
        if let Some(loc) = loc {
            Self::Parse {
                error,
                line: loc.line_number as usize,
                pos: loc.line_position as usize,
            }
        } else {
            Self::Parse {
                error,
                line: 0,
                pos: 0,
            }
        }
    }
}
