use rocket_db_pools::Connection;

use crate::{routes::ErrorResponder, Db};

pub mod author;
pub mod chapter;
pub mod genre;
pub mod manga;
pub mod page;
pub mod pattern;
pub mod query;
pub mod source;

#[async_trait]
pub trait Assemble : Sized {

    async fn all(_: &Db) -> Result<Vec<Self>, ErrorResponder> {
        Err(ErrorResponder {
            message: "not implemented".to_string()
        })
    }
    
    async fn assemble(_: &'_ str , _: &mut Connection<Db>,) -> Result<Self, ErrorResponder> {
        Err(ErrorResponder {
            message: "not implemented".to_string()
        })
    }

    async fn assemble_many(_: &'_ str, _: &mut Connection<Db>,) -> Result<Vec<Self>, ErrorResponder> {
        Err(ErrorResponder {
            message: "not implemented".to_string()
        })
    }

}

#[async_trait]
pub trait AssembleWithArgs<T>: Sized where T: std::marker::Send {
    
    async fn all_with_args<'a>(_: T, _: &mut Connection<Db>) -> Result<Vec<Self>, ErrorResponder> where T: 'a {
        Err(ErrorResponder {
            message: "not implemented".to_string()
        })
    }
    
    async fn assemble_with_args<'a>(_: &'_ str , _: T, _: &mut Connection<Db>,) -> Result<Self, ErrorResponder> where T: 'a {
        Err(ErrorResponder {
            message: "not implemented".to_string()
        })
    }

    async fn assemble_many_with_args<'a>(_: &'_ str, _: T, _: &mut Connection<Db>,) -> Result<Vec<Self>, ErrorResponder> where T: 'a {
        Err(ErrorResponder {
            message: "not implemented".to_string()
        })
    }

}

#[async_trait]
pub trait AssembleWithOutput<T>: Sized where T: std::marker::Send {
    
    async fn all_with_output(_: &Db) -> Result<T, ErrorResponder> {
        Err(ErrorResponder {
            message: "not implemented".to_string()
        })
    }
    
    // async fn assemble(_: &'_ str , _: T, _: &mut Connection<Db>,) -> Result<Self, ErrorResponder> {
    //     Err(ErrorResponder {
    //         message: "not implemented".to_string()
    //     })
    // }

    async fn assemble_many_with_output(_: &mut Connection<Db>,) -> Result<Vec<Self>, ErrorResponder> {
        Err(ErrorResponder {
            message: "not implemented".to_string()
        })
    }

}

#[async_trait]
pub trait AssembleWithArgsAndOutput<T, U>: Sized where T : std::marker::Send {
    
    async fn all_with_args_and_output<'a>(_: T, _: &mut Connection<Db>) -> Result<U, ErrorResponder> where T: 'a {
        Err(ErrorResponder {
            message: "not implemented".to_string()
        })
    }
    
    // async fn assemble(_: &'_ str , _: T, _: &mut Connection<Db>,) -> Result<Self, ErrorResponder> {
    //     Err(ErrorResponder {
    //         message: "not implemented".to_string()
    //     })
    // }

    // async fn assemble_many(_: &mut Connection<Db>,) -> Result<Vec<Self>, ErrorResponder> {
    //     Err(ErrorResponder {
    //         message: "not implemented".to_string()
    //     })
    // }

}
