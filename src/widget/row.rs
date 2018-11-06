use std::rc::Rc;

use layout_object::{FlexLayoutObject, LayoutObject};
use enums::Alignment;
use widget::{Template, Widget};


pub struct Row {
    pub children: Vec<Rc<Widget>>,
}

impl Default for Row {
    fn default() -> Row {
        Row {
            children: vec![],
        }
    }
}

impl Widget for Row {
    fn template(&self) -> Template {
        print!("Row -> ");
        if self.children.len() == 0 {
            Template::Empty
        } else if self.children.len() == 1 {
            Template::Single(self.children.get(0).unwrap().clone())
        } else {
            Template::Mutli(self.children.iter().map(|child| child.clone()).collect())
        }
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(FlexLayoutObject::new(Alignment::Horizontal))
    }
}
