use std::cmp::Ordering;

pub struct Box2D {
    x:f32,
    y:f32,
    w:f32,
    h:f32,
    id:i32,
}

impl Box2D {
    pub fn new(x:f32, y:f32, w:f32, h:f32, id:i32) -> Self {
        Self{x,y,h,w,id}
    }
}
fn main() {
    let mut boxes = vec![];
    boxes.push(Box2D::new(0.,0.,100.,30.,1));
    boxes.push(Box2D::new(0.,0.,10.,20.,2));
    boxes.push(Box2D::new(0.,0.,120.,330.,3));
    boxes.push(Box2D::new(0.,0.,40.,68.,4));
    boxes.push(Box2D::new(0.,0.,125.,68.,5));
    boxes.push(Box2D::new(0.,0.,10.,8.,6));

    let packed = pack_bin(&mut boxes);

    for a_box in packed.iter() {
        println!("id:{} x:{} y:{}", a_box.id,a_box.x,a_box.y);
    }
}

pub fn pack_bin(boxes:&mut Vec<Box2D>) -> Vec<Box2D> {
    // calculate total box area and maximum box width
    let mut area:f32 = 0.;
    let mut maxWidth:f32 = 0.;
    for a_box in boxes.iter() {
        area += a_box.w * a_box.h;
        maxWidth = maxWidth.max(a_box.w);
    }

    // sort the boxes for insertion by height, descending
    boxes.sort_by(|a, b| {
        if b.h > a.h{
            return Ordering::Greater
        }else if  b.h < a.h{
            return Ordering::Less
        }
        Ordering::Equal
    });

    // aim for a squarish resulting container,
    // slightly adjusted for sub-100% space utilization
    let mut startWidth = (area / 0.95).sqrt().ceil().max(maxWidth);

    // start with a single empty space, unbounded at the bottom
    let mut spaces = vec![Box2D::new(0.,0.,startWidth,f32::MAX,0)];
    let mut packed:Vec<Box2D> = vec![];

    for a_box in boxes.iter() {
        for i in (0 .. spaces.len()).rev()  {
            // look for empty spaces that can accommodate the current box
            if a_box.w > spaces[i].w || a_box.h > spaces[i].h {continue;}

            // found the space; add the box to its top-left corner
            // |-------|-------|
            // |  box  |       |
            // |_______|       |
            // |         space |
            // |_______________|
            packed.push(Box2D::new(spaces[i].x,spaces[i].y,a_box.w,a_box.h, a_box.id));

            if (a_box.w == spaces[i].w && a_box.h == spaces[i].h) {
                // space matches the box exactly; remove it
                let last = spaces.pop().unwrap();
                if i < spaces.len() {
                    spaces[i] = last;
                }

            } else if (a_box.h == spaces[i].h) {
                // space matches the box height; update it accordingly
                // |-------|---------------|
                // |  box  | updated space |
                // |_______|_______________|
                spaces[i].x += a_box.w;
                spaces[i].w -= a_box.w;

            }else if (a_box.w == spaces[i].w) {
                // space matches the box width; update it accordingly
                // |---------------|
                // |      box      |
                // |_______________|
                // | updated space |
                // |_______________|
                spaces[i].y += a_box.h;
                spaces[i].h -= a_box.h;

            } else {
                // otherwise the box splits the space into two spaces
                // |-------|-----------|
                // |  box  | new space |
                // |_______|___________|
                // | updated space     |
                // |___________________|
                spaces.push(Box2D::new(
                    spaces[i].x + a_box.w,
                     spaces[i].y,
                     spaces[i].w - a_box.w,
                     a_box.h,
                     0
                ));
                spaces[i].y += a_box.h;
                spaces[i].h -= a_box.h;
            }
            break;
        }
    }
    packed
}