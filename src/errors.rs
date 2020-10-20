/*error_chain! {
	foreign_links {
		Fmt(::std::fmt::Error);
		Io(::std::io::Error);
		TomlDeser(::toml::de::Error);
		Tera(::tera::Error);
		WalkDir(::walkdir::Error);
		StripPrefix(::std::path::StripPrefixError);
	}
}*/

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BoilrError {
	#[error("Error cannot format {0:?}")]
	FormatDisplayError(#[from] std::fmt::Error),
	#[error("Error cannot read from {path:?}")]
	ReadError { source: std::io::Error, path: PathBuf },
	#[error("Error cannot copy from {from_path:?} {to_path:?}")]
	CopyError { source: std::io::Error, from_path: PathBuf, to_path: PathBuf },
	#[error("Error cannot delete {path:?}")]
	DeleteError { source: std::io::Error, path: PathBuf },
	#[error("Error while reading {path:?}")]
	TomlDeserializeError{ source: toml::de::Error, path: PathBuf },
	#[error("Error while parsing files using Tera")]
	TeraTemplateError(#[from] tera::Error),
	#[error("Error while parsing directories")]
	WalkDirError(#[from] walkdir::Error),
	#[error("Internal path stripping error")]
	StripPrefixError(#[from] std::path::StripPrefixError),
	#[error("Cannot convert to String")]
	StrError,
	#[error("Cannot access current directory")]
	AccessCurrentDirError,
	#[error("Error arg not found in clap args")]
	ArgNotFound,
	#[error("{path:?} is not a directory")]
	NotADirectory { path: PathBuf }
}
