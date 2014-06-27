#[deriving(Show)]
pub enum Error {
	FailedToExtractContent,
}

pub enum RCoreError {
	FailedToInitCore,
	FailedToOpenFile,
	FailedToLoadBin,
	FailedCommandExec,
}

