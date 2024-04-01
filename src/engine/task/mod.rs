#[macro_export]
macro_rules! invoke_task {
    ($task:ident $(, $param:expr)*) => {
        macroquad::experimental::coroutines::start_coroutine($task ( $($param),* ))
    }
}

#[macro_export]
macro_rules! invoke_task_delayed {
    ($delay:expr, $task:ident $(, $param:expr)*) => {
        macroquad::experimental::coroutines::start_coroutine(async move {
            macroquad::experimental::coroutines::wait_seconds($delay).await;
            macroquad::experimental::coroutines::start_coroutine($task ( $($param),* ))
        })
    }
}
