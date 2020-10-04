# zootree
A small tool to display Znodes in Zookeeper in tree structure.

## Usage
```bash
$ zootree --server "localhost:2181"
```

## Sample output
```
/
|----/cluster (0 bytes)
|    |----/id (45 bytes)
|----/controller_epoch (2 bytes)
|----/brokers (0 bytes)
|    |----/ids (0 bytes)
|    |----/topics (0 bytes)
|    |    |----/events (152 bytes)
|    |    |    |----/partitions (0 bytes)
|    |    |    |    |----/0 (0 bytes)
|    |    |    |    |    |----/state (84 bytes)
|    |    |    |    |----/1 (0 bytes)
|    |    |    |    |    |----/state (84 bytes)
|    |    |    |    |----/2 (0 bytes)
|    |    |    |    |    |----/state (84 bytes)
|    |    |    |    |----/3 (0 bytes)
|    |    |    |    |    |----/state (84 bytes)
|    |    |    |    |----/4 (0 bytes)
|    |    |    |    |    |----/state (84 bytes)
|    |----/seqid (0 bytes)
|----/zookeeper (0 bytes)
|    |----/config (0 bytes)
|    |----/quota (0 bytes)
|----/admin (0 bytes)
|    |----/delete_topics (0 bytes)
|----/isr_change_notification (0 bytes)
|----/consumers (0 bytes)
|----/log_dir_event_notification (0 bytes)
|----/latest_producer_id_block (67 bytes)
|----/config (0 bytes)
|    |----/changes (0 bytes)
|    |----/clients (0 bytes)
|    |----/brokers (0 bytes)
|    |----/topics (0 bytes)
|    |    |----/events (25 bytes)
|    |----/users (0 bytes)

``` 
