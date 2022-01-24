use chrono::Utc;
use hdf5::{File, Result};
use std::thread::sleep;
use std::time;
use tonic::transport::Endpoint;

use vald_sample_rust_client::payload::v1::insert;
use vald_sample_rust_client::payload::v1::object;
use vald_sample_rust_client::payload::v1::remove;
use vald_sample_rust_client::payload::v1::search;
use vald_sample_rust_client::vald::v1::insert_client;
use vald_sample_rust_client::vald::v1::remove_client;
use vald_sample_rust_client::vald::v1::search_client;

// Dataset file name
static FILE: &str = "fashion-mnist-784-euclidean.hdf5";
// Dataset name
static DATASET: &str = "train";
// set Vald cluster host
static HOST: &str = "http://localhost:8080";
// Time duration for waiting to finish `CreateIndex` and `SaveIndex`
static DURATION: u64 = 60;

fn read_file() -> Result<Vec<Vec<f32>>> {
    let file = File::open(FILE).unwrap_or_else(|e| panic!("[ERR] failed to read file: {}", e));
    let data = file
        .dataset(DATASET)
        .unwrap_or_else(|e| panic!("[ERR] failed to get dataset: {}", e));
    let mut vector = Vec::new();
    for train in data.read_2d::<f32>()?.outer_iter() {
        let mut vec: Vec<f32> = Vec::new();
        vec.append(&mut train.to_vec());
        vector.push(vec);
        if vector.len() == 500 {
            break;
        }
    }
    Ok(vector)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("[Start] Load {} file\n", FILE);
    let vec = read_file()?;
    print!("[End] Success to load {} file\n", FILE);

    print!("[Start] Insert phase\n");
    // create insert client
    let mut insert_client =
        insert_client::InsertClient::connect(Endpoint::from_static(HOST)).await?;
    let mut ids: Vec<String> = Vec::new();
    for v in vec.iter() {
        let id = Utc::now().timestamp_nanos().to_string();
        ids.push(id.to_string());
        // insert vector
        let _ = insert_client
            .insert(insert::Request {
                vector: Some(object::Vector {
                    id: id.to_string(),
                    vector: v.to_vec(),
                }),
                config: Some(insert::Config {
                    skip_strict_exist_check: true,
                    filters: None,
                    timestamp: Utc::now().timestamp(),
                }),
            })
            .await?;
    }
    print!("[End] Finish Insert Phase\n");

    print!("[Sleep] Waiting SaveIndex is completed...\n");
    sleep(time::Duration::from_secs(DURATION));

    print!("[Start] Search phase\n");
    // create search client
    let mut search_client =
        search_client::SearchClient::connect(Endpoint::from_static(HOST)).await?;
    for id in ids.clone() {
        // search nearest neighbor vectors using searchById method
        let res = search_client
            .search_by_id(search::IdRequest {
                id: id.to_string(),
                config: Some(search::Config {
                    request_id: id.to_string(),
                    num: 10,
                    radius: -1.0,
                    epsilon: -1.0,
                    timeout: 500,
                    ingress_filters: None,
                    egress_filters: None,
                }),
            })
            .await?;
        print!("[Id]: {:?}\n", id.to_string());
        print!("[Result]: {:#?}\n", res.into_inner().results);
    }
    print!("[End] Finish Search Phase\n");

    print!("[Start] Remove phase\n");
    // create remove client
    let mut remove_client =
        remove_client::RemoveClient::connect(Endpoint::from_static(HOST)).await?;
    for id in ids.clone() {
        // remove vectors
        let _ = remove_client
            .remove(remove::Request {
                id: Some(object::Id { id: id.to_string() }),
                config: None,
            })
            .await?;
    }
    print!("[End] Finish Remove phase\n");

    Ok(())
}
