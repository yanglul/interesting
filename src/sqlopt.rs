
use anyhow::Result;
use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct item {
    pub id: i64,
    pub pid: i64,
    pub name:String,
    pub task_id:String,
    pub state: i32,
    pub executor:String,
}


pub enum ItemState{
    Runnig,
    Pass,
    Reject,
}
 
impl ItemState{
    pub fn to(&self)->u8{
        match self {
            ItemState::Runnig =>return 1,
            ItemState::Pass => return 2,
            ItemState::Reject => return 3,
        }
    }

}


async fn create_pool() -> Result<Pool> {
    let url =  "mysql://root:123456@192.168.61.131:3306/flow";
    let pool = Pool::new(url)?;
    Ok(pool)
}


 

pub async fn create_item(
    pool: &Pool,
    pid: i64,
    name: String,
    task_id:String,
    state: i32,
    executor: String,
) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop("INSERT INTO item (pid,name,task_id,state,executor) VALUES (?,?,?,?,?)", (pid,name,task_id,state,executor))?;
    Ok(())
}



pub async fn update_item(
    pool: &Pool,
    pid: i64,
    name: String,
    task_id:String,
    state: i32,
    executor: String,
) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop("update   item set state=?,   where task_id =? and executor=? and state=? ) ", (state,task_id,executor,ItemState::Runnig.to()))?;
    Ok(())
}







#[test]
fn run(){
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // 阻塞执行异步函数
    let result = rt.block_on(async {
        let pool = create_pool().await.unwrap();
        create_item(&pool,2, "CLA".to_string(), "TASK_20250430001".to_string(), 2, "10032".to_string()).await.unwrap();
    });
    



}