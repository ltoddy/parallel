use std::thread;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;

type Job = Box<FnBox + Send + 'static>;

trait FnBox {
    fn call(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call(self: Box<Self>) {
        (*self)()
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            loop {
                while let Ok(job) = receiver.lock().unwrap().recv() {
                    println!(" Worker {:?} got a job; executing.", id);

                    job.call()
                }
            }
        });

        return Worker {
            id,
            thread,
        };
    }
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        /// 线程池的size最好与电脑cpu数量挂钩.然后乘以电脑cpu数量的一个倍数
        /// ## example
        /// let cpu_count = os::cpu_count();
        ///
        /// let pool = ThreadPool::new(cpu_count * x);
        ///
        /// 这里的 x 是cpu数量的倍数,然后在未来我会为ThreadPool添加一个默认的size(也就是电脑cpu的数量),
        /// 未来构建我这个线程池的时候只需要参数x就可以了.
        ///
        /// ## In the future example:
        ///
        /// let pool = ThreadPool::new(x);
        ///
        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        return ThreadPool { workers, sender };
    }

    pub fn execute<F>(&self, f: F) -> ()
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
