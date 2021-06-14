#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub queued_at: u32,
    pub execution_duration: u32,
}

pub order_by_duration() -> (Vec<Option<Box<Task>>>, u64) {
  // TO DO
}

pub fn execution_order(tasks: Vec<Task>) -> Vec<u64> {
    let mut ref_tasks: Vec<Option<Box<Task>>> = vec![None; tasks.len()];
    let mut ordered_tasks: Vec<u64>;

    for task in tasks {
      let task_queued_at = task.queued_at;
      let task_ref = Some(Box::new(task));
      ref_tasks[task_queued_at as usize] = task_ref;
    }

    let start = 0;
    let end = 0;

    while end < tasks.len() {
      let tasks_ref_slice = &ref_tasks[start..end];
      
      order_by_duration(tasks_ref_slice) -> (ordered_tasks_ref, duration_sum)
      
      for task_ref in ordered_tasks_ref {
        ordered_tasks.push(task_ref.id);
      }

      let prev_start = start;
      start = end;
      end = previous_start + duration_sum;
    }

    ordered_tasks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_queue_order() {
        let tasks = vec![
            Task { id: 42, queued_at: 5, execution_duration: 3 },
            Task { id: 43, queued_at: 2, execution_duration: 3 },
            Task { id: 44, queued_at: 0, execution_duration: 2 },
        ];

        assert_eq!(execution_order(tasks), vec![44, 43, 42]);
    }

    #[test]
    fn two_items_queued_at_once() {
        // 0: #42 is queued
        // 0: #42 is started
        // 1: #43 is queued
        // 2: #44 is queued
        // 3: #42 is finished
        // 3: #44 is started (it is queued and has a lower execution_duration than #43)
        // 5: #44 is finished
        // 5: #43 is started
        // 8: #43 is finished

        let tasks = vec![
            Task { id: 42, queued_at: 0, execution_duration: 3 },
            Task { id: 43, queued_at: 1, execution_duration: 3 },
            Task { id: 44, queued_at: 2, execution_duration: 2 },
        ];

        assert_eq!(execution_order(tasks), vec![42, 44, 43]);
    }
}
