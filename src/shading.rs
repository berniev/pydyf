use crate::{PdfArrayObject, PdfDictionaryObject, PdfObject};
//--------------------------- ShadingType ----------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadingType {
    Function = 1,
    Axial = 2,
    Radial = 3,
    FreeFormGouraud = 4,
    LatticeGouraud = 5,
    CoonsPatch = 6,
    TensorPatch = 7,
}

//--------------------------- ShadingBase ----------------------//

pub trait ShadingBase {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject;

    fn with_background(mut self, background: PdfObject) -> Self
    where
        Self: Sized,
    {
        self.dict_mut().add("Background", background);

        self
    }

    fn with_bbox(mut self, bbox: PdfObject) -> Self
    where
        Self: Sized,
    {
        self.dict_mut().add("BBox", bbox);

        self
    }

    fn with_anti_alias(mut self, value: bool) -> Self
    where
        Self: Sized,
    {
        self.dict_mut().add("AntiAlias", value);

        self
    }
}

//--------------------------- builder ----------------------//

fn make_shading(color_space: PdfObject, shading_type: ShadingType) -> PdfDictionaryObject {
    let mut dict = PdfDictionaryObject::new();
    dict.add("ShadingType", shading_type as i64);
    dict.add("ColorSpace", color_space);

    dict
}

//--------------------------- FunctionShading (1) ----------------------//

pub struct Shading1Function {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading1Function {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading1Function {
    pub fn new(color_space: PdfObject, function: PdfDictionaryObject) -> Self {
        let mut dictionary = make_shading(color_space, ShadingType::Function);
        dictionary.add("Function", function);

        Self { dictionary }
    }

    pub fn with_domain(mut self, domain: PdfArrayObject) -> Self {
        self.dictionary.add("Domain", domain);

        self
    }

    pub fn with_matrix(mut self, matrix: PdfArrayObject) -> Self {
        self.dictionary.add("Matrix", matrix);

        self
    }
}

//--------------------------- AxialShading (2) ----------------------//

pub struct Shading2Axial {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading2Axial {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading2Axial {
    pub fn new(
        color_space: PdfObject,
        coords: PdfArrayObject,
        function: PdfDictionaryObject,
    ) -> Self {
        let mut dictionary = make_shading(color_space, ShadingType::Axial);
        dictionary.add("Coords", coords);
        dictionary.add("Function", function);

        Self { dictionary }
    }

    pub fn with_domain(mut self, domain: PdfArrayObject) -> Self {
        self.dictionary.add("Domain", domain);

        self
    }

    pub fn with_extend(mut self, extend: PdfArrayObject) -> Self {
        self.dictionary.add("Extend", extend);

        self
    }
}

//--------------------------- RadialShading (3) ----------------------//

pub struct Shading3Radial {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading3Radial {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading3Radial {
    pub fn new(color_space: PdfObject, function: PdfDictionaryObject) -> Self {
        let mut dictionary = make_shading(color_space, ShadingType::Radial);
        dictionary.add("Function", function);

        Self { dictionary }
    }

    pub fn with_domain(mut self, domain: PdfArrayObject) -> Self {
        self.dictionary.add("Domain", domain);

        self
    }

    pub fn with_extend(mut self, extend: PdfArrayObject) -> Self {
        self.dictionary.add("Extend", extend);

        self
    }
}

//--------------------------- FreeFormGouraudShading (4) ----------------------//

pub struct Shading4FreeFormGouraud {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading4FreeFormGouraud {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading4FreeFormGouraud {
    pub fn new(
        color_space: PdfObject,
        bits_per_coordinate: u64,
        bits_per_component: u64,
        bits_per_flag: u64,
        decode: PdfArrayObject,
    ) -> Self {
        let mut dictionary = make_shading(color_space, ShadingType::FreeFormGouraud);
        dictionary.add("BitsPerCordinate", bits_per_component);
        dictionary.add("BitsPerComponent", bits_per_coordinate);
        dictionary.add("BitsPerFlag", bits_per_flag);
        dictionary.add("Decode", decode);

        Self { dictionary }
    }

    pub fn with(mut self, function: PdfDictionaryObject) -> Self {
        self.dictionary.add("Function", function);

        self
    }
}

//--------------------------- LatticeGouraudShading (5) ----------------------//

pub struct Shading5LatticeGouraud {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading5LatticeGouraud {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading5LatticeGouraud {
    pub fn new(
        color_space: PdfObject,
        bits_per_coordinate: u64,
        bits_per_component: u64,
        vertices_per_row: u64,
        decode: PdfArrayObject,
    ) -> Self {
        let mut dictionary = make_shading(color_space, ShadingType::LatticeGouraud);
        dictionary.add("BitsPerCordinate", bits_per_component);
        dictionary.add("BitsPerComponent", bits_per_coordinate);
        dictionary.add("VerticesPerRow", vertices_per_row);
        dictionary.add("Decode", decode);

        Self { dictionary }
    }

    pub fn with(mut self, function: PdfDictionaryObject) -> Self {
        self.dictionary.add("Function", function);

        self
    }
}

//--------------------------- patch shading ----------------------//

fn make_patch_shading(
    color_space: PdfObject,
    shading_type: ShadingType,
    bits_per_coordinate: i64,
    bits_per_component: i64,
    bits_per_flag: i64,
) -> PdfDictionaryObject {
    let mut dictionary = make_shading(color_space, shading_type);
    dictionary.add("BitsPerCoordinate", bits_per_coordinate);
    dictionary.add("BitsPerComponent", bits_per_component);
    dictionary.add("BitsPerFlag", bits_per_flag);

    dictionary
}

//-------------------- CoonsPatchShading (6) -----------------------------------//

pub struct Shading6CoonsPatch {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading6CoonsPatch {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading6CoonsPatch {
    pub fn new(
        color_space: PdfObject,
        bits_per_coordinate: i64,
        bits_per_component: i64,
        bits_per_flag: i64,
    ) -> Self {
        Self {
            dictionary: make_patch_shading(
                color_space,
                ShadingType::CoonsPatch,
                bits_per_coordinate,
                bits_per_component,
                bits_per_flag,
            ),
        }
    }

    pub fn with_decode(mut self, decode: PdfArrayObject) -> Self {
        self.dictionary.add("Decode", decode);

        self
    }

    pub fn with_function(mut self, function: PdfDictionaryObject) -> Self {
        self.dictionary.add("Function", function);

        self
    }
}

//-------------------- TensorPatchShading (7) -----------------------------------//

pub struct Shading7TensorPatch {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading7TensorPatch {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading7TensorPatch {
    pub fn new(
        color_space: PdfObject,
        bits_per_coordinate: i64,
        bits_per_component: i64,
        bits_per_flag: i64,
    ) -> Self {
        Self {
            dictionary: make_patch_shading(
                color_space,
                ShadingType::TensorPatch,
                bits_per_coordinate,
                bits_per_component,
                bits_per_flag,
            ),
        }
    }

    pub fn with_decode(mut self, decode: PdfArrayObject) -> Self {
        self.dictionary.add("Decode", decode);

        self
    }

    pub fn with_function(mut self, function: PdfDictionaryObject) -> Self {
        self.dictionary.add("Function", function);

        self
    }
}
