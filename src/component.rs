use crate::dom::VitrualDom;
pub type Button = VitrualDom;
pub type Label = VitrualDom;

pub struct ImageBuffer {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

pub type Image = ();

// 组件widget
pub trait Widget {
    fn inner_image() -> Image {
        ()
    }
}


#[derive(Debug)]
pub enum ComponentType {
    View,
    Text,
    Other,
}
pub trait Component{
    fn component(&mut self) -> String;
}

// #[derive(Debug)]
pub struct View{
    ctype: ComponentType,
    children: Vec<Box<dyn Component>>,
}
impl View{
    pub fn new() -> Self{
        View{
            ctype: ComponentType::View,
            children: vec![],
        }
    }
    pub fn push(&mut self, component: Box<dyn Component>){
        self.children.push(component);
    }
    pub fn render(&mut self){
        println!("{}", self.component());
    }
}
impl Component for View{
    fn component(&mut self) -> String { 
        let mut component =  String::new();
        component.push_str("<div>");
        for view in &mut self.children{
            component.push_str(view.component().as_str());
        }
        component.push_str("</div>");
        component
     }
}

// #[derive(Debug)]
pub struct Text{
    ctype: ComponentType,
    text: String,
}
impl Component for Text{
    fn component(&mut self) -> String { 
        let component =  String::from(self.text.as_str());
        component
     }
    
}
impl Text {
    pub fn new() -> Self{
        Text{
            ctype: ComponentType::Text,
            text: String::from("string"),
        }
    }
}