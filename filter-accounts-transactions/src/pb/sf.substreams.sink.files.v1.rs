// @generated
/// Lines represents an ordered list of lines that have been extracted of a single block. You are
/// free the format each line as you please, the `substream-sink-files` tool does not make any
/// assumption about the content and simply write the content to the current bundle with a trailing
/// new line.
///
/// It is expected that your line do **not** contain a new line character as it will be managed
/// manually by the `substream-sink-files` tool.
///
/// The most common use case is to use CSV or JSONL format for your line. For example, you can
/// extract each transaction out of the block as a single line in JSON format as an object. The
/// `substream-sink-files` will then package them in a bundle for N blocks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lines {
    #[prost(string, repeated, tag="1")]
    pub lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// @@protoc_insertion_point(module)
