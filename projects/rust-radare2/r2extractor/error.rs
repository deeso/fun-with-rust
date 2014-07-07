#[deriving(Show)]
pub enum Error {
	FailedToExtractContent,
}

#[deriving(Show)]
pub enum RCoreError {
	FailedToInitCore,
	FailedToOpenFile,
	FailedToLoadBin,
	FailedCommandExec,
}

