use super::types::{Item, Lesson};

pub fn get_content() -> Lesson {
    vec![
        Item {
            characters: "What is idemopotency?",
            english: "An idempotent operation can be repeated over and over without changing the result. There are no side effects.",
            phonetic: "",
        },
        Item {
            characters: "What is atomicity?",
            english: "Atomicity refers to a series of operations (or one) such that all occur or none. This is important for database transactions.",
            phonetic: "",
        },
        Item {
            characters: "What is reliability?",
            english: "Reliability is if a system can remain operational if services/components fail.",
            phonetic: "",
        },
        Item {
            characters: "What is the difference between availability and reliability?",
            english: "An highly available system is not necessarily reliable. Reliability, however, implies availability.",
            phonetic: "",
        },
        Item {
            characters: "What is serviceability/manageability for a system?",
            english: "How easily system problems can be diagnosed/repaired when they arise/how easily services can be rolled back if necessary.",
            phonetic: "",
        },
        Item {
            characters: "What is load balancing?",
            english: "A load balancer distributes request traffic evenly to a series of services to spread the load and allow the system to provide high availability at a large request volume.",
            phonetic: "",
        },
        Item {
            characters: "Between which system layers can a load balancer be added?",
            english: "Between clients and web servers, between web servers and caches or application servers, between application servers and databases",
            phonetic: "",
        },
        Item {
            characters: "What are some load balancing algorithms to distribute requests?",
            english: "Least connection method (fewest active connections), least response time method, round robin, and so on. All utilize health checks and eliminate failed servers.",
            phonetic: "",
        },
        Item {
            characters: "What if a load balancer fails?",
            english: "A load balancer is a single point of failure. Typically two can operate in a cluster and monitor each others health. If the active balancer fails, the passive balancer can take over and the failed server can be replaced.",
            phonetic: "",
        },
        Item {
            characters: "What are some approaches for cache invalidation for a distributed system?",
            english: "Write-through cache (data is written to the cache and storage), write-around cache (data is written to storage only - a new request will miss the cache), write-back cache (data is written to the cache only, and later persisted to storage).",
            phonetic: "",
        },
        Item {
            characters: "What are some cache eviction strategies?",
            english: "Least frequently used, least recently used, last in first out, first in first out, etc.",
            phonetic: "",
        },
        Item {
            characters: "What is data sharding/partitioning?",
            english: "Data sharding is scaling a database layer of an application by distributing a single database across multiple partitions or shards. It's essentially scaling a database horizontally, which is usually required at some data scale.",
            phonetic: "",
        },
        Item {
            characters: "What is range based sharding, and one is one shortcoming of this approach?",
            english: "Distributing data based on where some part of it's input falls in a range, e.g. zip codes within a numeric range or user email based on letter range. A shortcoming is this can lead to unbalanced load across the database partitions.",
            phonetic: "",
        },
        Item {
            characters: "What is key/hash based partitioning?",
            english: "This approaches takes a key of some input value (e.g. user id), e.g. by simply modding it by the number of partitions, to determine the partition that data will belong to.",
            phonetic: "",
        },
        Item {
            characters: "What is one problem with hash/key based data partitioning?",
            english: "It's fixed to the number of database services at one point in time, and cannot easily accommodate additional partitions in the future.",
            phonetic: "",
        },
        Item {
            characters: "What is one problem with database sharding in terms of querying data?",
            english: "Joins between multiple data sources or tables can become very expensive and no longer possible. One solution to this is data denormalization, which allows queries which would have required joins to run on a single table. Materialized views is a similar approach, where specific query tables of data are maintained. However, these approaches increase the number of writes required and introduce problems of data consistency.",
            phonetic: "",
        },
        Item {
            characters: "What is a data integrity issue encountered with database sharding?",
            english: "You typically cannot support foreign key constraints across different database shards. This means referential integrity must be maintained in application code, or in routine SQL jobs which clean up dangling references.",
            phonetic: "",
        },
        Item {
            characters: "What is one problem with database sharding?",
            english: "Shard rebalancing may be required, if data is distributed unevenly or if load on a particular shard is too high. This means adding additional shards and changing the partitioning scheme, or rebalancing existing data, or both - all of which is very hard to do without downtime.",
            phonetic: "",
        },
        Item {
            characters: "What is database indexing? What are the tradeoffs?",
            english: "Database indexing means certain keys for input data are indexed so the same data can be immediately looked up by the key. This is why we indexed IOC by type|value at TruSTAR, for query performance. Indexing increase storage space and write time, however, so it's important not to add indexes unnecessarily. In addition, it's important to choose indexes carefully based on how the data will be queried.",
            phonetic: "",
        },
        Item {
            characters: "What is a proxy server?",
            english: "An intermediary that stands between clients and backing services. It can be used to filter requests, add logging, add headers, cache responses, handle API rate limiting, etc.",
            phonetic: "",
        },
        Item {
            characters: "What is a reverse proxy?",
            english: "A proxy server which stands between a client and returns data on the behalf of one or more servers. To the client, the data appears to come from the proxy server.",
            phonetic: "",
        },
        Item {
            characters: "What is redundancy?",
            english: "Building a system of backups so there is no single point of failure for any system component. For instance, redundancy is built in to main RDMS which use a master slave relationship to replicate write/update operations in multiple databases.",
            phonetic: "",
        },
        Item {
            characters: "Databases which hold unstructured data with dynamic schema in a distributed manner are:",
            english: "noSQL (non-relational) databases. These include key-value stores, document stores, wide-column databases, and graph databases. Examples include MongoDB, DynamoDB, Cassandra, Redis, Neo4j, etc.",
            phonetic: "",
        },
        Item {
            characters: "Databases which are structured and have predefined schema are?",
            english: "SQL (relational) databases. Examples include MySQL, Oracle, PostgresQL, SQLite, etc.",
            phonetic: "",
        },
        Item {
            characters: "What is an ACID transaction?",
            english: "Atomicity, Consistency, Reliability, Durability - a transaction either happens or doesn't as a single isolated unit.",
            phonetic: "",
        },
        Item {
            characters: "Which database scales more easily, SQL or NoSQL?",
            english: "Distributed data partitioning/sharding tends to be built in to many NoSQL non-relational databases like MongoDB or Cassandra.",
            phonetic: "",
        },
        Item {
            characters: "Why is it hard to scale SQL databases horizontally?",
            english: "SQL relies on representing datasets with multiple tables and relationship (keys) between tables. Running queries on sharded SQL tables is very expensive and slow, at best query results would have to be ordered/filtered/search in the application code.",
            phonetic: "",
        },
        Item {
            characters: "Which database system is better for performance/scalability? Which is better for reliability/consistency?",
            english: "NoSQL tends to be better for scale and performance and SQL tends to be better for reliability/consistency (typically SQL database are ACID compliant).",
            phonetic: "",
        },
        Item {
            characters: "What are some benefits of non-relational databases?",
            english: "You need performance or scale and can sacrifice consistency or reliability. You have unstructured data, data with poorly-defined schema, or data which is not well represented by tables.",
            phonetic: "",
        },
        Item {
            characters: "What is the CAP Theorem?",
            english: "The CAP Theorem states that no distributed system can be Consistent, Available, and Partition Tolerant. It can only be 2 of these 3.",
            phonetic: "",
        },
        Item {
            characters: "What is consistent hashing?",
            english: "Consistent hashing is a solution for distributed hash tables and data partitioning which is more balanced and scales better as the partition size grows",
            phonetic: "",
        },
        Item {
            characters: "What is long polling?",
            english: "A client-server request protocol where clients request data and if the server doesn't have data available or updated, it leaves the request open until it has new data. Then it responds, after which the client immediately requests more data.",
            phonetic: "",
        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
    ]
}
