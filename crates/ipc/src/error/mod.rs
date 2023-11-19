use {
  serde::{
    ser::SerializeStruct,
    Serialize,
    Serializer,
  },
  std::io::ErrorKind,
};

#[derive(Debug)]
pub struct Io {
  kind: ErrorKind,
  code: Option<i32>,
  msg: String,
}

impl Serialize for Io {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("Io", 3)?;
    state.serialize_field("kind", &self.kind.to_string().replace(' ', "_"))?;
    state.serialize_field("code", &self.code)?;
    state.serialize_field("msg", &self.msg)?;
    state.end()
  }
}

#[derive(Debug, Serialize, Default)]
pub struct Unknown {
  msg: String,
}

impl<T: std::error::Error> From<T> for Unknown {
  fn from(value: T) -> Self {
    Self {
      msg: value.to_string(),
    }
  }
}

impl From<std::io::Error> for Io {
  fn from(value: std::io::Error) -> Self {
    let code = value.raw_os_error();

    Self {
      kind: value.kind(),
      msg: value.to_string(),
      code,
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IpcError {
  Io(Box<Io>),
  Unknown(Box<Unknown>),
}

impl<T: std::error::Error> From<T> for IpcError {
  default fn from(value: T) -> Self {
    Self::Unknown(Box::new(value.into()))
  }
}

impl From<std::io::Error> for IpcError {
  fn from(value: std::io::Error) -> Self {
    Self::Io(Box::new(value.into()))
  }
}
