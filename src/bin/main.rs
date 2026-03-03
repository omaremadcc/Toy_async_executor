use toy_executor::*;
fn main() {
    let job_1 = Job::new(0, |tx| {
        loop {
            println!("Playing Basketball");
            println!("Waiting for the basketball court to get empty");
            let _ = tx.send(0);
            return MineFuture::Pending;
        }
    });
    let job_2 = Job::new(1, |tx| {
        loop {
            println!("Playing Soccer");
            println!("Waiting for the field to get empty");
            let _ = tx.send(1);
            return MineFuture::Pending;
        }
    });
    let job_3 = Job::new(2, |tx| {
        loop {
            println!("Playing tennis");
            println!("Waiting for the tennis court to get empty");
            let _ = tx.send(2);
            return MineFuture::Pending;
        }
    });

    let executor = Executor::new();
    executor.exeutor(vec![job_1, job_2, job_3]);
}
