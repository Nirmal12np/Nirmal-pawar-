#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct WorkRecord {
    pub record_id: u64,
    pub employee_name: String,
    pub organization: String,
    pub role: String,
    pub start_date: u64,
    pub end_date: u64,
    pub verified: bool,
}

#[contracttype]
pub enum WorkLog {
    Record(u64),
}

const RECORD_COUNT: Symbol = symbol_short!("R_COUNT");

#[contract]
pub struct WorkHistoryTracker;

#[contractimpl]
impl WorkHistoryTracker {
    pub fn add_work_record(
        env: Env,
        employee_name: String,
        organization: String,
        role: String,
        start_date: u64,
        end_date: u64,
    ) -> u64 {
        let mut count = env.storage().instance().get(&RECORD_COUNT).unwrap_or(0);
        count += 1;

        let record = WorkRecord {
            record_id: count,
            employee_name,
            organization,
            role,
            start_date,
            end_date,
            verified: false,
        };

        env.storage().instance().set(&WorkLog::Record(count), &record);
        env.storage().instance().set(&RECORD_COUNT, &count);
        env.storage().instance().extend_ttl(5000, 5000);

        count
    }

    pub fn verify_work_record(env: Env, record_id: u64) {
        let mut record: WorkRecord = env
            .storage()
            .instance()
            .get(&WorkLog::Record(record_id))
            .unwrap_or_else(|| panic!("Record not found"));

        record.verified = true;

        env.storage().instance().set(&WorkLog::Record(record_id), &record);
        env.storage().instance().extend_ttl(5000, 5000);
    }

    pub fn view_work_record(env: Env, record_id: u64) -> WorkRecord {
        env.storage()
            .instance()
            .get(&WorkLog::Record(record_id))
            .unwrap_or(WorkRecord {
                record_id: 0,
                employee_name: String::from_str(&env, "Not_Found"),
                organization: String::from_str(&env, "Not_Found"),
                role: String::from_str(&env, "Not_Found"),
                start_date: 0,
                end_date: 0,
                verified: false,
            })
    }

    pub fn total_records(env: Env) -> u64 {
        env.storage().instance().get(&RECORD_COUNT).unwrap_or(0)
    }
}
