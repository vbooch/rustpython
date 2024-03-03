pub mod manager;
pub mod table;

use manager::FactsTableManager;
//use std::io;
//use std::io::prelude::*;
//use table::FactsTable;
//use std::borrow::BorrowMut;
//use std::time::Instant;
use std::sync::Mutex;
use pyo3::prelude::*;
use pyo3::types::{PyTuple};
use lazy_static::lazy_static;


lazy_static! {
    static ref MANAGER: Mutex<FactsTableManager> = Mutex::new(FactsTableManager::init());
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn create_facts_table(dim_count: u16) -> PyResult<u16> {
    let mut manager = MANAGER.lock().unwrap();
    let table = manager.create_table(dim_count);

    Ok(table.id)
}



/// Formats the sum of two numbers as string.
#[pyfunction]
fn get_value(table_id: u16, address: &PyTuple) -> PyResult<f32> {
    let addr: Vec<i32> = address.extract()?;

    let mut manager = MANAGER.lock().unwrap();
    let table_result = manager.get(&table_id);       
    
    match table_result {
        Some(table) => {
            let result = (*table).get(addr.as_slice());
            match result {
                Some(value) => Ok(*value),
                None => Ok(0.0),
            }
        },
        None => Ok(0.0),
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn set_value(table_id: u16, address: &PyTuple, value: f32) -> PyResult<()> {
    let addr: Vec<i32> = address.extract()?;
    let mut manager = MANAGER.lock().unwrap();
    let table_result = manager.get(&table_id);

    match table_result {
        Some(table) => {
            (*table).set(addr.as_slice(), value);
            Ok(())
        },
        None => Ok(()),
    }    
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(create_facts_table, m)?)?;
    m.add_function(wrap_pyfunction!(get_value, m)?)?;
    m.add_function(wrap_pyfunction!(set_value, m)?)?;
    Ok(())
}
