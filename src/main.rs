extern crate rusoto;
#[macro_use]
extern crate clap;

use std::default::Default;

use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::{S3Client, ListObjectsRequest};

fn main() {
    let provider = DefaultCredentialsProvider::new().unwrap();
    let bare_s3 = S3Client::new(provider, Region::UsEast1);

    let args = clap::App::new("Address")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Simple S3 Command Line Utility")
        .subcommand(clap::SubCommand::with_name("ls")
            .about("List buckets or objects. Listing objects begins with s3://bucket path."))
        .subcommand(clap::SubCommand::with_name("cp")
            .about("Copy objects"))
        .subcommand(clap::SubCommand::with_name("rm")
            .about("Remove objects"))
        .subcommand(clap::SubCommand::with_name("mv")
            .about("Move objects"))
        .get_matches();

    if let Some(ls) = args.subcommand_matches("ls") {
        let bucket = ""; // bucket name here
        let mut list_request = ListObjectsRequest::default();
        list_request.bucket = bucket.to_string();
        let result = bare_s3.list_objects(&list_request).unwrap();
        println!("result is {:?}", result);
    }
}
