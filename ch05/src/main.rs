use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    Future, FutureExt,
};

struct Hello {
    state: StateHello,
}
enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    fn new() -> Self {
        Hello {
            state: StateHello::HELLO,
        }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                println!("Hello, ");
                (*self).state = StateHello::WORLD;
                Poll::Pending
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                Poll::Pending
            }
            StateHello::END => Poll::Ready(()),
        }
    }
}

struct Task {
    hello: Mutex<BoxFuture<'static, ()>>,
}

impl Task {
    fn new() -> Self {
        let hello = Hello::new();
        Task {
            hello: Mutex::new(hello.boxed()),
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

fn main() {
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker); // <2>
    let mut hello = task.hello.lock().unwrap();

    // 停止と再開の繰り返し <3>
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
}
