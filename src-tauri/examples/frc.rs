use kiwi_app_lib::{
    app,
    commands::frontend::frame::find_relative_colors,
    types::{ColoredPoint, Point, RelativeColoredPoint, RgbOffset},
};
use std::{thread, time::Duration};
fn main() {
    let app = app::get();

    // run capturer
    {
        let app_capturer = app.clone();
        thread::spawn(move || {
            app_capturer.with_capturer(|engine| {
                engine.clear_frame();
                let _ = engine.start_background();
            });
        });
    }

    // ensure frame is ok.
    loop {
        if app.get_frame_arc().is_ok() {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }

    // find relative colors
    loop {
        let origin = app.get_frame().unwrap().to_base64_png().unwrap();
        let points = vec![
            RelativeColoredPoint::new(
                ColoredPoint::new(Point::new(19, 9), "#b65d2f".to_string()),
                Point::new(0, 0),
            ),
            RelativeColoredPoint::new(
                ColoredPoint::new(Point::new(20, 12), "#572516".to_string()),
                Point::new(1, 3),
            ),
            RelativeColoredPoint::new(
                ColoredPoint::new(Point::new(23, 20), "#e68a31".to_string()),
                Point::new(4, 11),
            ),
            RelativeColoredPoint::new(
                ColoredPoint::new(Point::new(24, 31), "#944323".to_string()),
                Point::new(5, 22),
            ),
            RelativeColoredPoint::new(
                ColoredPoint::new(Point::new(29, 30), "#7e4017".to_string()),
                Point::new(10, 21),
            ),
            RelativeColoredPoint::new(
                ColoredPoint::new(Point::new(32, 22), "#743826".to_string()),
                Point::new(13, 13),
            ),
            RelativeColoredPoint::new(
                ColoredPoint::new(Point::new(44, 42), "#eba94d".to_string()),
                Point::new(25, 33),
            ),
        ];
        let start_point = Point::new(0, 0);
        let end_point = Point::new(1920, 1080);
        let offset_value = 10;
        let rgb_offset = RgbOffset::new(offset_value, offset_value, offset_value);
        match find_relative_colors(origin, points, start_point, end_point, rgb_offset) {
            Ok(result) => match result {
                Some(colored_point) => {
                    println!(
                        "vertex point : ({}, {}), hex: {}",
                        colored_point.point.x, colored_point.point.y, colored_point.hex
                    )
                }
                None => {
                    println!("Not matched.")
                }
            },
            Err(e) => {
                println!("error: {:#?}", e);
            }
        }
    }
}

// client.find_relative_colors(
//     points=[
//         RelativeColoredPoint(colored_point=ColoredPoint(point=Point(x=19, y=9), hex="#b65d2f"), relative_point=Point(x=0, y=0)),
//         RelativeColoredPoint(colored_point=ColoredPoint(point=Point(x=20, y=12), hex="#572516"), relative_point=Point(x=1, y=3)),
//         RelativeColoredPoint(colored_point=ColoredPoint(point=Point(x=23, y=20), hex="#e68a31"), relative_point=Point(x=4, y=11)),
//         RelativeColoredPoint(colored_point=ColoredPoint(point=Point(x=24, y=31), hex="#944323"), relative_point=Point(x=5, y=22)),
//         RelativeColoredPoint(colored_point=ColoredPoint(point=Point(x=29, y=30), hex="#7e4017"), relative_point=Point(x=10, y=21)),
//         RelativeColoredPoint(colored_point=ColoredPoint(point=Point(x=32, y=22), hex="#743826"), relative_point=Point(x=13, y=13)),
//         RelativeColoredPoint(colored_point=ColoredPoint(point=Point(x=44, y=42), hex="#eba94d"), relative_point=Point(x=25, y=33)),
//     ],
//     start_point=Point(x=0,y=0),
//     end_point=Point(x=1920,y=1080),
//     rgb_offset=RgbOffset(r=2, g=2, b=2),
// )
