use packbin2d::{Box2D, pack_bin};

fn main() {
    let mut boxes = vec![];
    boxes.push(Box2D::new(0.,0.,100.,30.,"1"));
    boxes.push(Box2D::new(0.,0.,10.,20.,"2"));
    boxes.push(Box2D::new(0.,0.,120.,330.,"3"));
    boxes.push(Box2D::new(0.,0.,40.,68.,"4"));
    boxes.push(Box2D::new(0.,0.,125.,68.,"5"));
    boxes.push(Box2D::new(0.,0.,10.,8.,"6"));

    pack_bin(&mut boxes);

    for a_box in boxes.iter() {
        println!("id:{} x:{} y:{}", a_box.id,a_box.x,a_box.y);
    }
}