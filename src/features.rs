use crate::version::Version;

// list based on https://www.prepressure.com/pdf/basics/version
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Feature {
    Text,
    Images,
    Pages,
    HypertextLinks,
    Bookmarks,
    ThumbnailSketches,

    Rc4Encryption40bit,
    Links,
    Threads,
    Password,
    DeviceIndependentColor,
    BinaryFormat,

    Forms,
    Unicode,
    MultimediaFeatures,
    Opi13,
    CmykAndSpot,
    EmbeddedHalftoneFunctions,
    OverprintInstructions,
    TwoByteCidFonts,
    Opi20,
    AddtlColorSpaces,
    SmoothShading,
    Annotations,
    DigitalSignatures,
    JavaScriptActions,
    Rc4Encryption,
    Transparency,
    JavaScript15,
    TaggedPdf,
    Jbig2Compression,
    RC4Encryption128bit,
    ObjectStreams,
    JPEG2000Compression,
    EnhancedXrefTable,
    XRefStreams,
    XObjectStreams,
    Layers,
    ImprovedTaggedPdf,
    XmlFormsArchitectureXFA,
    AdditionalTransitions,
    NChannel,
    AesEncryption,
    EnhancedAnnotations,
    EnhancedTagging,
    OpenTypeFontDirectEmbedding,
    EmbeddedFiles,
    Embedded3dData,
    XmlForms,
    ImprovedCommenting,
    ImprovedSecurity,
    CommentsIn3dObjects,
    Enhanced3dAnimationControl,
    EmbeddedDefaultPrinterSettings,
}

impl Feature {
    #[rustfmt::skip]
    pub const fn min_version(self) -> Version {
        use Feature::*;
        use Version::*;
        match self {
            Text                           |
            Images                         |
            Pages                          |
            HypertextLinks                 |
            Bookmarks                      |
            ThumbnailSketches              => V1_0,

            Rc4Encryption40bit             |
            Links                          |
            Threads                        |
            Password                       |
            DeviceIndependentColor         |
            BinaryFormat                   => V1_1,

            Forms                          |
            Unicode                        |
            MultimediaFeatures             |
            Opi13                          |
            CmykAndSpot                    |
            EmbeddedHalftoneFunctions      |
            OverprintInstructions          => V1_2,

            TwoByteCidFonts                |
            Opi20                          |
            AddtlColorSpaces               |
            SmoothShading                  |
            Annotations                    |
            DigitalSignatures              |
            JavaScriptActions              |
            Rc4Encryption                  => V1_3,

            Transparency                   |
            JavaScript15                   |
            TaggedPdf                      |
            Jbig2Compression               |
            RC4Encryption128bit            => V1_4,

            ObjectStreams                  |
            JPEG2000Compression            |
            EnhancedXrefTable              |
            XRefStreams                    |
            XObjectStreams                 |
            Layers                         |
            ImprovedTaggedPdf              |
            XmlFormsArchitectureXFA        |
            AdditionalTransitions          => V1_5,

            NChannel                       |
            AesEncryption                  |
            EnhancedAnnotations            |
            EnhancedTagging                |
            OpenTypeFontDirectEmbedding    |
            EmbeddedFiles                  |
            Embedded3dData                 |
            XmlForms                       => V1_6,

            ImprovedCommenting             |
            ImprovedSecurity               |
            CommentsIn3dObjects            |
            Enhanced3dAnimationControl     |
            EmbeddedDefaultPrinterSettings => V1_7,
        }
    }
}
