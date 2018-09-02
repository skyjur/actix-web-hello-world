extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate lmdb_zero as lmdb;
use actix_web::AsyncResponder;
use actix_web::{
    error::ErrorBadRequest, http::Method, server, App, HttpMessage, HttpRequest, HttpResponse,
    Responder,
};
use futures::Future;

mod db;

struct State {
    reader: actix::Addr<db::DbReader>,
    writer: actix::Addr<db::DbWriter>,
}

fn get(req: &HttpRequest<State>) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");

    let r = req.state().reader.send(db::DbGet {
        key: String::from(to),
    });

    let rr = r.and_then(|res| match res {
        Ok(s) => Ok(HttpResponse::Ok().body(s)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    });

    rr.responder()
}

fn post(req: &HttpRequest<State>) -> impl Responder {
    let key = String::from(req.match_info().get("name").unwrap());
    let writer = req.state().writer.clone();
    let b = req.body();

    let bb = b
        .from_err()
        .and_then(move |bytes| match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(s),
            Err(_e) => Err(ErrorBadRequest("Encoding Error")),
        });

    let bbb = bb.and_then(move |body| {
        let r = writer
            .send(db::DbPut {
                key: key,
                value: body,
            }).from_err();

        r.and_then(|res| match res {
            Ok(ok) => Ok(HttpResponse::Ok().body(ok)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
    });

    bbb.responder()
}

fn main() {
    let sys = actix::System::new("actix-hello");

    let db = db::open();

    let reader = {
        let db2 = db.clone();
        actix::SyncArbiter::start(4, move || db::DbReader { db: db2.clone() })
    };
    let writer = {
        let db2 = db.clone();
        actix::SyncArbiter::start(1, move || db::DbWriter { db: db2.clone() })
    };

    server::new(move || {
        App::with_state(State {
            reader: reader.clone(),
            writer: writer.clone(),
        }).resource("/{name}", |r| {
            r.method(Method::GET).f(get);
            r.method(Method::POST).f(post);
        })
    }).bind("127.0.0.1:8005")
    .expect("Can not bind to port 8005")
    .run();

    sys.run();
}
