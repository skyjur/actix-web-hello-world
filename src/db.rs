extern crate actix;
extern crate lmdb_zero as lmdb;
use std::sync::Arc;

pub struct DbWriter {
    pub db: Arc<lmdb::Database<'static>>,
}

pub struct DbReader {
    pub db: Arc<lmdb::Database<'static>>,
}

impl actix::Actor for DbWriter {
    type Context = actix::SyncContext<Self>;
}

impl actix::Actor for DbReader {
    type Context = actix::SyncContext<Self>;
}

pub struct DbPut {
    pub key: String,
    pub value: String,
}

pub struct DbGet {
    pub key: String,
}

impl actix::Message for DbPut {
    type Result = Result<String, String>;
}

impl actix::Message for DbGet {
    type Result = Result<String, String>;
}

impl actix::Handler<DbPut> for DbWriter {
    type Result = Result<String, String>;

    fn handle(&mut self, msg: DbPut, _: &mut Self::Context) -> Self::Result {
        let txn = try!(
            lmdb::WriteTransaction::new(self.db.env())
                .map_err(|_e| format!("Cant open transaction (code: {})", _e))
        );
        {
            let mut access = txn.access();
            try!(
                access
                    .put(
                        &self.db,
                        msg.key.as_str(),
                        msg.value.as_str(),
                        lmdb::put::Flags::empty()
                    ).map_err(|_e| format!("Error in put() code: {}", _e))
            )
        }
        txn.commit().unwrap();
        self.db.env().sync(false).unwrap();
        Ok(format!("Inserted at key {} value: {}", msg.key, msg.value))
    }
}

impl actix::Handler<DbGet> for DbReader {
    type Result = Result<String, String>;

    fn handle(&mut self, msg: DbGet, _: &mut Self::Context) -> Self::Result {
        let txn = lmdb::ReadTransaction::new(self.db.env()).unwrap();
        let access = txn.access();
        //let key = lmdb::traits::AsLmdbBytes::as_lmdb_bytes(msg.key.as_str());
        let value = access.get::<str, str>(&self.db, msg.key.as_str());
        // let value: String = try!(
        //     access.get(&self.db, key)
        //     // .map_err(|_e| match _e {
        //     //     lmdb::Error::Code(lmdb::error::NOTFOUND) => format!("NOT FOUND ({})", _e),
        //     //     _ => format("GET ERR {}", _e)
        //     // })
        // );
        match value {
            Ok(s) => Ok(format!("Retrieved at key {} value: {}", msg.key, s)),
            Err(lmdb::Error::Code(lmdb::error::NOTFOUND)) => {
                Err(format!("Not Found key {}", msg.key))
            }
            Err(_e) => Err(format!("Error code {} at key {}", _e, msg.key)),
        }
    }
}

pub fn open() -> Arc<lmdb::Database<'static>> {
    let env = Arc::new(unsafe {
        let mut b = lmdb::EnvBuilder::new().unwrap();
        b.set_mapsize(10485760000).unwrap();
        b.open("db", lmdb::open::Flags::empty(), 0o400).unwrap()
    });

    Arc::new(lmdb::Database::open(env.clone(), None, &lmdb::DatabaseOptions::defaults()).unwrap())
}
