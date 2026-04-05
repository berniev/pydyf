use flate2::Compression;
use flate2::write::ZlibEncoder;
/// PDF content stream.
///
/// Content streams define page content, eg:
/// - Graphics: paths, rectangles, curves
/// - Text: fonts, positioning, display
/// - Colors: RGB, CMYK, grayscale
/// - Images: inline images
/// - Transformations: matrices, state management
///
///   A stream object, like a string object, is a sequence of bytes. Furthermore, a stream may be
///   of unlimited length, whereas a string shall be subject to an implementation limit. For this
///   reason, objects with potentially large amounts of data, such as images and page
///   descriptions, shall be represented as streams.
///
///   A stream shall consist of a dictionary followed by zero or more bytes bracketed between the
///   keywords'stream' and 'endstream'.
///
///   All streams shall be indirect objects and the stream dictionary shall be a direct object.
///
///   Beginning with PDF 1.5, indirect objects may reside in object streams (see 7.5.7, "Object
///   Streams"). They are referred to in the same way; however, their definition shall not
///   include the keywords obj and endobj, and their generation number shall be zero.
///
/// ```
/// Stream Extent: Entries common to all stream --dictionaries--:
/// ===========================================================================
/// Entry         Type     Reqd Description
/// ============  ==========  = ===============================================
/// Length        int         R The length of the stream in bytes
/// Filter        nam or arr  O A filter or sequence of filters to be applied
/// DecodeParms   dic or arr  O Parameters for the filter(s) in Filter
/// F             filespec    O A file specification for the stream data
/// FFilter       nam or arr  O A filter or sequence of filters to file data
/// FDecodeParms  dic or arr  O Parameters for the filter(s) in FFilter
/// DL            int         O Non-negative len of the decoded stream in bytes
/// ===========================================================================
/// ```
///   Filter:
///   An optional part of the specification of a stream object, indicating how the data in the
///   stream should be decoded before it is used
/// ```
/// Stream Filters:
/// =============================================================================
/// Name            P V Type    Decode/Decompress
/// =============== = = ======= =================================================
/// DCTDecode       y 5 image   Discrete Cosine Transform technique based on JPEG
/// JPXDecode       n 5 image   Wwavelet-based JPEG2000 standard
/// JBIG2Decode     y 4 image   JBig2 standard -> mono or approx
/// ASCIIHexDecode  n   binary  ASCII hex
/// ASCII85Decode   n   binary  ASCII base-85
/// LZWDecode       y   txt/bin LZE (Lempel-Ziv-Welch) algorithm
/// FlateDecode     y 2 txt/bin zlib/deflate compression
/// RunLengthDecode n   txt/bin byte-oriented run-length encoding algorithm
/// CCITTFaxDecode  y   image   CCITT facsimile standard. typ mono 1 bit/pixel
/// JBIG2Decode     y 4 image   JBig2 standard -> mono or approx
/// DCTDecode       y   image   Discrete Cosine Transform technique based on JPEG
/// JPXDecode       n 5 image   Wwavelet-based JPEG2000 standard
/// Crypt           y 5 data    Data encrypted by a security handler
/// =============================================================================
/// ```

use std::io::Write as IoWrite;

use crate::PdfDictionaryObject;
use crate::error::PdfError;
use crate::objects::pdf_object::PdfObj;
pub use crate::util::{CompressionMethod, Dims, Matrix, Posn, StrokeOrFill, ToPdf, WindingRule};

//------------------------ PdfStreamObject -----------------------

#[derive(Clone)]
pub struct PdfStreamObject {
    pub(crate) dict: PdfDictionaryObject,
    pub(crate) content: Vec<u8>,
    pub(crate) object_number: Option<u64>,

    pub(crate) compression_method: CompressionMethod,
}

impl Default for PdfStreamObject {
    fn default() -> Self {
        Self {
            dict: PdfDictionaryObject::new(),
            content: Vec::new(),
            object_number: None,

            compression_method: CompressionMethod::None,
        }
    }
}

impl PdfStreamObject {
    //-------------------------- Constructors --------------------------
    pub fn new() -> Self {
        Self {
            compression_method: CompressionMethod::None,
            ..Default::default()
        }
    }

    pub fn compressed(mut self) -> Self {
        self.compression_method = CompressionMethod::Flate;
        self.dict.add("Filter", PdfObj::name("FlateDecode"));
        self
    }

    pub fn with_data(mut self, stream: Vec<u8>, dict: PdfDictionaryObject) -> Self {
        self.content = stream;
        self.dict = dict;

        self
    }

    //----------------------------------------------------------------

    pub fn compression_method(&self) -> CompressionMethod {
        self.compression_method
    }

    pub fn add(&mut self, bytes: Vec<u8>) {
        self.content.extend(bytes);
    }

    pub fn serialise(&self) -> Result<Vec<u8>, PdfError> {
        let stream_bytes: Vec<u8> = match self.compression_method {
            CompressionMethod::None => self.content.clone(),
            CompressionMethod::Flate => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&self.content)?;
                encoder.finish()?
            }
        };

        let mut dict = self.dict.clone(); // else self must be mut, which it can't be
        dict.add("Length", stream_bytes.len() as f64);

        let mut vec = dict.serialise()?;

        vec.push(b'\n');
        vec.extend(b"stream\n");
        vec.extend(&stream_bytes);
        vec.extend(b"endstream\n");

        Ok(vec)
    }
}
