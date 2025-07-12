use raylib::prelude::*;

fn draw_pixel(image: &mut Image, x: i32, y: i32, color: Color) {
        image.draw_pixel(x, y, color);
    
}

fn draw_line(image: &mut Image, mut x1: i32, mut y1: i32, x2: i32, y2: i32, color: Color) {
    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        draw_pixel(image, x1, y1, color);
        if x1 == x2 && y1 == y2 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x1 += sx;
        }
        if e2 <= dx {
            err += dx;
            y1 += sy;
        }
    }
}

fn draw_polygon(image: &mut Image, points: &[(i32, i32)], color: Color) {
    for i in 0..points.len() - 1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];
        draw_line(image, x1, y1, x2, y2, color);
    }
}

fn fill_polygon(image: &mut Image, points: &[(i32, i32)], color: Color) {
    let min_y = points.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap();
    
    for y in min_y..=max_y {
        let mut intersections = Vec::new();
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % points.len()];
            if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
                let x = x1 + (y - y1) * (x2 - x1) / (y2 - y1);
                intersections.push(x);
            }
        }
        intersections.sort();
        for i in 0..intersections.len() / 2 {
            let x_start = intersections[2 * i];
            let x_end = intersections[2 * i + 1];
            for x in x_start..=x_end {
                draw_pixel(image, x, y, color);
            }
        }
    }
}

fn main() {
    let screen_width = 1000;
    let screen_height = 1000;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Polígono con Raylib")
        .build();

    let mut image = Image::gen_image_color(screen_width, screen_height, Color::BLACK);

    let points = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
        (165, 380),
    ];

    draw_polygon(&mut image, &points, Color::WHITE);

    fill_polygon(&mut image, &points, Color::YELLOW);

    let points = vec![
        (321, 335), (288, 286), (339, 251), (374, 302), (321, 335),
    ];

    draw_polygon(&mut image, &points, Color::WHITE);
    fill_polygon(&mut image, &points, Color::BLUE);

    let points = vec![
       (377, 249), (411, 197), (436, 249), (377, 249),

    ];

    draw_polygon(&mut image, &points, Color::WHITE);
    fill_polygon(&mut image, &points, Color::RED);

    let points = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    draw_polygon(&mut image, &points, Color::WHITE);
    fill_polygon(&mut image, &points, Color::PURPLE);

    let points = vec![
        (682, 175), (708, 120), (735, 148), (739, 170), (682, 175),
    ];

    draw_polygon(&mut image, &points, Color::WHITE);
    fill_polygon(&mut image, &points, Color::BLACK);


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        
    }


    image.export_image("output.png");
}
