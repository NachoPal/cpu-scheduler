use std::collections::hash_map::HashMap;
use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub struct Task {
    pub id: u64,
    pub queued_at: u32,
    pub execution_duration: u32,
}

fn order_queue(tasks_hash_map: &HashMap<u32, Box<Task>>, range: Range<u32>) -> (Vec<u64>, u32) {
  let mut aux_vec: Vec<Task>= Vec::new();

  for index in range {
    if tasks_hash_map.get(&index).is_some() {
      aux_vec.push(**tasks_hash_map.get(&index).unwrap());
    }
  }

  aux_vec.sort_by(|a, b| a.execution_duration.cmp(&b.execution_duration));

  let ordered_ids = aux_vec.iter().map(|t| t.id).collect();
  let total_duration = aux_vec.iter().fold(0, |acc, t| acc + t.execution_duration);

  (ordered_ids, total_duration)
}

pub fn execution_order(tasks: Vec<Task>) -> Vec<u64> {  
    let mut tasks_hash_map: HashMap<u32, Box<Task>> = HashMap::new();

    let mut max_queued_at = 0;
    let mut min_queued_at = 0;

    let mut ordered_ids: Vec<u64> = Vec::new();

    for task in tasks {
      let queued_at = task.queued_at;

      tasks_hash_map.insert(queued_at, Box::new(task));

      if queued_at > max_queued_at {
        max_queued_at = queued_at;
      }

      if queued_at < min_queued_at {
        min_queued_at = queued_at;
      }
    }

    let mut start = min_queued_at;
    let mut end = start + 1;

    while start < max_queued_at + 1 {
      let range = start..end;

      let (mut ids, total_duration) = order_queue(&tasks_hash_map, range);

      if ids.is_empty() {
        end+=1;
      } else {
        ordered_ids.append(&mut ids);

        let prev_start = start;
        start = end;
        end = prev_start + total_duration;
      }
    }

    ordered_ids
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

    #[test]
    fn my_test() {
        let tasks = vec![
            Task { id: 1, queued_at: 0, execution_duration: 5 },
            Task { id: 2, queued_at: 2, execution_duration: 3 },
            Task { id: 3, queued_at: 3, execution_duration: 1 },
            Task { id: 4, queued_at: 5, execution_duration: 2 },
            Task { id: 5, queued_at: 6, execution_duration: 4 },
            Task { id: 6, queued_at: 7, execution_duration: 3 },
            Task { id: 7, queued_at: 8, execution_duration: 2 },
            Task { id: 8, queued_at: 80, execution_duration: 2 },
            Task { id: 9, queued_at: 83, execution_duration: 1 },
        ];

        assert_eq!(
          execution_order(tasks), 
          vec![1, 3, 2, 4, 5, 7, 6, 8, 9]
        );
    }
}
