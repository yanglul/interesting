 

#[derive(Debug)]
pub struct Sudoku{
    c:[[i32; 9]; 9],
}

impl Sudoku{
    fn is_valid(&self,row:usize,col:usize,num:i32)->bool{
        for i in 0..9{
            if self.c[row][i]==num|| self.c[i][col]==num{
                return false;
            }
            if self.c[3*(row/3)+i/3][3*(col/3)+i%3]==num{
                return false;
            }
        }
        true
    }

    fn backtrack(&mut self)->bool{
        for i in 0..9{
            for j in 0..9{
                if (self.c[i][j] != 0){
                    continue;
                }
                for k in 1..=9{
                    if self.is_valid(i, j, k){
                        self.c[i][j] = k;
                        if self.backtrack(){
                            return true;
                        }
                        self.c[i][j] = 0;
                    }
                
                }
                return false;
            }
        }
        true
    }


}




pub fn run(){
    let mut board=[
    [5,3,0,0,7,0,0,0,0],
    [6,0,0,1,9,5,0,0,0],
    [0,9,8,0,0,0,0,6,0],
    [8,0,0,0,6,0,0,0,3],
    [4,0,0,8,0,3,0,0,1],
    [7,0,0,0,2,0,0,0,6],
    [0,6,0,0,0,0,2,8,0],
    [0,0,0,4,1,9,0,0,5],
    [0,0,0,0,8,0,0,7,9]
    ];

    let mut  s = Sudoku{
        c:board
    };
    let _ = s.backtrack();
    println!("结果{:?}",s);

}