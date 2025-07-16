 use std::collections::HashSet;

#[derive(Debug,Clone,Copy)]
pub struct ColorP{
    c:[[i32; 10]; 8],
    step:usize
}
#[derive(Debug,Clone,Copy)]
pub struct Pcolor{
    x:usize,
    y:usize,
    c:i32,
}





impl ColorP{
    fn is_finished(&self )->bool{
        let mut color = self.c[0][0];
        for i in 0..8{
            for j in 0..10{
                let temp = self.c[i][j];
                if color==0&&temp!=0{
                    color = temp;
                }
                if color!=0&& temp!=0&& temp!=color{
                    return false;
                }
            }
        }
        true
    }

    fn near_color(&mut self, p:Pcolor)->HashSet<i32>{
        let mut temp = HashSet::new();
        let current = self.c[p.x][p.y];
        if p.x>0{
            if  current!=self.c[p.x-1][p.y] && self.c[p.x-1][p.y]!=0{
                temp.insert(self.c[p.x-1][p.y]);
            }else if current==self.c[p.x-1][p.y] && self.c[p.x-1][p.y]!=0{
                let temp_p = Pcolor{x:p.x-1,y:p.y,c:0};
                temp.extend(self.near_color(temp_p));
            }
        }
        if p.x<7{
            if  current!=self.c[p.x+1][p.y] && self.c[p.x+1][p.y]!=0{
                temp.insert(self.c[p.x+1][p.y]);
            }else if current==self.c[p.x+1][p.y] && self.c[p.x+1][p.y]!=0{
                let temp_p = Pcolor{x:p.x+1,y:p.y,c:0};
                temp.extend(self.near_color(temp_p));
            }
            
        }
        if p.y>0 {
            if current!=self.c[p.x][p.y-1] && self.c[p.x][p.y-1]!=0{
                temp.insert(self.c[p.x][p.y-1]);
            }else if current==self.c[p.x][p.y-1] && self.c[p.x][p.y-1]!=0{
                let temp_p = Pcolor{x:p.x,y:p.y-1,c:0};
                temp.extend(self.near_color(temp_p));
            }
            
        }
        if p.y<9 {
            if current!=self.c[p.x][p.y+1] && self.c[p.x][p.y+1]!=0{
                temp.insert(self.c[p.x][p.y+1]);
            }else if current==self.c[p.x][p.y+1] && self.c[p.x][p.y+1]!=0{
                let temp_p = Pcolor{x:p.x,y:p.y+1,c:0};
                temp.extend(self.near_color(temp_p));
            }
            
        }
        temp
    }

    fn change_color(&mut self, p:Pcolor){
        let current = self.c[p.x][p.y];
        self.c[p.x][p.y] = p.c;
        if p.x>0&& current ==self.c[p.x-1][p.y]{
            let temp_pc = Pcolor{
                    x:p.x-1,
                    y:p.y,
                    c:p.c,
                };
            self.change_color(temp_pc);
        }
        if p.x<7&& current ==self.c[p.x+1][p.y]{
            let temp_pc = Pcolor{
                    x:p.x+1,
                    y:p.y,
                    c:p.c,
                };
            self.change_color(temp_pc);
        }
        if p.y>0&& current==self.c[p.x][p.y-1]{
            let temp_pc = Pcolor{
                    x:p.x,
                    y:p.y-1,
                    c:p.c,
                };
            self.change_color(temp_pc);
        }
        if p.y<9&& current==self.c[p.x][p.y+1]{
            let temp_pc = Pcolor{
                    x:p.x,
                    y:p.y+1,
                    c:p.c,
                };
            self.change_color(temp_pc);
        }
    }

    fn is_instep(&self,way:&mut Vec<Pcolor>,pc:Pcolor){
        
    }

    fn backtrack(&mut self, way:&mut Vec<Pcolor>)->bool{
        
        for i in 0..8{

            for j in 0..10{

                if self.c[i][j]==0{
                    continue;
                }
                let mut pc = Pcolor{
                    x:i,
                    y:j,
                    c:0,
                };


                let near = self.near_color(pc);
                let current = self.c[i][j];
                for k in near{
                    pc.c = k;
                    self.change_color(pc);
                    way.push(pc);
                    if way.len()>self.step{
                        return false;
                    }
                    if self.is_finished(){
                        return true;
                    }
                    if  self.backtrack(way){
                        return true;
                    }
                    pc.c = current;
                    self.change_color(pc);
                    way.pop();

                }


            }
        }
        true
    }


}



#[test]
pub fn run(){
    let  board=[
    [3,0,0,4,4,4,4,1,1,3],
    [3,1,1,2,2,2,2,3,1,3],
    [3,1,3,4,4,4,4,1,1,3],
    [3,1,1,2,2,2,2,3,3,3],
    [3,3,3,4,4,4,4,1,1,3],
    [3,1,1,2,2,2,2,3,1,3],
    [3,1,3,4,4,4,4,1,1,3],
    [3,1,1,2,2,2,2,0,0,3]
    ];

    let mut  s = ColorP{
        c:board,
        step:5,
    };
    let mut way:Vec<Pcolor> = Vec::new();
    let _ = s.backtrack(&mut way);
    println!("{:?}",way);
    println!("结果{:?}",s);

}