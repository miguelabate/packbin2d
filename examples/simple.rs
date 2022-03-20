use packbin2d::{Box2D, pack_bin};

fn main() {
    let mut boxes = vec![Box2D::new(0.,0.,100.,34.,"1")
                                    ,Box2D::new(0.,0.,34.,23.,"2")
                                    ,Box2D::new(0.,0.,250.,123.,"3")
                                    ,Box2D::new(0.,0.,67.,100.,"4")
                                    ,Box2D::new(0.,0.,125.,68.,"5")
                                    ,Box2D::new(0.,0.,10.,8.,"6")
                                    ,Box2D::new(0.,0.,30.,60.,"7")];
    let (width, height) = pack_bin(&mut boxes);
    println!("width tot:{} height tot:{}", width, height);
    for a_box in boxes.iter() {
        println!("id:{} x:{} y:{} w:{} h:{}", a_box.id,a_box.x,a_box.y, a_box.w,a_box.h);
    }

    debug_draw_boxes(&boxes, width, height, 4.);
}

/// Some naive way of drawing the boxes in the console for debugging
fn debug_draw_boxes(boxes:&[Box2D], width:f32, height:f32, scale_factor:f32) {
    for y in 0 ..= (height/scale_factor) as i32 {
        for x in 0 ..= (width/scale_factor) as i32 {
            if is_border(boxes, x, y, scale_factor) {
                print!("#");
            }else{
                print!(".");
            }
        }
        println!();
    }
}

fn is_border(boxes:&[Box2D], x:i32, y:i32, scale_factor:f32) -> bool {
    for a_box in boxes.iter() {
        if ((a_box.x/scale_factor) as i32  == x || ((a_box.x+a_box.w)/scale_factor) as i32  == x )
            && (a_box.y/scale_factor) as i32  <= y
            && ((a_box.y+a_box.h)/scale_factor) as i32  >= y {
            return true;
        }
        if ((a_box.y/scale_factor) as i32  == y || ((a_box.y+a_box.h)/scale_factor) as i32  == y)
            && (a_box.x/scale_factor) as i32  <= x
            && ((a_box.x+a_box.w)/scale_factor) as i32  >= x {
            return true;
        }
    }
    false
}