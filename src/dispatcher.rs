use ::Job;

use serde_json::Value;

use ::thread::Threadable;

use std::error::Error;

pub struct Dispatcher {
    jobs: Vec<Job>,
}

impl Dispatcher {
    #[allow(dead_code)]
    pub fn new(jobs: Vec<Job>) -> Self {
        Dispatcher {
            jobs: jobs
        }
    }

    pub fn dispatch(self) {
        for job in self.jobs {
            job.spawn();
        }
    }
}

use ::from_value::FromValue;

impl FromValue for Dispatcher {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        let jobs = value.get("jobs")
                        .unwrap_or(&Value::Null)
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .into_iter()
                        .map(| job | {
                            Job::new_from_value(job.to_owned()).unwrap()
                        }).collect();

        Ok(Dispatcher {
            jobs: jobs,
        })
    }
}

#[cfg(test)]
mod tests {
    use ::from_value::FromValue;
    use ::dispatcher::Dispatcher;

    #[test]
    fn test_from_value() {
        let dispatcher = json!({});

        Dispatcher::new_from_value(dispatcher).unwrap();
    }
}