use rusty_pdf::color::RGB;
use rusty_pdf::drawing_commands::DrawingCommands;
use rusty_pdf::util::StrokeOrFill::Fill;
use rusty_pdf::util::{Dims, Posn, WindingRule};
use rusty_pdf::{PageSize, Pdf, PdfError};

fn main() -> Result<(), PdfError> {
    println!("rusty_pdf - PDF library for Rust");
    println!("Originally based on Python rusty_pdf\n");

    let mut pdf = Pdf::new()?.with_default_page_size(PageSize::A4);

    let mut cmd = DrawingCommands::new();
    cmd.set_color_rgb(RGB::BLUE, Fill);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );
    cmd.fill(WindingRule::EvenOdd);

    cmd.set_color_rgb(RGB::RED, Fill);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 100.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );

    cmd.fill(WindingRule::EvenOdd);

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_color_rgb(RGB::BLUE, Fill);
    cmd.set_text_position(Posn { x: 50.0, y: 250.0 });
    cmd.show_single_text_string("Hello, Blue World");
    cmd.end_text();

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_color_rgb(RGB::RED, Fill);
    cmd.set_text_position(Posn { x: 50.0, y: 200.0 });
    cmd.show_single_text_string("Hello, RED World");
    cmd.end_text();

    let data = cmd.flush();

    let root_tree = pdf.page_ops.root_tree();

    root_tree.add_page_using(data.clone())?;

    root_tree.add_page_using(data)?;

    cmd.set_color_rgb(RGB::ORANGE, Fill);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );
    cmd.fill(WindingRule::EvenOdd);

    cmd.set_color_rgb(RGB::GREEN, Fill);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 100.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );

    cmd.fill(WindingRule::EvenOdd);
    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_color_rgb(RGB::RED, Fill);
    cmd.set_text_position(Posn { x: 50.0, y: 200.0 });
    cmd.show_single_text_string("Page 3, A5");
    cmd.end_text();

    let data = cmd.flush();
    let mut new_tree = root_tree.make_tree()?.with_default_page_size(PageSize::A5);
    new_tree.add_page_using(data)?;

    root_tree.add_tree(new_tree)?;

    let path = "output.pdf";
    pdf.finalise(path)?;

    println!("Created {path}:\n\n{}", std::fs::read_to_string(path)?);

    Ok(())
}
