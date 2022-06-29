use std::fmt;
use std::future::Future;
use std::task::{Context, Poll};
use tower_service::Service;

pub fn service_fn<C,T>(cfg: C, func: T) -> ServiceFn<C,T> {
    ServiceFn { cfg, func }
}

//#[derive(Copy, Clone)]
#[derive(Clone)]
pub struct ServiceFn<C,T> {
    cfg: C,
    func: T,
}

impl<C,T> fmt::Debug for ServiceFn<C, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServiceFn")
            .field("func", &format_args!("{}", std::any::type_name::<T>()))
            .field("cfg", &format_args!("{}", std::any::type_name::<C>()))
            .finish()
    }
}

impl<C, T, F, Request, R, E> Service<Request> for ServiceFn<C,T>
where
    T: FnMut(C, Request) -> F,
    F: Future<Output = Result<R, E>>,
    C: Clone
{
    type Response = R;
    type Error = E;
    type Future = F;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), E>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request) -> Self::Future {
        (self.func)(self.cfg.clone(), req)
    }
}
