// #[derive(Default)]
// pub struct DomBox {
//     marginLeft: i32,
//     marginRight: i32,
//     marginBottom: i32,
//     border: i32,
//     padding: i32,
// }
// impl DomBox {
//     pub fn new() -> Self {
//         DomBox::default()
//     }
// }
/**盒模型 */
pub trait DomEvent {
    fn on_click();
}
pub trait DomBox: DomEvent {
    fn on_click() {

    }
}
#[derive(Default, Debug)]
pub struct VitrualDom {
    pub virtualDomId: i32,
    pub vtype: &'static str ,
    pub shouldRender: bool,
    pub children: Vec<VitrualDom>,
}
impl VitrualDom {
    pub fn new() -> Self {
        VitrualDom::default()
    }
    // 
    pub fn pushChild(&mut self, vd: VitrualDom) {
        self.children.push(vd);
    }
    pub fn render(&mut self){
        if(self.shouldRender){
            self.update();
            self.shouldRender = false;
        } else {
            self.shouldRender = true;
        }
    }
    pub fn update(&mut self){
        println!("virtualDOM {:?} is update", self.virtualDomId);
        for item in &mut self.children {
            item.render();
        }
        self.shouldRender = false;
    }
}