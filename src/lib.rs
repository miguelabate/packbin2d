use std::cmp::Ordering;

pub trait Box2DTrait {
    fn get_x(&self) -> f32;
    fn set_x(&mut self, x: f32);
    fn get_y(&self) -> f32;
    fn set_y(&mut self, y: f32);
    fn get_w(&self) -> f32;
    fn set_w(&mut self, w: f32);
    fn get_h(&self) -> f32;
    fn set_h(&mut self, h: f32);
    fn get_id(&self) -> &String;
}

pub struct Box2D {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub id: String,
}

impl Box2D {
    pub fn new(x: f32, y: f32, w: f32, h: f32, id: &str) -> Self {
        Self { x, y, h, w, id:id.to_string() }
    }
}

impl Box2DTrait for Box2D {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn get_w(&self) -> f32 {
        self.w
    }

    fn set_w(&mut self, w: f32) {
        self.w = w;
    }

    fn get_h(&self) -> f32 {
        self.h
    }

    fn set_h(&mut self, h: f32) {
        self.h = h;
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}

//this uses [0,0] top left and it's the corner of the box. orders the passed vector and returns the dimensions of the combined pack (width, height)
pub fn pack_bin<T: Box2DTrait>(boxes: &mut Vec<T>) -> (f32, f32) {
    // calculate total box area and maximum box width
    let mut area: f32 = 0.;
    let mut max_width: f32 = 0.;
    let mut final_height: f32 = 0.;
    for a_box in boxes.iter() {
        area += a_box.get_w() * a_box.get_h();
        max_width = max_width.max(a_box.get_w());
    }

    // sort the boxes for insertion by height, descending
    boxes.sort_by(|a, b| {
        if b.get_h() > a.get_h() {
            return Ordering::Greater;
        } else if b.get_h() < a.get_h() {
            return Ordering::Less;
        }
        Ordering::Equal
    });

    // aim for a squarish resulting container,
    // slightly adjusted for sub-100% space utilization
    let start_width = (area / 0.95).sqrt().ceil().max(max_width);

    // start with a single empty space, unbounded at the bottom
    let mut spaces = vec![Box2D::new(0., 0., start_width, f32::MAX, "0")];

    for a_box in boxes.iter_mut() {
        for i in (0..spaces.len()).rev() {
            // look for empty spaces that can accommodate the current box
            if a_box.get_w() > spaces[i].get_w() || a_box.get_h() > spaces[i].get_h() {
                continue;
            }

            // found the space; add the box to its top-left corner
            // |-------|-------|
            // |  box  |       |
            // |_______|       |
            // |         space |
            // |_______________|
            // packed.push(Box2D::new(spaces[i].x,spaces[i].y,a_box.w,a_box.h, a_box.id));
            a_box.set_x(spaces[i].get_x());
            a_box.set_y(spaces[i].get_y());
            if a_box.get_w() == spaces[i].get_w() && a_box.get_h() == spaces[i].get_h() {
                // space matches the box exactly; remove it
                let last = spaces.pop().unwrap();
                if i < spaces.len() {
                    spaces[i] = last;
                }
            } else if a_box.get_h() == spaces[i].get_h() {
                // space matches the box height; update it accordingly
                // |-------|---------------|
                // |  box  | updated space |
                // |_______|_______________|
                let x = spaces[i].get_x() + a_box.get_w();
                spaces[i].set_x(x);
                let w = spaces[i].get_w() - a_box.get_w();
                spaces[i].set_w(w);
            } else if a_box.get_w() == spaces[i].get_w() {
                // space matches the box width; update it accordingly
                // |---------------|
                // |      box      |
                // |_______________|
                // | updated space |
                // |_______________|
                let y = spaces[i].get_y() + a_box.get_h();
                spaces[i].set_y(y);
                let h = spaces[i].get_h() - a_box.get_h();
                spaces[i].set_h(h);
            } else {
                // otherwise the box splits the space into two spaces
                // |-------|-----------|
                // |  box  | new space |
                // |_______|___________|
                // | updated space     |
                // |___________________|
                spaces.push(Box2D::new(
                    spaces[i].get_x() + a_box.get_w(),
                    spaces[i].get_y(),
                    spaces[i].get_w() - a_box.get_w(),
                    a_box.get_h(),
                    "0",
                ));
                let y = spaces[i].get_y() + a_box.get_h();
                spaces[i].set_y(y);
                let h = spaces[i].get_h() - a_box.get_h();
                spaces[i].set_h(h);
            }
            break;
        }
        //update max height to return
        if final_height < (a_box.get_h() + a_box.get_y()) {
            final_height = a_box.get_h() + a_box.get_y();
        }
    }
    (start_width, final_height)
}