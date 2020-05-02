use rand::Rng;

#[cfg(not(test))]
fn get_probability() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, 1.0)
}

#[cfg(test)]
fn get_probability() -> f64 {
    0.7
}

struct Task(String);
struct TaskList(Vec<(Task, f64)>);
impl TaskList {
    fn new(v: Vec<(Task, f64)>) -> TaskList {
        let total_probability: f64 = v.iter().map(|e| e.1).sum();
        if total_probability != 1.0 {
            panic!("total probability must add up to 1.0");
        }

        TaskList(v)
    }

    fn pick_task(&self) -> &Task {
        let p = get_probability();
        let mut curr = 0.0;

        for t in &self.0 {
            curr += t.1;
            if p < curr {
                return &t.0;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod task_list_test {
    use super::{Task, TaskList};

    #[test]
    #[should_panic(expected = "total probability must add up to 1.0")]
    fn must_have_valid_probability() {
        let data = vec![(String::from(""), 0.5)];
        let _ts = TaskList::new(data.into_iter().map(|e| (Task(e.0), e.1)).collect());
    }

    #[test]
    fn picks_second_task() {
        let data = vec![(String::from("a"), 0.5), (String::from("b"), 0.5)];
        let ts = TaskList::new(data.into_iter().map(|e| (Task(e.0), e.1)).collect());
        assert_eq!(ts.pick_task().0, "b");
    }
}
