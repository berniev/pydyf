use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{EvenOdd, StrokeOrFill};
use pydyf::page::PageSize;
use pydyf::util::{Dims, Posn};
use pydyf::{PDF, PageObject, PdfObject, StreamObject};

#[test]
fn test_create_pdf() {
    let pdf = PDF::new();
    assert_eq!(pdf.objects.len(), 1);
}

#[test]
fn test_add_page() {
    let mut pdf = PDF::new();
    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));

    let next_num = pdf.objects.len() - 1;
    let mut page = PageObject::new(next_num, 0);
    page.set_media_box(PageSize::A4);

    pdf.add_page(page);

    assert!(pdf.objects.len() > 1);
}

#[test]
fn test_stream_operations() {
    let mut stream = StreamObject::compressed();

    let color = RGB {
        red: Color { color: 1.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 0.0 },
    };
    let _ = stream.set_color_rgb(color, StrokeOrFill::Stroke);
    stream.rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            height: 200.0,
            width: 150.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    assert!(stream.stream.len() > 0);
}

#[test]
fn test_compressed_stream() {
    let stream = StreamObject::compressed();
    assert!(stream.compress);
}

/*#[test]
fn test_text_operations() {
    let mut stream = StreamObject::new();

    stream.begin_text();
    stream.set_font_size("Helvetica", 12.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 100.0, 700.0);
    stream.show_text_string("Test");
    assert!(stream.stream.len() > 0);
}*/

#[test]
fn test_add_page_with_pagesize_adds_mediabox() {
    let mut pdf = PDF::new();
    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));

    let next_num = pdf.objects.len() - 1;

    let mut page = PageObject::new(next_num, 0);
    page.set_media_box(PageSize::A4);

    // Should contain MediaBox because it was explicitly provided
    assert_eq!(page.media_box, Some(PageSize::A4));
}

#[test]
fn test_default_page_size() {
    let mut pdf = PDF::new();

    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));

    let next_num = pdf.objects.len() - 1;

    let page = PageObject::new(next_num, 0);
    pdf.add_page(page);

    let page_obj = pdf.objects.last().unwrap();
    let data = page_obj.data();
    let data_str = String::from_utf8_lossy(&data);

    // Should NOT contain MediaBox because it's inherited. todo: from where. how?
    assert!(!data_str.contains("/MediaBox"));
}

#[test]
fn test_root_mediabox_inheritance() {
    let pdf = PDF::new();
    let pages_tree = &pdf.page_tree;
    let mediabox = pages_tree.get("MediaBox").unwrap();
    assert_eq!(String::from_utf8_lossy(mediabox), "[0 0 595 842]");
}

#[test]
fn test_negative_pagesize_is_zeroed() {
    let size = PageSize::Custom(Dims {
        width: -100.0, // invalid width. should be made zero
        height: 500.0,
    });
    let dimensions = size.dimensions();
    assert_eq!(dimensions.width, 0.0);
    assert_eq!(dimensions.height, 500.0);
}

#[test]
fn test_pagesize_custom_validation() {
    let size = PageSize::Custom(Dims {
        width: 100.0, // invalid width should be made zero
        height: 500.0,
    });
    let (width,height):Dims = size.dimensions();
    assert_eq!(width, 0.0);
    assert_eq!(height, 500.0);
}
