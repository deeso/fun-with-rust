#[deriving(Show)]
pub enum Error {
	FailedToOpenFile,
	FailedToReadFile,
	FailedToExtractContent,
}
