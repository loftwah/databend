---
title: system.numbers
---

This table contains a single UInt64 column named number that contains almost all the natural numbers starting from zero.

You can use this table for tests, or if you need to do a brute force search.

Reads from this table are parallelized too.

Used for tests.

```sql
mysql> SELECT avg(number) FROM numbers(100000000);
+-------------+
| avg(number) |
+-------------+
|  49999999.5 |
+-------------+
```