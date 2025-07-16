use std::collections::VecDeque;
use std::rc::Rc;
use anyhow::Result;

//definition start->end node type
pub enum NodeType {
    Start,
    Middle,
    End,
}
pub struct Node{
    pid:i32,
    name:String,
    pre:Option<Rc<Node>>,
    next:Option<Vec<Rc<Node>>>,//mybe multiple nodes
    typ:NodeType,
    check:bool, //false if goback this node will be skip
}
impl Node {
    pub fn new(pid:i32,name:String,check:bool,nt:NodeType)->Self{
        Self { pid:pid,
            name:name,
            pre:None,
            next:None,
            typ:nt,
            check:check, 
        }
    
    }

    pub fn set_next(self,nodes:Vec<Rc<Node>>)->Self{
        Self { pid:self.pid,
            name:self.name,
            pre:self.pre,
            next:Some(nodes),
            typ:self.typ,
            check:self.check, 
        }
    }


}
//Process Approach
pub struct Flow {
    vec: VecDeque<Node>,
}


trait Porcess {
    fn reject()->Result<()>;
    fn goback(name:String)->Result<()>;
    fn submit()->Result<()>;
}




impl Porcess for Flow{
    fn reject()->Result<()>{
        
        Ok(())
    }

    fn goback(name:String)->Result<()>{
        Ok(())
    }

    fn submit()->Result<()>{
        Ok(())
    }

}





#[test]
pub  fn run(){
    // let pool = MySqlPool::connect("jdbc:mysql://192.168.61.131:3306/mysql?allowPublicKeyRetrieval=true").await?;


}