#[macro_use]
extern crate rbatis;
use rbatis::executor::Executor;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    )
        .expect("App log init fail");

    let mut rb = Rbatis::new();
    rb.init(PgDriver {}, "postgres://sere:sere@localhost:5432/sere")
        .unwrap();

    let entry = TodoEntry {
        id: None,
        cont: String::from("test"),
        status: String::from("Pending"),
        deadline: FastDateTime::now(),
        ctime: FastDateTime::now(),
        mtime: FastDateTime::now()
    };
    let res = TodoEntry::insert(&mut rb, &entry).await.unwrap();
    println!("insert result: {:?}", res);

    let res = select_by_status(&mut rb, "Pending").await;
    println!("query result: {:?}", res);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoEntry {
    pub id: Option<u64>,
    pub cont: String,
    pub status: String,
    pub deadline: FastDateTime,
    pub ctime: FastDateTime,
    pub mtime: FastDateTime,
}

rbatis::impl_insert!(TodoEntry {});

#[html_sql(
r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<select id="select_by_status">
      `select * from todo_entry`
      <where>
          <if test="status != ''">
              ` status = #{status}`
          </if>
      </where>
</select>"#
)]
async fn select_by_status(rb: &mut dyn Executor, status: &str) -> Vec<TodoEntry> {
    impled!()
}