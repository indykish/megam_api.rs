use megam_api::api::Api;
use megam_api::util::csars::Csar;
use rustc_serialize::json;
use megam_api::api::Options;

//#[test]
fn create() {
    // create hashmap for api settings
    let options = Options {
    Email: "b@a.com".to_string(),
    Apikey: "GGUdgT-sjLXn9EYRkS8Q7g==".to_string(),
    Host: "http://localhost:9000".to_string(),
    Version: "/v2".to_string(),
    };

     println!("{:?}", json::encode(&options).unwrap());

    let mut a = Csar::new();		

//example yaml string
    a.desc = "
tosca_definitions_version: tosca_simple_yaml_1_0

description: Template for deploying a two-tier application servers on two

inputs:
  # Admin user name and password to use with the WordPress application
  wp_admin_username:
     type: string
  wp_admin_password:
     type string
  wp_db_name:
    type: string
  wp_db_user:
    type: string
  wp_db_password:
    type: string
  wp_db_port:
    type: integer
  postgres_root_password:
     type string
  postgres_port:
     type integer

node_templates:
  wordpress:
    type: tosca.nodes.WebApplication.WordPress
    properties:
      admin_user: { get_input: wp_admin_username }
      admin_password: { get_input: wp_admin_password }
      db_host: { get_property: [ db_server, ip_address ] }
    requirements:
      - host: apache
      - database_endpoint: wordpress_db
    interaces:
      Lifecycle:
        inputs:
          db_host: { get_property: [ db_server, ip_address ] }
          db_port: { get_property: [ wordpress_db, db_port ] }
          db_name: { get_property: [ wordpress_db, db_name ] }
          db_user: { get_property: [ wordpress_db, db_user ] }
          db_password: { get_property: [ wordpress_db, db_password ] }

  apache:
    type: tosca.nodes.WebServer.Apache
    properties:
      # omitted here for sake of brevity
    requirements:
      - host: web_server

  web_server:
    type: tosca.nodes.Compute   

  wordpress_db:
    type: tosca.nodes.Database.PostgreSQL
    properties:
      db_name: { get_input: wp_db_name }
      db_user: { get_input: wp_db_user }
      db_password: { get_input: wp_db_password }
      db_port: { get_input: wp_db_port }
    requirements:
      - host: postgres

  postgres:
    type: tosca.nodes.DBMS.PostgresSQL
    properties:
      dbms_root_password: { get_input: postgres_root_password }
      dbms_port: { get_input: postgres_port }
    requirements:
      - host: db_server

  db_server:
    type: tosca.nodes.Compute
".to_string();   

     match a.create(json::encode(&options).unwrap()) {
        Ok(n) => println!("result: Is OK: {:?}", n),
        Err(FailOne) => println!("result: Failed One: {:?}", FailOne),
    }
}

//#[test]
fn list() {
		// create hashmap for api settings
    let options = Options {
    Email: "b@a.com".to_string(),
    Apikey: "GGUdgT-sjLXn9EYRkS8Q7g==".to_string(),
    Host: "http://localhost:9000".to_string(),
    Version: "/v2".to_string(),
    };

     println!("{:?}", json::encode(&options).unwrap());

    let mut a = Csar::new();		

   	 match a.list(json::encode(&options).unwrap()) {
        Ok(n) => println!("result: Is OK: {:?}", n),
        Err(FailOne) => println!("result: Failed One: {:?}", FailOne),
    }
}

#[test]
fn push() {
		// create hashmap for api settings
    let options = Options {
    Email: "b@a.com".to_string(),
    Apikey: "GGUdgT-sjLXn9EYRkS8Q7g==".to_string(),
    Host: "http://localhost:9000".to_string(),
    Version: "/v2".to_string(),
    };

     println!("{:?}", json::encode(&options).unwrap());

    let mut a = Csar::new();		

   	 match a.push(json::encode(&options).unwrap(), "CSR1217830905918390272".to_string()) {
        Ok(n) => println!("result: Is OK: {:?}", n),
        Err(FailOne) => println!("result: Failed One: {:?}", FailOne),
    }
}

